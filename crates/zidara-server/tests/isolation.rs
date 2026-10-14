//! Row-level security, proven against real PostgreSQL. The app role is not the owner.
mod support;

use support::*;

const TABLES: [&str; 6] = [
    "tenants",
    "cohorts",
    "users",
    "memberships",
    "events",
    "applied_envelopes",
];

#[tokio::test]
async fn without_context_nothing_is_visible() {
    let db = db().await;
    owner_event(&db, db.seed.a, None).await;
    for table in TABLES {
        let n: i64 =
            sqlx::query_scalar(sqlx::AssertSqlSafe(format!("select count(*) from {table}")))
                .fetch_one(&db.app)
                .await
                .unwrap();
        assert_eq!(n, 0, "{table}");
    }
}

#[tokio::test]
async fn a_reused_connection_without_context_sees_nothing() {
    let db = db().await;
    owner_event(&db, db.seed.a, None).await;
    let pool = db.app_pool(1).await;
    let mut tx = as_user(&pool, db.seed.amina).await;
    assert_eq!(count(&mut tx, "events").await, 1);
    tx.commit().await.unwrap();
    // Same physical connection; the setting now exists but is empty.
    let n: i64 = sqlx::query_scalar("select count(*) from events")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(n, 0);
}

#[tokio::test]
async fn members_see_their_cohort_only() {
    let db = db().await;
    let mut amina = as_user(&db.app, db.seed.amina).await;
    let cohorts: Vec<uuid::Uuid> = sqlx::query_scalar("select id from cohorts")
        .fetch_all(&mut *amina)
        .await
        .unwrap();
    assert_eq!(cohorts, vec![db.seed.a]);
    assert_eq!(count(&mut amina, "tenants").await, 1);
    let mut sara = as_user(&db.app, db.seed.sara).await;
    let cohorts: Vec<uuid::Uuid> = sqlx::query_scalar("select id from cohorts")
        .fetch_all(&mut *sara)
        .await
        .unwrap();
    assert_eq!(cohorts, vec![db.seed.b]);
}

#[tokio::test]
async fn learners_cannot_list_classmates() {
    let db = db().await;
    let mut amina = as_user(&db.app, db.seed.amina).await;
    assert_eq!(count(&mut amina, "memberships").await, 1);
    let users: Vec<uuid::Uuid> = sqlx::query_scalar("select id from users")
        .fetch_all(&mut *amina)
        .await
        .unwrap();
    assert_eq!(users, vec![db.seed.amina]);
    let mut karim = as_user(&db.app, db.seed.karim).await;
    let mut seen: Vec<uuid::Uuid> = sqlx::query_scalar("select user_id from memberships")
        .fetch_all(&mut *karim)
        .await
        .unwrap();
    seen.sort();
    let mut cohort_a = vec![db.seed.amina, db.seed.yacine, db.seed.karim, db.seed.agent];
    cohort_a.sort();
    assert_eq!(seen, cohort_a);
    assert_eq!(count(&mut karim, "users").await, 4);
}

#[tokio::test]
async fn learners_see_their_own_and_cohort_wide_events() {
    let db = db().await;
    owner_event(&db, db.seed.a, Some(db.seed.amina)).await;
    owner_event(&db, db.seed.a, Some(db.seed.yacine)).await;
    owner_event(&db, db.seed.a, None).await;
    owner_event(&db, db.seed.b, Some(db.seed.sara)).await;
    for (user, expected) in [
        (db.seed.amina, 2),
        (db.seed.yacine, 2),
        (db.seed.karim, 3),
        (db.seed.sara, 1),
        (db.seed.nadia, 1),
    ] {
        let mut tx = as_user(&db.app, user).await;
        assert_eq!(count(&mut tx, "events").await, expected, "{user}");
    }
}

#[tokio::test]
async fn the_agent_sees_every_cohort_it_serves() {
    let db = db().await;
    owner_event(&db, db.seed.a, Some(db.seed.amina)).await;
    owner_event(&db, db.seed.b, Some(db.seed.sara)).await;
    let mut agent = as_user(&db.app, db.seed.agent).await;
    assert_eq!(count(&mut agent, "events").await, 2);
}

#[tokio::test]
async fn writes_outside_your_cohort_are_refused() {
    let db = db().await;
    assert!(
        try_insert_event(&db, db.seed.amina, db.seed.b, None)
            .await
            .is_err()
    );
    assert!(
        try_insert_event(&db, db.seed.amina, db.seed.a, Some(db.seed.yacine))
            .await
            .is_err()
    );
    assert!(
        try_insert_event(&db, db.seed.amina, db.seed.a, Some(db.seed.amina))
            .await
            .is_ok()
    );
    assert!(
        try_insert_event(&db, db.seed.karim, db.seed.a, Some(db.seed.yacine))
            .await
            .is_ok()
    );
    assert!(
        try_insert_event(&db, db.seed.karim, db.seed.b, None)
            .await
            .is_err()
    );
}

#[tokio::test]
async fn the_log_is_append_only() {
    let db = db().await;
    owner_event(&db, db.seed.a, Some(db.seed.amina)).await;
    let mut karim = as_user(&db.app, db.seed.karim).await;
    assert!(
        sqlx::query("update events set at_ms = 1")
            .execute(&mut *karim)
            .await
            .is_err()
    );
    let mut karim = as_user(&db.app, db.seed.karim).await;
    assert!(
        sqlx::query("delete from events")
            .execute(&mut *karim)
            .await
            .is_err()
    );
}

#[tokio::test]
async fn envelope_ids_belong_to_their_sender() {
    let db = db().await;
    for user in [db.seed.amina, db.seed.yacine] {
        let mut tx = as_user(&db.app, user).await;
        sqlx::query("insert into applied_envelopes (id, tenant_id, cohort_id, user_id) values ($1, $2, $3, $4)")
            .bind(uuid::Uuid::now_v7())
            .bind(db.seed.tenant)
            .bind(db.seed.a)
            .bind(user)
            .execute(&mut *tx)
            .await
            .unwrap();
        tx.commit().await.unwrap();
    }
    let mut amina = as_user(&db.app, db.seed.amina).await;
    assert_eq!(count(&mut amina, "applied_envelopes").await, 1);
    // Recording an envelope in someone else's name is refused.
    let err = sqlx::query(
        "insert into applied_envelopes (id, tenant_id, cohort_id, user_id) values ($1, $2, $3, $4)",
    )
    .bind(uuid::Uuid::now_v7())
    .bind(db.seed.tenant)
    .bind(db.seed.a)
    .bind(db.seed.yacine)
    .execute(&mut *amina)
    .await;
    assert!(err.is_err());
}

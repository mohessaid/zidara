//! The one write path: lock, ledger, history, decide, insert, commit.
mod support;

use support::*;
use uuid::Uuid;

use zidara_kernel::decide::{Command, Envelope, KernelError};
use zidara_kernel::event::EventBody;
use zidara_kernel::ids::{FrameworkId, Timestamp};
use zidara_kernel::roles::{Permission, Role};
use zidara_server::commands::{ApiError, Outcome, apply};
use zidara_server::school::{FixedClock, School};
use zidara_server::store;

const NOW: i64 = 1_791_158_400_000 + 3_600_000; // 5 October 2026, 01:00 UTC
const DAY: i64 = 86_400_000;

fn envelope(at: i64, command: Command) -> Envelope {
    Envelope {
        id: Uuid::now_v7(),
        at: Timestamp(at),
        command,
    }
}

async fn history(db: &TestDb, user: Uuid) -> Vec<zidara_kernel::event::Event> {
    let mut tx = store::begin_as(&db.app, user).await.unwrap();
    store::history(&mut tx, db.seed.a).await.unwrap()
}

#[tokio::test]
async fn a_learners_quiz_is_marked_and_stored() {
    let db = db().await;
    let (school, clock) = (School::demo(), FixedClock::at(NOW));
    let outcome = apply(
        &db.app,
        &school,
        &clock,
        db.seed.amina,
        db.seed.a,
        envelope(NOW, quiz_f2(0, 3)),
    )
    .await
    .unwrap();
    let Outcome::Applied { events } = outcome else {
        panic!("{outcome:?}")
    };
    assert_eq!(events.len(), 2);
    let stored = history(&db, db.seed.amina).await;
    assert_eq!(stored, events);
    let learners: Vec<Option<Uuid>> = sqlx::query_scalar("select learner_id from events")
        .fetch_all(&db.owner)
        .await
        .unwrap();
    assert_eq!(learners, vec![Some(db.seed.amina); 2]);
}

#[tokio::test]
async fn replayed_envelope_applies_once() {
    let db = db().await;
    let (school, clock) = (School::demo(), FixedClock::at(NOW));
    let sent = envelope(NOW, quiz_f2(0, 3));
    assert!(matches!(
        apply(
            &db.app,
            &school,
            &clock,
            db.seed.amina,
            db.seed.a,
            sent.clone()
        )
        .await
        .unwrap(),
        Outcome::Applied { .. }
    ));
    assert_eq!(
        apply(&db.app, &school, &clock, db.seed.amina, db.seed.a, sent)
            .await
            .unwrap(),
        Outcome::Duplicate
    );
    let quizzes = history(&db, db.seed.amina)
        .await
        .into_iter()
        .filter(|e| matches!(e.body, EventBody::QuizSubmitted { .. }))
        .count();
    assert_eq!(quizzes, 1);
}

#[tokio::test]
async fn refused_commands_leave_no_trace() {
    let db = db().await;
    let (school, clock) = (School::demo(), FixedClock::at(NOW));
    let sent = envelope(
        NOW,
        Command::ChangeFramework {
            framework: FrameworkId::new("cambridge-style"),
        },
    );
    let refused = Outcome::Refused {
        error: KernelError::Forbidden {
            role: Role::Learner,
            permission: Permission::ChangeFramework,
        },
    };
    assert_eq!(
        apply(
            &db.app,
            &school,
            &clock,
            db.seed.amina,
            db.seed.a,
            sent.clone()
        )
        .await
        .unwrap(),
        refused
    );
    assert_eq!(
        apply(&db.app, &school, &clock, db.seed.amina, db.seed.a, sent)
            .await
            .unwrap(),
        refused
    );
    let (events, envelopes): (i64, i64) = sqlx::query_as(
        "select (select count(*) from events), (select count(*) from applied_envelopes)",
    )
    .fetch_one(&db.owner)
    .await
    .unwrap();
    assert_eq!((events, envelopes), (0, 0));
}

#[tokio::test]
async fn future_dated_envelopes_are_capped() {
    let db = db().await;
    let (school, clock) = (School::demo(), FixedClock::at(NOW));
    apply(
        &db.app,
        &school,
        &clock,
        db.seed.amina,
        db.seed.a,
        envelope(NOW + DAY, quiz_f2(0, 3)),
    )
    .await
    .unwrap();
    assert!(
        history(&db, db.seed.amina)
            .await
            .iter()
            .all(|e| e.at == Timestamp(NOW))
    );
}

#[tokio::test]
async fn backdated_envelopes_are_floored() {
    let db = db().await;
    let (school, clock) = (School::demo(), FixedClock::at(NOW));
    apply(
        &db.app,
        &school,
        &clock,
        db.seed.amina,
        db.seed.a,
        envelope(NOW, quiz_f2(0, 3)),
    )
    .await
    .unwrap();
    clock.set(NOW + DAY);
    apply(
        &db.app,
        &school,
        &clock,
        db.seed.amina,
        db.seed.a,
        envelope(NOW - DAY, quiz_f2(1, 0)),
    )
    .await
    .unwrap();
    // The weak retry also brings remediation; every event of it is floored to NOW.
    let times: Vec<i64> = history(&db, db.seed.amina)
        .await
        .iter()
        .map(|e| e.at.0)
        .collect();
    assert!(
        times.len() > 2 && times.iter().all(|t| *t == NOW),
        "{times:?}"
    );
    // Offline work keeps its own time when it is not older than what the server has.
    apply(
        &db.app,
        &school,
        &clock,
        db.seed.amina,
        db.seed.a,
        envelope(NOW + 5, quiz_f2(2, 0)),
    )
    .await
    .unwrap();
    assert_eq!(
        history(&db, db.seed.amina).await.last().unwrap().at,
        Timestamp(NOW + 5)
    );
}

#[tokio::test]
async fn concurrent_weak_attempts_assign_remediation_once() {
    let db = db().await;
    let (school, clock) = (School::demo(), FixedClock::at(NOW));
    // F2 opens once F1 is mastered, but remediation does not depend on that; one weak try first.
    apply(
        &db.app,
        &school,
        &clock,
        db.seed.amina,
        db.seed.a,
        envelope(NOW, quiz_f2(0, 0)),
    )
    .await
    .unwrap();
    // Eight at once: without the cohort lock, several would see "not yet assigned".
    let attempts = (0..8).map(|_| {
        apply(
            &db.app,
            &school,
            &clock,
            db.seed.amina,
            db.seed.a,
            envelope(NOW, quiz_f2(1, 0)),
        )
    });
    for result in futures::future::join_all(attempts).await {
        result.unwrap();
    }
    let assigned = history(&db, db.seed.amina)
        .await
        .into_iter()
        .filter(|e| matches!(e.body, EventBody::ActivityAssigned { .. }))
        .count();
    assert_eq!(assigned, 1);
}

#[tokio::test]
async fn non_members_are_refused() {
    let db = db().await;
    let (school, clock) = (School::demo(), FixedClock::at(NOW));
    let err = apply(
        &db.app,
        &school,
        &clock,
        db.seed.sara,
        db.seed.a,
        envelope(NOW, quiz_f2(0, 3)),
    )
    .await
    .unwrap_err();
    assert!(matches!(err, ApiError::NotMember), "{err:?}");
}

#[tokio::test]
async fn the_teacher_sees_what_the_learner_did() {
    let db = db().await;
    let (school, clock) = (School::demo(), FixedClock::at(NOW));
    apply(
        &db.app,
        &school,
        &clock,
        db.seed.amina,
        db.seed.a,
        envelope(NOW, quiz_f2(0, 3)),
    )
    .await
    .unwrap();
    apply(
        &db.app,
        &school,
        &clock,
        db.seed.yacine,
        db.seed.a,
        envelope(NOW, quiz_f2(0, 0)),
    )
    .await
    .unwrap();
    assert_eq!(history(&db, db.seed.amina).await.len(), 2);
    let karim = history(&db, db.seed.karim).await;
    assert!(karim.len() >= 4, "{}", karim.len());
}

#[tokio::test]
async fn teachers_switch_and_learners_see_it() {
    let db = db().await;
    let (school, clock) = (School::demo(), FixedClock::at(NOW));
    let switch = envelope(
        NOW,
        Command::ChangeFramework {
            framework: FrameworkId::new("cambridge-style"),
        },
    );
    assert!(matches!(
        apply(&db.app, &school, &clock, db.seed.karim, db.seed.a, switch)
            .await
            .unwrap(),
        Outcome::Applied { .. }
    ));
    let seen = history(&db, db.seed.amina).await;
    assert!(matches!(seen[0].body, EventBody::FrameworkChanged { .. }));
}

/// Deterministic proof of the cohort lock: while another transaction holds it, a command waits.
#[tokio::test]
async fn commands_wait_for_the_cohort_lock() {
    let db = db().await;
    let (school, clock) = (School::demo(), FixedClock::at(NOW));
    let mut holder = store::begin_as(&db.app, db.seed.karim).await.unwrap();
    store::lock_cohort(&mut holder, db.seed.a).await.unwrap();
    let mut pending = Box::pin(apply(
        &db.app,
        &school,
        &clock,
        db.seed.amina,
        db.seed.a,
        envelope(NOW, quiz_f2(0, 3)),
    ));
    let waited = tokio::time::timeout(std::time::Duration::from_millis(500), &mut pending).await;
    assert!(
        waited.is_err(),
        "the command ran while the cohort was locked"
    );
    holder.commit().await.unwrap();
    assert!(matches!(pending.await.unwrap(), Outcome::Applied { .. }));
}

#[tokio::test]
async fn a_cohort_with_an_unknown_lesson_is_an_error() {
    let db = db().await;
    sqlx::query("update cohorts set lesson_id = 'nope' where id = $1")
        .bind(db.seed.a)
        .execute(&db.owner)
        .await
        .unwrap();
    let (school, clock) = (School::demo(), FixedClock::at(NOW));
    let err = apply(
        &db.app,
        &school,
        &clock,
        db.seed.amina,
        db.seed.a,
        envelope(NOW, quiz_f2(0, 3)),
    )
    .await
    .unwrap_err();
    assert!(
        matches!(err, ApiError::UnknownLesson(ref l) if l == "nope"),
        "{err:?}"
    );
}

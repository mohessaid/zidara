//! One PostgreSQL 16 container per test binary, one fresh database per test.
//! Containers carry the label `org.zidara.test=1`; `make test` removes them afterwards,
//! because a container held in a static is never dropped.
#![allow(dead_code)]

use sqlx::{Executor, PgPool, Postgres, Transaction, postgres::PgPoolOptions};
use testcontainers_modules::postgres::Postgres as PostgresImage;
use testcontainers_modules::testcontainers::{ContainerAsync, ImageExt, runners::AsyncRunner};
use tokio::sync::OnceCell;
use uuid::Uuid;
use zidara_server::seed::Seed;

struct Cluster {
    _node: ContainerAsync<PostgresImage>,
    port: u16,
}

static CLUSTER: OnceCell<Cluster> = OnceCell::const_new();

fn url(user: &str, password: &str, port: u16, database: &str) -> String {
    format!("postgres://{user}:{password}@127.0.0.1:{port}/{database}")
}

async fn cluster() -> &'static Cluster {
    CLUSTER
        .get_or_init(|| async {
            let node = PostgresImage::default()
                .with_tag("16")
                .with_label("org.zidara.test", "1")
                .start()
                .await
                .expect("Docker can run postgres:16");
            let port = node.get_host_port_ipv4(5432).await.unwrap();
            let admin = PgPool::connect(&url("postgres", "postgres", port, "postgres"))
                .await
                .unwrap();
            // The migration creates the role when missing; tests create it first, with a password.
            admin
                .execute("create role zidara_app login nobypassrls password 'app'")
                .await
                .unwrap();
            admin.close().await;
            Cluster { _node: node, port }
        })
        .await
}

pub struct TestDb {
    pub owner: PgPool,
    pub app: PgPool,
    pub seed: Seed,
    pub port: u16,
    pub name: String,
}

impl TestDb {
    /// A separate app pool, e.g. with one connection to test reuse.
    pub async fn app_pool(&self, max: u32) -> PgPool {
        PgPoolOptions::new()
            .max_connections(max)
            .connect(&url("zidara_app", "app", self.port, &self.name))
            .await
            .unwrap()
    }
}

/// A fresh, migrated and seeded database.
pub async fn db() -> TestDb {
    let cluster = cluster().await;
    let name = format!("t{}", Uuid::now_v7().simple());
    let admin = PgPool::connect(&url("postgres", "postgres", cluster.port, "postgres"))
        .await
        .unwrap();
    admin
        .execute(sqlx::AssertSqlSafe(format!("create database {name}")))
        .await
        .unwrap();
    admin.close().await;
    let owner = PgPool::connect(&url("postgres", "postgres", cluster.port, &name))
        .await
        .unwrap();
    zidara_server::migrate(&owner).await.unwrap();
    let seed = zidara_server::seed::seed(&owner).await.unwrap();
    let app = PgPool::connect(&url("zidara_app", "app", cluster.port, &name))
        .await
        .unwrap();
    TestDb {
        owner,
        app,
        seed,
        port: cluster.port,
        name,
    }
}

/// A transaction acting as `user`.
pub async fn as_user(pool: &PgPool, user: Uuid) -> Transaction<'static, Postgres> {
    zidara_server::store::begin_as(pool, user).await.unwrap()
}

pub async fn count(tx: &mut Transaction<'static, Postgres>, table: &str) -> i64 {
    sqlx::query_scalar(sqlx::AssertSqlSafe(format!("select count(*) from {table}")))
        .fetch_one(&mut **tx)
        .await
        .unwrap()
}

/// Inserts an event as the owner (bypassing RLS) for visibility tests.
pub async fn owner_event(db: &TestDb, cohort: Uuid, learner: Option<Uuid>) -> Uuid {
    let id = Uuid::now_v7();
    sqlx::query(
        "insert into events (id, tenant_id, cohort_id, learner_id, at_ms, actor, body)
         values ($1, $2, $3, $4, 0, '{\"kind\":\"system\"}', '{}')",
    )
    .bind(id)
    .bind(db.seed.tenant)
    .bind(cohort)
    .bind(learner)
    .execute(&db.owner)
    .await
    .unwrap();
    id
}

/// Inserts an event as `user`, through RLS.
pub async fn try_insert_event(
    db: &TestDb,
    user: Uuid,
    cohort: Uuid,
    learner: Option<Uuid>,
) -> Result<(), sqlx::Error> {
    let mut tx = as_user(&db.app, user).await;
    sqlx::query(
        "insert into events (id, tenant_id, cohort_id, learner_id, at_ms, actor, body)
         values ($1, $2, $3, $4, 0, '{\"kind\":\"system\"}', '{}')",
    )
    .bind(Uuid::now_v7())
    .bind(db.seed.tenant)
    .bind(cohort)
    .bind(learner)
    .execute(&mut *tx)
    .await?;
    tx.commit().await
}

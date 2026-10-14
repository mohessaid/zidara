//! The Zidara server: a shell around the pure kernel. PostgreSQL decides who sees what.

pub mod agent;
pub mod assessor;
pub mod commands;
pub mod error;
pub mod ids;
pub mod school;
pub mod seed;
pub mod store;
pub mod visibility;
pub mod xapi;

/// Applies the embedded migrations. Run as the schema owner, never as `zidara_app`.
pub async fn migrate(owner: &sqlx::PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(owner).await
}

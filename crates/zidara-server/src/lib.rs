//! The Zidara server: a shell around the pure kernel. PostgreSQL decides who sees what.

pub mod seed;
pub mod store;

/// Applies the embedded migrations. Run as the schema owner, never as `zidara_app`.
pub async fn migrate(owner: &sqlx::PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(owner).await
}

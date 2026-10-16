//! The Zidara server: a shell around the pure kernel. PostgreSQL decides who sees what.

pub mod agent;
pub mod assessor;
pub mod auth;
pub mod commands;
pub mod config;
pub mod error;
pub mod ids;
pub mod routes;
pub mod school;
pub mod seed;
pub mod store;
pub mod visibility;
pub mod xapi;

/// Applies the embedded migrations. Run as the schema owner, never as `zidara_app`.
pub async fn migrate(owner: &sqlx::PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(owner).await
}

use std::sync::Arc;

use assessor::Assessor;
use auth::Keys;
use school::{Clock, School};

/// Everything a request needs, shared across requests.
#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: sqlx::PgPool,
    pub school: Arc<School>,
    pub clock: Arc<dyn Clock>,
    pub keys: Keys,
    pub assessor: Arc<Assessor>,
    /// The assessment agent's user id.
    pub agent: uuid::Uuid,
    /// One marking pass per class at a time.
    pub passes: Arc<agent::Passes>,
    /// Sign in by user id, for demos only.
    pub demo_login: bool,
}

pub fn app(state: AppState) -> axum::Router {
    routes::router(state)
}

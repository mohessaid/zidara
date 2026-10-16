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

/// Adds the built web app (e.g. `web/dist`) to the API: files are served as files, any other
/// path that is not under `/api/` gets `index.html` (the app handles its own routes), and
/// unknown API paths stay a JSON 404. One process can then host the whole prototype.
pub fn with_web(api: axum::Router, dir: &std::path::Path) -> axum::Router {
    use axum::http::{StatusCode, Uri};
    use axum::response::IntoResponse;
    use tower::ServiceExt;
    use tower_http::services::{ServeDir, ServeFile};

    let files = ServeDir::new(dir).fallback(ServeFile::new(dir.join("index.html")));
    api.fallback(move |uri: Uri, request: axum::extract::Request| {
        let files = files.clone();
        async move {
            if uri.path().starts_with("/api/") {
                return (
                    StatusCode::NOT_FOUND,
                    axum::Json(serde_json::json!({ "error": "not found" })),
                )
                    .into_response();
            }
            match files.oneshot(request).await {
                Ok(response) => response.into_response(),
                Err(never) => match never {},
            }
        }
    })
}

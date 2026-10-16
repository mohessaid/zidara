//! `zidara-server migrate | seed | serve`. Migrate and seed run as the schema owner
//! (`DATABASE_URL` of the owner); serve runs as `zidara_app`.

use std::process::ExitCode;
use std::sync::Arc;
use std::time::Duration;

use zidara_server::assessor::{Assessor, Claude};
use zidara_server::auth::Keys;
use zidara_server::config::{AssessorConfig, Config};
use zidara_server::school::{School, SystemClock};
use zidara_server::{AppState, app, migrate, seed};

#[tokio::main]
async fn main() -> ExitCode {
    let command = std::env::args().nth(1).unwrap_or_default();
    let config = match Config::from_env(|key| std::env::var(key).ok()) {
        Ok(config) => config,
        Err(problem) => {
            eprintln!("{problem}");
            return ExitCode::FAILURE;
        }
    };
    match run(&command, config).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(problem) => {
            eprintln!("{problem}");
            ExitCode::FAILURE
        }
    }
}

async fn run(command: &str, config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let pool = sqlx::PgPool::connect(&config.database_url).await?;
    match command {
        "migrate" => Ok(migrate(&pool).await?),
        "seed" => Ok(seed::seed(&pool).await.map(|_| ())?),
        "serve" => {
            let assessor = match &config.assessor {
                AssessorConfig::Rubric => Assessor::Rubric,
                AssessorConfig::Claude {
                    base_url,
                    api_key,
                    model,
                } => Assessor::Claude(Claude::new(
                    base_url.clone(),
                    api_key.clone(),
                    model.clone(),
                    Duration::from_secs(10),
                )),
            };
            let state = AppState {
                pool,
                school: Arc::new(School::demo()),
                clock: Arc::new(SystemClock),
                keys: Keys::from_secret(config.secret.as_bytes()),
                assessor: Arc::new(assessor),
                agent: config.agent,
                passes: Default::default(),
                demo_login: config.demo_login,
            };
            let listener = tokio::net::TcpListener::bind(&config.addr).await?;
            println!("zidara-server listening on {} ({config:?})", config.addr);
            Ok(axum::serve(listener, app(state)).await?)
        }
        _ => Err("usage: zidara-server migrate | seed | serve".into()),
    }
}

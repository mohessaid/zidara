//! Settings from the environment, read through a lookup function so tests need no real env.

use std::fmt;

use uuid::Uuid;

use crate::assessor::DEFAULT_MODEL;
use crate::seed::Seed;

#[derive(Clone, PartialEq, Eq)]
pub enum AssessorConfig {
    Rubric,
    Claude {
        base_url: String,
        api_key: String,
        model: String,
    },
}

#[derive(Clone, PartialEq, Eq)]
pub struct Config {
    pub database_url: String,
    pub secret: String,
    pub addr: String,
    pub demo_login: bool,
    pub agent: Uuid,
    pub assessor: AssessorConfig,
    /// The built web app to serve beside the API (`ZIDARA_WEB_DIR`), if any.
    pub web_dir: Option<String>,
}

impl Config {
    pub fn from_env(var: impl Fn(&str) -> Option<String>) -> Result<Self, String> {
        let database_url = var("DATABASE_URL").ok_or("DATABASE_URL is not set")?;
        let secret = var("ZIDARA_SECRET").ok_or("ZIDARA_SECRET is not set")?;
        if secret.len() < 32 {
            return Err("ZIDARA_SECRET must be at least 32 bytes".into());
        }
        let agent = match var("ZIDARA_AGENT_ID") {
            Some(id) => Uuid::parse_str(&id).map_err(|_| "ZIDARA_AGENT_ID is not a UUID")?,
            None => Seed::DEMO.agent,
        };
        let assessor = match var("ANTHROPIC_API_KEY") {
            Some(api_key) => AssessorConfig::Claude {
                base_url: var("ANTHROPIC_BASE_URL")
                    .unwrap_or_else(|| "https://api.anthropic.com".into()),
                api_key,
                model: var("ZIDARA_ASSESSOR_MODEL").unwrap_or_else(|| DEFAULT_MODEL.into()),
            },
            None => AssessorConfig::Rubric,
        };
        Ok(Self {
            database_url,
            secret,
            addr: var("ZIDARA_ADDR").unwrap_or_else(|| "127.0.0.1:8080".into()),
            demo_login: var("ZIDARA_DEMO_LOGIN").as_deref() == Some("1"),
            agent,
            assessor,
            web_dir: var("ZIDARA_WEB_DIR"),
        })
    }
}

/// Never print secrets or keys.
impl fmt::Debug for AssessorConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssessorConfig::Rubric => f.write_str("Rubric"),
            AssessorConfig::Claude {
                base_url, model, ..
            } => write!(f, "Claude {{ base_url: {base_url:?}, model: {model:?} }}"),
        }
    }
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .field("addr", &self.addr)
            .field("demo_login", &self.demo_login)
            .field("agent", &self.agent)
            .field("assessor", &self.assessor)
            .field("web_dir", &self.web_dir)
            .finish_non_exhaustive()
    }
}

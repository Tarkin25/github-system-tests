use anyhow::Context;
use config::{Config, Environment, File};
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub server_port: u16,
    pub database: Database,
    pub log_format: LogFormat,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Copy, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Text,
    Json,
}

impl Configuration {
    pub fn new() -> anyhow::Result<Self> {
        let config = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(Environment::default())
            .build()?;

        config
            .try_deserialize()
            .context("failed to deserialize configuration")
    }

    pub async fn create_pool(&self) -> anyhow::Result<PgPool> {
        PgPoolOptions::new()
            .connect(&self.database.url)
            .await
            .context("failed to obtain database connection")
    }
}

pub fn init_logging(configuration: &Configuration) {
    let subscriber = tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env());

    if let LogFormat::Json = configuration.log_format {
        subscriber.json().init();
    } else {
        subscriber.init();
    }
}

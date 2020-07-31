pub mod crypto;

use color_eyre::Result;
use eyre::WrapErr;
use dotenv::dotenv;

use serde::Deserialize;
use sqlx::postgres::PgPool;

use crypto::CryptoService;
use std::sync::Arc;
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub secret_key: String,
    pub jwt_secret: String
}

impl Config {

    #[instrument]
    pub fn from_env() -> Result<Config> {
        dotenv().ok();

        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        info!("Loading Config");
        // using config crate to get the variables from env to the config struct
        let mut c = config::Config::new();

        c.merge(config::Environment::default())?;

        c.try_into()
            .context("Loading variables from env")
    }

    #[instrument(skip(self))]
    pub async fn db_pool(&self) -> Result<PgPool> {
        info!("Creating database connection pool.");
        PgPool::builder()
            .connect_timeout(std::time::Duration::from_secs(30))
            .build(&self.database_url)
            .await
            .context("creating database connection pool")
    }

    #[instrument(skip(self))]
    pub fn hashing(&self) -> CryptoService {
        CryptoService {
            key: Arc::new(self.secret_key.clone()),
            jwt_secret: Arc::new(self.jwt_secret.clone()),
        }
    }
}

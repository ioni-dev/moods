#[macro_use]
extern crate validator_derive;

mod config;
mod db;
mod errors;
mod handlers;
mod models;

use color_eyre::Result;
use crate::config::Config;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use tracing::{info, instrument};
use handlers::app_config;

#[actix_rt::main]
#[instrument]
async fn main() -> Result<()> {

    let config = Config::from_env().expect("Server configuration");

    let pool = config.db_pool().await.expect("Database configuration");

    let hashing = config.hashing();

    info!("Starting server at http://{}:{}/", config.host, config.port);
    // instantiation of server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .data(hashing.clone())
            .configure(app_config)
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await?;

    Ok(())
}

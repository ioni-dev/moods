mod config;

use color_eyre::Result;
use crate::config::Config;
use actix_web::{App, HttpServer, middleware::Logger};

#[actix_rt::main]
async fn main() -> Result<()> {

    let config = Config::from_env().expect("Server configuration");

    // instantiation of server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await?;

    Ok(())
}

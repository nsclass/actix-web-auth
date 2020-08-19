#[macro_use]
extern crate validator_derive;

mod config;
mod handlers;
mod models;

use crate::config::Config;
use actix_web::{middleware::Logger, App, HttpServer};
use color_eyre::Result;
use handlers::app_config;
use tracing::info;

#[actix_rt::main]
async fn main() -> Result<()> {
    let conf = Config::from_env().expect("server configuration");
    let pool = conf.db_pool().await.expect("Database configuration");
    let crypto_service = conf.crypto_service();

    info!("starting server at http://{}:{}", conf.host, conf.port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .data(crypto_service.clone())
            .configure(app_config)
    })
    .bind(format!("{}:{}", conf.host, conf.port))?
    .run()
    .await?;

    Ok(())
}

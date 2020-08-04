mod config;
mod handlers;

use crate::config::Config;
use actix_web::{middleware::Logger, App, HttpServer};
use color_eyre::Result;
use handlers::app_config;
use tracing::info;

#[actix_rt::main]
async fn main() -> Result<()> {
    let conf = Config::from_env().expect("server configuration");
    info!("starting server at http://{}:{}", conf.host, conf.port);

    HttpServer::new(move || App::new().wrap(Logger::default()).configure(app_config))
        .bind(format!("{}:{}", conf.host, conf.port))?
        .run()
        .await?;

    Ok(())
}
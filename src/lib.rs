pub mod city;
pub mod db;
pub mod routes;

use crate::db::{db_connect, PoolDb};
use crate::routes::fetch_cities::fetch_cities_handler;
use crate::routes::{create_city::create_city_handler, health::health_handler};
use actix_web::{middleware, web, App, HttpServer};
use log::info;
use std::env;
use std::io::{Error, ErrorKind};

pub struct AppState {
    pub db: PoolDb,
}

pub async fn create_server() -> std::io::Result<()> {
    pretty_env_logger::init_timed();

    let pool = db_connect().await?;

    let addr = get_env("CITY_API_ADDR").unwrap_or("127.0.0.1".to_owned());
    let port = get_env("CITY_API_PORT").unwrap_or("2022".to_owned());
    info!("Server running on http://{}:{}", addr, port);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(health_handler)
            .service(create_city_handler)
            .service(fetch_cities_handler)
    })
    .bind(format!("{}:{}", addr, port))?
    .run()
    .await
}

pub fn get_env(key: &str) -> Result<String, Error> {
    env::var(key).map_err(|_| Error::new(ErrorKind::NotFound, format!("'{}' not found", key)))
}

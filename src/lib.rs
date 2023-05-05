mod city;
mod routes;

use crate::routes::health::health_handler;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use log::info;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use std::io::{Error, ErrorKind};

pub async fn create_server() -> std::io::Result<()> {
    pretty_env_logger::init_timed();

    let pool = db_connect().await?;

    let addr = get_env("CITY_API_ADDR").unwrap_or("127.0.0.1".to_owned());
    let port = get_env("CITY_API_PORT").unwrap_or("2022".to_owned());
    info!("Server running on http://{}:{}", addr, port);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(health_handler)
    })
    .bind(format!("{}:{}", addr, port))?
    .run()
    .await
}

async fn db_connect() -> Result<Pool<Postgres>, Error> {
    let _db_user = get_env("CITY_API_DB_USER")?;
    let _db_pwd = get_env("CITY_API_DB_PWD")?;
    let db_url = get_env("CITY_API_DB_URL")?;

    let result = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await;

    match result {
        Ok(db) => Ok(db),
        Err(error) => Err(Error::new(ErrorKind::ConnectionRefused, error.to_string())),
    }
}

fn get_env(key: &str) -> Result<String, Error> {
    env::var(key).map_err(|_| Error::new(ErrorKind::NotFound, format!("'{}' not found", key)))
}

use actix_web::{get, web, App, HttpServer};
use log::info;
use sea_orm::{Database, DatabaseConnection};
use std::env;
use std::io::{Error, ErrorKind};

#[get("/")]
async fn index() -> String {
    "Hello city-api!".to_owned()
}

pub async fn create_server() -> std::io::Result<()> {
    pretty_env_logger::init_timed();

    let db = db_connect().await?;

    let addr = get_env("CITY_API_ADDR").unwrap_or("127.0.0.1".to_owned());
    let port = get_env("CITY_API_PORT").unwrap_or("2022".to_owned());
    info!("Server running on http://{}:{}", addr, port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(web::scope("/api/v1").service(index))
    })
    .bind(format!("{}:{}", addr, port))?
    .run()
    .await
}

async fn db_connect() -> Result<DatabaseConnection, Error> {
    let _db_user = get_env("CITY_API_DB_USER")?;
    let _db_pwd = get_env("CITY_API_DB_PWD")?;
    let db_url = get_env("CITY_API_DB_URL")?;

    let result = Database::connect(db_url).await;

    match result {
        Ok(db) => Ok(db),
        Err(error) => Err(Error::new(ErrorKind::ConnectionRefused, error.to_string())),
    }
}

fn get_env(key: &str) -> Result<String, Error> {
    env::var(key).map_err(|_| Error::new(ErrorKind::NotFound, format!("'{}' not found", key)))
}

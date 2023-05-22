use crate::get_env;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::io::{Error, ErrorKind};

pub type PoolDb = Pool<Postgres>;

pub async fn db_connect() -> Result<PoolDb, Error> {
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

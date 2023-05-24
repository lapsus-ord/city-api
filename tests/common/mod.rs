use city_api::db::{db_connect, PoolDb};

pub async fn setup() -> Result<PoolDb, std::io::Error> {
    db_connect().await
}

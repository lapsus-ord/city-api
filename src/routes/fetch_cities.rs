use crate::city::City;
use crate::AppState;
use actix_web::{get, web, HttpResponse};
use log::trace;

#[get("/city")]
pub async fn fetch_cities_handler(data: web::Data<AppState>) -> HttpResponse {
    let result: Result<Vec<City>, sqlx::Error> = sqlx::query_file_as!(City, "sql/fetch_cities.sql")
        .fetch_all(&data.db)
        .await;

    trace!(
        "{:?} fetched cities",
        result.as_ref().map(|v| v.len()).unwrap_or(0)
    );

    match result {
        Ok(cities) => HttpResponse::Ok().json(cities),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

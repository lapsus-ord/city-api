use crate::city::City;
use crate::AppState;
use actix_web::{get, web, HttpResponse};

#[get("/city")]
pub async fn fetch_cities_handler(data: web::Data<AppState>) -> HttpResponse {
    let result = sqlx::query_file_as!(City, "sql/fetch_cities.sql")
        .fetch_all(&data.db)
        .await;

    match result {
        Ok(cities) => HttpResponse::Ok().json(cities),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

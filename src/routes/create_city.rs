use crate::city::{City, CreateCity};
use crate::AppState;
use actix_web::{post, web, HttpResponse};
use log::trace;

#[post("/city")]
pub async fn create_city_handler(
    json: web::Json<CreateCity>,
    data: web::Data<AppState>,
) -> HttpResponse {
    trace!("new city: {:?}", json);
    let city = json.into_inner();

    let result = sqlx::query_file_as!(
        City,
        "sql/insert_city.sql",
        city.department_code,
        city.insee_code,
        city.zip_code,
        city.name,
        city.lat,
        city.lon
    )
    .fetch_optional(&data.db)
    .await;

    match result {
        Ok(Some(created_city)) => HttpResponse::Created().json(created_city),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

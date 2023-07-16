use actix_web::{get, HttpResponse};

#[get("/_health")]
pub async fn health_handler() -> HttpResponse {
    HttpResponse::NoContent().finish()
}

use actix_web::get;
use actix_web::web::Json;
use serde_json::{json, Value};

#[get("/_health")]
pub async fn health_handler() -> Json<Value> {
    Json(json!({ "health": "Alive! 💓" }))
}

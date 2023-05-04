use city_api::create_server;

#[actix_web::main]
async fn main() {
    create_server().await.expect("Cannot start the server");
}

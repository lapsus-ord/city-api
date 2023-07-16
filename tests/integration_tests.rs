mod common;

use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::{test, App};

use city_api::city::CreateCity;
use city_api::routes::create_city::create_city_handler;
use city_api::routes::fetch_cities::fetch_cities_handler;
use city_api::routes::health::health_handler;
use city_api::AppState;

#[actix_web::test]
async fn test_health_get() {
    let app = test::init_service(App::new().service(health_handler)).await;
    let req = test::TestRequest::get().uri("/_health").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);
}

#[actix_web::test]
async fn test_fetch_cities() {
    let db = common::setup().await.unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .service(fetch_cities_handler),
    )
    .await;
    let req = test::TestRequest::get().uri("/city").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let _: Vec<CreateCity> = test::read_body_json(resp).await;
}

#[actix_web::test]
async fn test_create_city() {
    let db = common::setup().await.unwrap();

    let city = CreateCity {
        department_code: "test".to_owned(),
        insee_code: None,
        zip_code: None,
        name: "ville".to_owned(),
        lat: 2.0,
        lon: 3.0,
    };

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .service(create_city_handler),
    )
    .await;
    let req = test::TestRequest::post()
        .uri("/city")
        .set_json(&city)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    assert_eq!(resp.status(), StatusCode::CREATED);

    let created_city: CreateCity = test::read_body_json(resp).await;
    assert_eq!(created_city, city);
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct City {
    pub id: i32,
    pub department_code: String,
    pub insee_code: Option<String>,
    pub zip_code: Option<String>,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CreateCity {
    pub department_code: String,
    pub insee_code: Option<String>,
    pub zip_code: Option<String>,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
}

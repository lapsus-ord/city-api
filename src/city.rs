use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct City {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub department_code: String,
    pub insee_code: Option<String>,
    pub zip_code: Option<String>,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
}

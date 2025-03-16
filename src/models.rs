use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AddBatchRequest {
    pub symbol: String,
    pub values: Vec<f64>,
}

#[derive(Deserialize)]
pub struct StatsQuery {
    pub symbol: String,
    pub k: Option<u32>,
}

#[derive(Serialize, Default)]
pub struct StatsResponse {
    pub min: f64,
    pub max: f64,
    pub last: f64,
    pub avg: f64,
    pub var: f64,
}

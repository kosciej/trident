use serde::Deserialize;

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

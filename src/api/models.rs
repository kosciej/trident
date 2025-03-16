use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct StatsResponse {
    pub total_items: usize,
    pub average_value: f64,
}

#[derive(Deserialize)]
pub struct AddBatchRequest {
    pub items: Vec<i32>,
}

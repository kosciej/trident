use crate::api::models::{AddBatchRequest, StatsResponse};
use axum::Json;
use serde_json::json;

pub async fn stats_handler() -> Json<StatsResponse> {
    let stats = StatsResponse {
        total_items: 100,
        average_value: 50.5,
    };
    Json(stats)
}

pub async fn add_batch_handler(Json(payload): Json<AddBatchRequest>) -> Json<serde_json::Value> {
    println!("Received batch: {:?}", payload.items);
    Json(json!({ "status": "success" }))
}

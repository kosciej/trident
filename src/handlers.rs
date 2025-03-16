use crate::models::{AddBatchRequest, StatsQuery, StatsResponse};
use axum::{
    extract::{Json, Query},
    http::StatusCode,
    routing::{get, post},
    Router,
};

#[axum::debug_handler]
async fn stats_handler(query: Query<StatsQuery>) -> Result<Json<StatsResponse>, StatusCode> {
    let k = query.k.unwrap_or(1);
    if !(1..=8).contains(&k) {
        return Err(StatusCode::BAD_REQUEST);
    }

    println!("k = {}", k);

    Ok(Json(StatsResponse::default()))
}

// POST /add_batch/
async fn add_batch_handler(Json(payload): Json<AddBatchRequest>) -> StatusCode {
    println!(
        "Received batch for symbol {}: {:?}",
        payload.symbol, payload.values
    );

    if payload.values.len() > 10000 {
        return StatusCode::BAD_REQUEST;
    }

    StatusCode::OK
}

pub fn create_api_routes() -> Router {
    Router::new()
        .route("/stats", get(stats_handler))
        .route("/add_batch", post(add_batch_handler))
}

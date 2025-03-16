use axum::{
    extract::{Query, Json},
    routing::{get, post},
    Router,
    http::StatusCode,
};
use crate::api::models::{StatsResponse, AddBatchRequest, StatsQuery};


#[axum::debug_handler]
async fn stats_handler(
    _query: Query<StatsQuery>
) -> Json<StatsResponse> {
    Json(StatsResponse::default())
}

// POST /add_batch/
async fn add_batch_handler(Json(payload): Json<AddBatchRequest>) -> StatusCode {
    println!("Received batch for symbol {}: {:?}", payload.symbol, payload.values);

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
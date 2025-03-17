use crate::{
    models::{AddBatchRequest, StatsQuery},
    state::AppState,
};

use calculator_lib::StatsResponse;

use axum::{
    extract::{Json, Query},
    http::StatusCode,
    routing::{get, post},
    Router,
};

#[axum::debug_handler]
async fn stats_handler(
    axum::extract::State(state): axum::extract::State<AppState>,
    query: Query<StatsQuery>,
) -> Result<Json<StatsResponse>, StatusCode> {
    let k = query.k.unwrap_or(1);
    if !(1..=8).contains(&k) {
        return Err(StatusCode::BAD_REQUEST);
    }

    state
        .stats(&query.symbol, k)
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

// POST /add_batch/
async fn add_batch_handler(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(payload): Json<AddBatchRequest>,
) -> StatusCode {
    println!(
        "Received batch for symbol {}: {:?}",
        payload.symbol, payload.values
    );

    if payload.values.len() > 10000 {
        return StatusCode::BAD_REQUEST;
    }

    state.append(payload.symbol, &payload.values);

    StatusCode::OK
}

pub fn create_api_routes(state: AppState) -> Router {
    Router::new()
        .route("/stats", get(stats_handler))
        .route("/add_batch", post(add_batch_handler))
        .with_state(state)
}

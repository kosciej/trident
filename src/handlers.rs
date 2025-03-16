use crate::{
    calculator::{
        naive::{self, NaiveCalculator},
        Calculator,
    },
    models::{AddBatchRequest, StatsQuery, StatsResponse},
    AppState,
};
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

    if let Some(calculator) = state
        .calculators
        .lock()
        .expect("Poisoned lock")
        .get_mut(&query.symbol)
    {
        let stats = calculator.calculate_stats(k as u8);
        Ok(Json(stats))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
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

    {
        let mut map = state.calculators.lock().expect("Poisoned mutex");
        map.entry(payload.symbol)
            .and_modify(|e| e.append(&payload.values))
            .or_insert_with(|| {
                let mut calc = crate::calculator::naive::NaiveCalculator::new(100_000_000);
                calc.append(&payload.values);
                calc
            });
    }

    StatusCode::OK
}

pub fn create_api_routes(state: AppState) -> Router {
    Router::new()
        .route("/stats", get(stats_handler))
        .route("/add_batch", post(add_batch_handler))
        .with_state(state)
}

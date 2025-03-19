use crate::{
    models::{AddBatchRequest, StatsQuery, StatsResponse},
    state::AppState,
};

use axum::{
    extract::{Json, Query},
    http::StatusCode,
    routing::{get, post},
    Router,
};

async fn stats_handler(
    axum::extract::State(state): axum::extract::State<AppState>,
    query: Query<StatsQuery>,
) -> Result<Json<StatsResponse>, StatusCode> {
    tracing::debug!(
        "Received stats request for symbol {}: k = {:?}",
        query.symbol,
        query.k
    );
    let k = query.k.unwrap_or(1);
    if !(1..=8).contains(&k) {
        return Err(StatusCode::BAD_REQUEST);
    }

    state
        .stats(&query.symbol, k)
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn add_batch_handler(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(payload): Json<AddBatchRequest>,
) -> StatusCode {
    tracing::debug!(
        "Received batch for symbol {}: {:?}",
        payload.symbol,
        payload.values
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_stats_handler() {
        let state = crate::state::new();
        state.append("TEST".into(), &[10.0, 20.0, 30.0]);

        let app = create_api_routes(state);

        // Test valid request
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/stats?symbol=TEST&k=1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // Test invalid k value
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/stats?symbol=TEST&k=9")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        // Test non-existent symbol
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/stats?symbol=INVALID")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}

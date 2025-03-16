pub mod handlers;
pub mod models;

use axum::{
    routing::{get, put},
    Router,
};

pub fn create_router() -> Router {
    Router::new()
        .route("/stats", get(handlers::stats_handler))
        .route("/add_batch", put(handlers::add_batch_handler))
}

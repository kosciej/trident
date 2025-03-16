pub mod handlers;
pub mod models;

use axum::Router;
pub use self::handlers::create_api_routes;
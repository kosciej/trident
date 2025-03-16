use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
mod calculator;
mod handlers;
mod models;

#[derive(Clone)]
struct AppState {
    calculators: Arc<Mutex<HashMap<String, calculator::naive::NaiveCalculator>>>,
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    let mut calculators: HashMap<String, calculator::naive::NaiveCalculator> = HashMap::new();
    calculators.insert(
        "instrument1".to_string(),
        calculator::naive::NaiveCalculator::new(100_000_000),
    );

    let shared_state = AppState {
        calculators: Arc::new(Mutex::new(calculators)),
    };

    let app = handlers::create_api_routes(shared_state);

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

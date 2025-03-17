use std::net::SocketAddr;
use tokio::net::TcpListener;
mod handlers;
mod models;
mod state;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    let shared_state = state::new();

    let app = handlers::create_api_routes(shared_state);

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

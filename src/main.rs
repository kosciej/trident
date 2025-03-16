use std::net::SocketAddr;
use tokio::net::TcpListener;
mod handlers;
mod models;

#[tokio::main]
async fn main() {
    let app = handlers::create_api_routes();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

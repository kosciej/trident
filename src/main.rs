use std::net::SocketAddr;
use tokio::net::TcpListener;
mod api;

#[tokio::main]
async fn main() {
    let app = api::create_api_routes();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

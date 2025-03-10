use axum::{Router, routing::get};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_world));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Server running on http://{}", addr);

    // Create a TCP listener
    let listener = TcpListener::bind(addr).await.unwrap();

    // Serve the application
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn hello_world() -> &'static str{
    "Hello, World"
}
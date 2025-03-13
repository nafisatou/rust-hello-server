use axum::{
    extract::Multipart,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/upload", post(upload_file))
        .layer(CorsLayer::new().allow_methods(Any).allow_origin(Any));

    let addr = SocketAddr::from(([127, 0, 0, 2], 3000));
    println!("🚀 Server running on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app.into_make_service()) // Use TcpListener instead of hyper::Server
        .await
        .unwrap();
}

async fn upload_file(mut multipart: Multipart) -> Result<String, StatusCode> {
    if create_dir_all("./uploads").is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap_or("unknown").to_string();
        let data = field.bytes().await.unwrap();

        let file_path = format!("./uploads/{}", file_name);
        let mut file = File::create(&file_path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        file.write_all(&data)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        return Ok(format!(" File '{}' uploaded successfully!", file_name));
    }

    Err(StatusCode::BAD_REQUEST)
}

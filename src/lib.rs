use::tokio;
use axum::{routing::{get}, Router};

pub async fn run() {
    let app = Router::new().route("/", get(|| async { "Hello World" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
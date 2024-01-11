use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub async fn returns_201() -> Response {
    (StatusCode::CREATED, "This is a 201").into_response()
}
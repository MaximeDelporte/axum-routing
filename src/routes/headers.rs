use axum::http::header::{CONTENT_LENGTH};
use axum::http::HeaderMap;

pub async fn headers(headers: HeaderMap) -> String {
    let content_length_result = headers.get(CONTENT_LENGTH).unwrap().to_str();

    return match content_length_result {
        Ok(content_length) => String::from(content_length),
        _ => "An error happened".to_string(),
    }
}
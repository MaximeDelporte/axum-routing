use axum::http::header::{CONTENT_LENGTH, USER_AGENT};
use axum::http::HeaderMap;

pub async fn headers(headers: HeaderMap) -> String {
    let content_length_result = headers.get(CONTENT_LENGTH).unwrap().to_str();
    let content_length = content_length_result.unwrap();

    let user_agent_result = headers.get(USER_AGENT).unwrap().to_str();
    let user_agent = user_agent_result.unwrap();

    return format!("Content-Length: {content_length}\nUser-Agent: {user_agent}");
}
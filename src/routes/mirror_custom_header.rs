use axum::http::HeaderMap;

pub async fn mirror_custom_header(headers: HeaderMap) -> String {
    let token_value = headers.get("x-auth-token").unwrap();
    return token_value.to_str().unwrap().to_owned();
}
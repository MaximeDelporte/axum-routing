use axum::{
    http::StatusCode,
    response::{Response},
    middleware::{Next},
    extract::{Request},
};
use axum::http::header::ToStrError;
use crate::routing::read_middleware_custom_header::HeaderMessage;

pub async fn set_middleware_custom_header(
    mut req: Request,
    next: Next
) -> Result<Response, StatusCode> {

    let header_message = req.headers()
        .get("message")
        .ok_or_else(|| StatusCode::BAD_REQUEST)?;

    let message = header_message
        .to_str()
        .map_err(|_error: ToStrError| StatusCode::BAD_REQUEST)?
        .to_owned();

    let extensions = req.extensions_mut();
    extensions.insert(HeaderMessage(message));

    Ok(next.run(req).await)
}
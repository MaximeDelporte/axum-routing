use axum::http::{StatusCode};
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RequestUser {
    username: String,
    password: String
}

#[derive(Serialize)]
pub struct ResponseUser {
    username: String,
    id: i32,
}

pub async fn validate_with_serde(Json(user): Json<RequestUser>) -> Result<Json<ResponseUser>, Response> {
    if user.password != "Azerty123".to_string() {
        let response = (
            StatusCode::IM_A_TEAPOT,
            "The password isn't correct".to_owned()
        ).into_response();

        Err(response)
    } else {
        let response = ResponseUser { username: user.username, id: 42 };
        Ok(Json(response))
    }
}
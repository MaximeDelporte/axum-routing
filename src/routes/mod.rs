mod hello_world;
mod mirror_body_string;
mod mirror_body_json;
mod path_variables;
mod query_parameters;
mod headers;
mod mirror_custom_header;
mod middleware_message;
mod read_middleware_custom_header;
mod set_middleware_custom_header;
mod always_error;
mod returns_201;
mod get_json;
mod validate_with_serde;
mod custom_json_extractor;

use axum::{routing::{get, post}, Router};
use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};
use hello_world::hello_world;
use mirror_body_string::mirror_body_string;
use mirror_body_json::mirror_body_json;
use path_variables::{path_variables, hard_coded_path};
use query_parameters::query_parameters;
use headers::headers;
use mirror_custom_header::mirror_custom_header;
use middleware_message::middleware_message;
use read_middleware_custom_header::read_middleware_custom_header;
use set_middleware_custom_header::set_middleware_custom_header;
use always_error::always_error;
use returns_201::returns_201;
use get_json::get_json;
use validate_with_serde::validate_with_serde;
use custom_json_extractor::custom_json_extractor;

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let shared_data = SharedData {
        message: "Message from shared data, I am a state now.".to_owned(),
    };

    Router::new()
        .route(
            "/middleware_custom_header",
            get(read_middleware_custom_header)
        )
        .route_layer(axum::middleware::from_fn(set_middleware_custom_header))
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/8", get(hard_coded_path))
        .route("/path_variables/:id", get(path_variables))
        .route("/query_parameters", get(query_parameters))
        .route("/headers", post(headers))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_message))
        .with_state(shared_data)
        .layer(cors)
        .route("/always_error", get(always_error))
        .route("/returns_201", post(returns_201))
        .route("/get_json", get(get_json))
        .route("/validate_data", post(validate_with_serde))
        .route("/custom_json_extractor", post(custom_json_extractor))
}
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

use axum::{routing::{get, post}, Router, Extension};
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

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let shared_data = SharedData {
        message: "Message from shared data.".to_owned(),
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
        .layer(Extension(shared_data))
        .layer(cors)
}
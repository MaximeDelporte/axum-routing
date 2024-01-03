mod hello_world;
mod mirror_body_string;
mod mirror_body_json;
mod path_variables;
mod query_parameters;
mod headers;
mod mirror_custom_header;

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

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/8", get(hard_coded_path))
        .route("/path_variables/:id", get(path_variables))
        .route("/query_parameters", get(query_parameters))
        .route("/headers", post(headers))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .layer(cors)
}
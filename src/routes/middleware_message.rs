use axum::Extension;
use crate::routes::SharedData;

pub async fn middleware_message(Extension(shared_data): Extension<SharedData>) -> String {
    return shared_data.message;
}
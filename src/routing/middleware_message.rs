use axum::extract::State;
use crate::routing::SharedData;

pub async fn middleware_message(State(state): State<SharedData>) -> String {
    return state.message;
}
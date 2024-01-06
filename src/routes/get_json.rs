use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    message: String,
    count: i32,
    username: String,
}

pub async fn get_json() -> Json<Data> {
    let data = Data {
        message: "Message from get_json function !".to_owned(),
        count: 8080,
        username: "Maxime".to_owned()
    };

    return Json(data);
}
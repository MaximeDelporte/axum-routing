use axum::{
    async_trait,
    extract::{Request, FromRequest},
    body::Bytes, http::{StatusCode},
    RequestExt,
    Json}
;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct RequestUser {
    #[validate(email(message = "must be a valid email"))]
    pub username: String,
    #[validate(length(min = 8, message = "must have at least 8 characters"))]
    pub password: String,
}

#[async_trait]
impl<S> FromRequest<S> for RequestUser
    where
        Bytes: FromRequest<S>,
        S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(user) = req
            .extract::<Json<RequestUser>, _>()
            .await
            .map_err(|error| (StatusCode::BAD_REQUEST, format!("{}", error)))?;

        if let Err(errors) = user.validate() {
            return Err((StatusCode::BAD_REQUEST, format!("{}", errors)));
        }

        Ok(user)
    }
}

pub async fn custom_json_extractor(user: RequestUser) -> String {
    dbg!(&user);
    return "User verification is done.".to_owned();
}
use axum::{extract::FromRequest, response::IntoResponse};
use serde::Serialize;
use crate::error::ApiError;

#[derive(FromRequest, Debug)]
#[from_request(via(axum::Json), rejection(ApiError))]
pub struct Json<T>(pub T);

impl<T: Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> axum::response::Response {
        let Self(value) = self;
        axum::Json(value).into_response()
    }
}

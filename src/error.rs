use axum::{
    extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse,
};
use serde_json::json;

#[derive(Debug)]
pub struct ApiError {
    message: String,
    status: StatusCode,
}

impl ApiError {
    pub fn new(status: StatusCode, message: &'static str) -> Self {
        Self {
            message: message.to_string(),
            status,
        }
    }
}

impl From<JsonRejection> for ApiError {
    fn from(rejection: JsonRejection) -> Self {
        Self {
            message: rejection.body_text(),
            status: rejection.status(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let payload = json!({
            "message": self.message,
            "status": self.status.as_str(),
        });

        (self.status, axum::Json(payload)).into_response()
    }
}

use thiserror::Error;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Feed parsing error: {0}")]
    FeedParsingError(String),

    #[error("Summarization API error: {0}")]
    SummarizationError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DatabaseError(ref e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::BadRequest(ref e) => (StatusCode::BAD_REQUEST, e.to_string()),
            AppError::FeedParsingError(ref e) => (StatusCode::BAD_REQUEST, e.to_string()),
            AppError::SummarizationError(ref e) => (StatusCode::SERVICE_UNAVAILABLE, e.to_string()),
            AppError::NotFound(ref e) => (StatusCode::NOT_FOUND, e.to_string()),
            AppError::ValidationError(ref e) => (StatusCode::BAD_REQUEST, e.to_string()),
            AppError::InternalServerError(ref e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };

        let body = Json(json!({
            "error": {
                "message": error_message,
                "type": format!("{:?}", self)
            }
        }));

        (status, body).into_response()
    }
}

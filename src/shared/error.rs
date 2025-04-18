use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;
use sqlx::Error as SqlxError;
use serde::Serialize;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DbError(#[from] SqlxError),
    #[error("Not found")]
    NotFound,
    #[error("Invalid input: {0}")]
    BadRequest(String),
    #[error("Unexpected error")]
    Unexpected,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let message = self.to_string();
        let body = ErrorResponse { error: message };

        match self {
            AppError::DbError(_) | AppError::Unexpected => {
                HttpResponse::InternalServerError().json(body)
            }
            AppError::BadRequest(_) => HttpResponse::BadRequest().json(body),
            AppError::NotFound => HttpResponse::NotFound().json(body),
        }
    }
}


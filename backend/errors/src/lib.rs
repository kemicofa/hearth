use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::ValidationErrors;

pub type ErrorCode = String;

pub type FieldName = String;
pub type FieldErrorCode = String;

pub enum FieldValidationError {
    TooShort,
    TooLong,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum TechnicalErrors {
    NotFound(String),
    Unexpected(String, Option<String>),
    Unknown,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Error)]
#[serde(tag = "error", content = "message")]
pub enum HearthError {
    #[error("technical error")]
    Technical(TechnicalErrors),
    #[error("domain error")]
    Domain(ErrorCode),
    #[error("unexpected error")]
    Unexpected(ErrorCode),
    #[error("validation error")]
    Validation(ErrorCode, ValidationErrors),
}

impl HearthError {
    pub fn not_found(code: ErrorCode) -> Self {
        Self::Technical(TechnicalErrors::NotFound(code))
    }

    pub fn unexpected(code: ErrorCode, reason: Option<String>) -> Self {
        Self::Technical(TechnicalErrors::Unexpected(code, reason))
    }
}

impl ResponseError for HearthError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            HearthError::Domain(_) => StatusCode::BAD_GATEWAY,
            HearthError::Technical(TechnicalErrors::NotFound(_)) => StatusCode::NOT_FOUND,
            HearthError::Technical(TechnicalErrors::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
            HearthError::Technical(TechnicalErrors::Unexpected(_, _)) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            HearthError::Validation(_, _) => StatusCode::BAD_REQUEST,
            HearthError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        HttpResponse::build(status_code).json(serde_json::json!(self))
    }
}

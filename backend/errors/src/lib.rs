use actix_web::{ HttpResponse, ResponseError, http::StatusCode };
use serde::{ Deserialize, Serialize };
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
    Unknown,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Error)]
#[serde(tag = "error", content = "message")]
pub enum ZwitterError {
    #[error("technical error")] Technical(TechnicalErrors),
    #[error("domain error")] Domain(ErrorCode),
    #[error("unexpected error")] Unexpected(ErrorCode),
    #[error("validation error")] Validation(ErrorCode, ValidationErrors),
}

impl ZwitterError {
    pub fn not_found(code: ErrorCode) -> Self {
        Self::Technical(TechnicalErrors::NotFound(code))
    }
}

impl ResponseError for ZwitterError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ZwitterError::Domain(_) => StatusCode::BAD_GATEWAY,
            ZwitterError::Technical(TechnicalErrors::NotFound(_)) => StatusCode::NOT_FOUND,
            ZwitterError::Technical(TechnicalErrors::Unknown) => StatusCode::INTERNAL_SERVER_ERROR,
            ZwitterError::Validation(_, _) => StatusCode::BAD_REQUEST,
            ZwitterError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        HttpResponse::build(status_code).json(serde_json::json!(self))
    }
}

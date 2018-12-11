use actix_web::{error::Error, http::StatusCode, HttpResponse};
use failure;
use serde_json;

#[derive(Debug)]
pub enum AppError {
    ServerError {
        err: Error,
        status: StatusCode,
        message: String,
    },
    ClientError {
        status: StatusCode,
        message: String,
    },
}

impl AppError {
    fn status(&self) -> &StatusCode {
        match self {
            AppError::ClientError { status, .. } => status,
            AppError::ServerError { status, .. } => status,
        }
    }

    fn message(&self) -> &String {
        match self {
            AppError::ClientError { message, .. } => message,
            AppError::ServerError { message, .. } => message,
        }
    }

    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "status": self.status().as_u16(),
            "error": self.status().canonical_reason(),
            "message": self.message()
        })
    }

    pub fn to_response(self) -> HttpResponse {
        let payload = self.to_json();
        match self {
            AppError::ClientError { status, .. } => {
                HttpResponse::build(status).json(payload)
            }
            AppError::ServerError { err, status, .. } => {
                HttpResponse::from_error(err)
                    .into_builder()
                    .status(status)
                    .json(payload)
            }
        }
    }
}

pub fn bad_request(message: String) -> AppError {
    AppError::ClientError {
        status: StatusCode::BAD_REQUEST,
        message,
    }
}

pub fn unauthorized(message: String) -> AppError {
    AppError::ClientError {
        status: StatusCode::UNAUTHORIZED,
        message,
    }
}

pub fn bad_implementation(err: failure::Error) -> AppError {
    AppError::ServerError {
        err: Error::from(err),
        status: StatusCode::INTERNAL_SERVER_ERROR,
        message: "An unexpected error occurred.".into(),
    }
}

pub fn service_unavailable(err: failure::Error) -> AppError {
    AppError::ServerError {
        err: Error::from(err),
        status: StatusCode::SERVICE_UNAVAILABLE,
        message: "The server is currently unable to handle the request.".into(),
    }
}

pub type AppResult<T> = Result<T, AppError>;

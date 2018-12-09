use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use failure;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::{
    fmt::{self, Display},
    result,
};

#[derive(Debug, Fail)]
pub struct Error {
    status: StatusCode,
    message: String,
    reason: Option<failure::Error>,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Error", 3)?;
        state.serialize_field("status", &self.status.as_u16())?;
        state.serialize_field("error", &self.status.canonical_reason())?;
        state.serialize_field("message", &self.message)?;
        state.end()
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        if let Some(err) = &self.reason {
            eprintln!("{:?}", err);
        }
        HttpResponse::build(self.status).json(self)
    }
}

pub fn unauthorized(message: String) -> Error {
    Error {
        status: StatusCode::UNAUTHORIZED,
        message,
        reason: None,
    }
}

pub fn bad_implementation(err: failure::Error) -> Error {
    Error {
        reason: Some(err),
        status: StatusCode::INTERNAL_SERVER_ERROR,
        message: "An unexpected Error occurred.".into(),
    }
}

pub fn service_unavailable(err: failure::Error) -> Error {
    Error {
        reason: Some(err),
        status: StatusCode::SERVICE_UNAVAILABLE,
        message: "The server is currently unable to handle the request.".into(),
    }
}

pub type AppResult<T> = result::Result<T, Error>;

use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::{
    fmt::{self, Debug, Display},
    result,
};

#[derive(Debug, Fail)]
pub struct Error {
    status: StatusCode,
    error: &'static str,
    message: &'static str,
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
        state.serialize_field("error", &self.error)?;
        state.serialize_field("message", &self.message)?;
        state.end()
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status).json(self)
    }
}

pub fn unauthorized(message: &'static str) -> Error {
    Error {
        status: StatusCode::UNAUTHORIZED,
        error: "Unauthorized",
        message,
    }
}

pub fn bad_implementation(_err: impl Display+Debug) -> Error {
    Error {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        error: "Internal Server Error",
        message: "An unexpected Error occurred.",
    }
}

pub fn service_unavailable(_err: impl Display+Debug) -> Error {
    Error {
        status: StatusCode::SERVICE_UNAVAILABLE,
        error: "Service Unavailable",
        message: "The server is currently unable to handle the request.",
    }
}

pub type Result<T> = result::Result<T, Error>;

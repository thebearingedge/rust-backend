use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::fmt;

#[derive(Fail, Debug)]
pub struct Error {
    status: StatusCode,
    error: String,
    message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
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

impl Error {
    pub fn unauthorized(message: &str) -> Self {
        Error {
            status: StatusCode::UNAUTHORIZED,
            error: "Unauthorized".to_owned(),
            message: message.to_owned(),
        }
    }

    pub fn bad_implementation() -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            error: "Internal Server Error".to_owned(),
            message: "An unexpected Error occurred.".to_owned(),
        }
    }

    pub fn service_unavailable() -> Self {
        Error {
            status: StatusCode::SERVICE_UNAVAILABLE,
            error: "Service Unavailable".to_owned(),
            message: "The server is unable to handle the request at this time."
                .to_owned(),
        }
    }
}

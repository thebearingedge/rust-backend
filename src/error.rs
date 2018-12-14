use crate::app;
use actix_web::{
    error::{Error, InternalError, Result},
    http::StatusCode,
    HttpRequest, HttpResponse,
};
use failure;

#[derive(Serialize)]
struct JsonResponse {
    status: u16,
    error: String,
    message: String,
}

impl JsonResponse {
    fn with_message(status: StatusCode, message: String) -> HttpResponse {
        HttpResponse::build(status).json(JsonResponse {
            message,
            status: status.as_u16(),
            error: status.canonical_reason().unwrap().into(),
        })
    }
}

pub fn bad_request(message: String) -> Error {
    let status = StatusCode::BAD_REQUEST;
    let response = JsonResponse::with_message(status, message.clone());
    InternalError::from_response(message, response).into()
}

pub fn unauthorized(message: String) -> Error {
    let status = StatusCode::UNAUTHORIZED;
    let response = JsonResponse::with_message(status, message.clone());
    InternalError::from_response(message, response).into()
}

pub fn not_found(message: String) -> Error {
    let status = StatusCode::NOT_FOUND;
    let response = JsonResponse::with_message(status, message.clone());
    InternalError::from_response(message, response).into()
}

pub fn bad_implementation(err: failure::Error) -> Error {
    let status = StatusCode::INTERNAL_SERVER_ERROR;
    let response = JsonResponse::with_message(
        status,
        "An unexpected error occurred.".into(),
    );
    InternalError::from_response(err, response).into()
}

pub fn service_unavailable(err: failure::Error) -> Error {
    let status = StatusCode::SERVICE_UNAVAILABLE;
    let response = JsonResponse::with_message(
        status,
        "The server is currently unable to handle the request.".into(),
    );
    InternalError::from_response(err, response).into()
}

pub fn not_found_handler(
    req: &HttpRequest<app::State>,
) -> Result<&'static str> {
    Err(not_found(format!("Cannot {} {}", req.method(), req.path())))
}

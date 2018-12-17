use actix_web::{
    error::{Error, InternalError, Result},
    http::StatusCode,
    middleware::{Middleware, Response},
    HttpRequest, HttpResponse,
};
use failure;

#[derive(Serialize)]
pub struct ErrorBody<'a> {
    status: u16,
    error: &'a str,
    message: &'a str,
}

impl<'a> ErrorBody<'a> {
    fn response(status: StatusCode, message: &str) -> HttpResponse {
        HttpResponse::build(status).json(ErrorBody {
            message,
            status: status.as_u16(),
            error: status.canonical_reason().unwrap(),
        })
    }
}

pub fn bad_request(message: String) -> Error {
    let status = StatusCode::BAD_REQUEST;
    let response = ErrorBody::response(status, &message);

    InternalError::from_response(format!("{} - {}", status, message), response)
        .into()
}

pub fn unauthorized(message: String) -> Error {
    let status = StatusCode::UNAUTHORIZED;
    let response = ErrorBody::response(status, &message);

    InternalError::from_response(format!("{} - {}", status, message), response)
        .into()
}

pub fn internal_server_error<E: Into<failure::Error>>(err: E) -> Error {
    let status = StatusCode::INTERNAL_SERVER_ERROR;

    InternalError::new(err.into(), status).into()
}

pub fn service_unavailable<E: Into<failure::Error>>(err: E) -> Error {
    let status = StatusCode::SERVICE_UNAVAILABLE;
    let response = ErrorBody::response(
        status,
        "The server is currently unable to handle the request.",
    );

    InternalError::from_response(
        format!("{} - {}", status, err.into()),
        response,
    )
    .into()
}

pub fn not_found(message: String) -> Error {
    let status = StatusCode::NOT_FOUND;
    let response = ErrorBody::response(status, &message);

    InternalError::from_response(format!("{} - {}", status, message), response)
        .into()
}

fn bad_implementation(res: HttpResponse) -> HttpResponse {
    let status = res.status();

    res.into_builder().json(ErrorBody {
        status: status.as_u16(),
        error: status.canonical_reason().unwrap().into(),
        message: "An unexpected error occurred.",
    })
}

pub struct ErrorHandler;

impl<S> Middleware<S> for ErrorHandler {
    fn response(
        &self,
        _req: &HttpRequest<S>,
        res: HttpResponse,
    ) -> Result<Response> {
        match res.status() {
            StatusCode::INTERNAL_SERVER_ERROR => {
                Ok(Response::Done(bad_implementation(res)))
            }
            _ => Ok(Response::Done(res)),
        }
    }
}

use actix_web::{
    error::{Error, InternalError, Result},
    http::StatusCode,
    middleware::{Middleware, Response},
    HttpRequest, HttpResponse,
};
use failure;

#[derive(Serialize)]
struct JsonBody {
    status: u16,
    error: String,
    message: String,
}

impl JsonBody {
    fn response(status: StatusCode, message: String) -> HttpResponse {
        HttpResponse::build(status).json(JsonBody {
            message,
            status: status.as_u16(),
            error: status.canonical_reason().unwrap().into(),
        })
    }
}

pub fn bad_request(message: String) -> Error {
    let status = StatusCode::BAD_REQUEST;
    let response = JsonBody::response(status, message.clone());

    InternalError::from_response(format!("{} - {}", status, message), response)
        .into()
}

pub fn unauthorized(message: String) -> Error {
    let status = StatusCode::UNAUTHORIZED;
    let response = JsonBody::response(status, message.clone());

    InternalError::from_response(format!("{} - {}", status, message), response)
        .into()
}

pub fn internal_server_error(err: failure::Error) -> Error {
    let status = StatusCode::INTERNAL_SERVER_ERROR;

    InternalError::new(err, status).into()
}

pub fn service_unavailable(err: failure::Error) -> Error {
    let status = StatusCode::SERVICE_UNAVAILABLE;
    let response = JsonBody::response(
        status,
        "The server is currently unable to handle the request.".into(),
    );

    InternalError::from_response(format!("{} - {}", status, err), response)
        .into()
}

pub fn not_found(message: String) -> Error {
    let status = StatusCode::NOT_FOUND;
    let response = JsonBody::response(status, message.clone());

    InternalError::from_response(format!("{} - {}", status, message), response)
        .into()
}

fn bad_implementation(res: HttpResponse) -> HttpResponse {
    let status = res.status();

    res.into_builder().json(JsonBody {
        status: status.as_u16(),
        error: status.canonical_reason().unwrap().into(),
        message: String::from("An unexpected error occurred."),
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

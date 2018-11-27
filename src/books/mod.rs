use actix_web::{AsyncResponder, Error, HttpRequest, HttpResponse};

use futures::future::{result, Future};

#[derive(Serialize)]
struct Payload {
    message: String,
}

pub fn create(_req: &HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {
    result(Ok(HttpResponse::Created().json(Payload {
        message: String::from("Hello, World!"),
    })))
    .responder()
}

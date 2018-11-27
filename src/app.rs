use actix_web::{http::Method, App};

pub fn create() -> App {
    App::new().resource("/books", |r| r.method(Method::POST).f(crate::books::create))
}

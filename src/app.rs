use actix_web::{http::Method, App};

pub fn create() -> App {
    App::new().resource("/books", |r| r.method(Method::GET).f(crate::books::create))
}

use crate::{auth, db::DbActor, error};
use actix_web::{
    self, actix::Addr, middleware::Logger, App, HttpRequest, Result,
};

pub struct State {
    pub db: Addr<DbActor>,
}

fn not_found_handler<S>(req: &HttpRequest<S>) -> Result<&'static str> {
    let message = format!("Cannot {} {}", req.method(), req.path());
    Err(error::not_found(message))
}

pub fn create(state: State) -> App<State> {
    App::with_state(state)
        .middleware(error::ErrorHandler)
        .middleware(Logger::default())
        .scope("/auth", |scope| {
            scope
                .resource("/sign-up", |r| r.post().with(auth::sign_up))
                .resource("/sign-in", |r| r.post().with(auth::sign_in))
        })
        .default_resource(|r| r.f(not_found_handler))
}

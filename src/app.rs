use crate::{auth, db::DbActor};
use actix_web::{actix::Addr, http::Method, middleware::Logger, App};

pub struct State {
    pub db: Addr<DbActor>,
}

pub fn create(state: State) -> App<State> {
    App::with_state(state).middleware(Logger::default()).scope(
        "/auth",
        |scope| {
            scope
                .resource("/sign-up", |resource| {
                    resource.method(Method::POST).with(auth::sign_up)
                })
                .resource("/sign-in", |resource| {
                    resource.method(Method::POST).with(auth::sign_in)
                })
        },
    )
}

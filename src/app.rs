use crate::auth;
use crate::db::DbActor;
use actix_web::{actix::Addr, http::Method, App};

pub struct State {
    pub db: Addr<DbActor>,
}

pub fn create(state: State) -> App<State> {
    App::with_state(state).scope("/auth", |scope| {
        scope.resource("/sign-up", |resource| {
            resource.method(Method::POST).with(auth::sign_up)
        })
    })
}

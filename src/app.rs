use crate::{auth, db::DbActor, error};
use actix_web::{self, actix::Addr, middleware::Logger, App};

pub struct State {
    pub db: Addr<DbActor>,
}

pub fn create(state: State) -> App<State> {
    App::with_state(state)
        .middleware(Logger::default())
        .scope("/auth", |scope| {
            scope
                .resource("/sign-up", |r| r.post().with(auth::sign_up))
                .resource("/sign-in", |r| r.post().with(auth::sign_in))
        })
        .default_resource(|r| r.route().f(error::not_found_handler))
}

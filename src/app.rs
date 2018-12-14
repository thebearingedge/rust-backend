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
                .resource("/sign-up", |resource| {
                    resource.post().with(auth::sign_up)
                })
                .resource("/sign-in", |resource| {
                    resource.post().with(auth::sign_in)
                })
        })
        .default_resource(|resource| {
            resource.route().f(error::not_found_handler)
        })
}

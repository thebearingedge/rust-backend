use super::users;
use crate::db::DbActor;
use crate::error::Error;
use actix_web::actix::{Handler, Message};

impl Message for users::SignUp {
    type Result = Result<users::CreatedUser, Error>;
}

impl Handler<users::SignUp> for DbActor {
    type Result = Result<users::CreatedUser, Error>;

    fn handle(
        &mut self,
        payload: users::SignUp,
        _: &mut Self::Context,
    ) -> Self::Result {
        self.conn
            .get()
            .map_err(|_| Error::service_unavailable())
            .and_then(|conn| {
                users::create(&conn, payload)
                    .map_err(|_| Error::bad_implementation())
            })
    }
}

impl Message for users::SignIn {
    type Result = Result<users::Claims, Error>;
}

impl Handler<users::SignIn> for DbActor {
    type Result = Result<users::Claims, Error>;

    fn handle(
        &mut self,
        payload: users::SignIn,
        _: &mut Self::Context,
    ) -> Self::Result {
        self.conn
            .get()
            .map_err(|_| Error::service_unavailable())
            .and_then(|conn| {
                users::authenticate(&conn, payload)
                    .map_err(|_| Error::unauthorized("Invalid login."))
            })
    }
}

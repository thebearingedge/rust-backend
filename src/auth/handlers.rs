use super::{models, users};
use crate::db::DbActor;
use crate::error::Error;
use actix_web::actix::{Handler, Message};

impl Message for models::SignUp {
    type Result = Result<models::CreatedUser, Error>;
}

impl Handler<models::SignUp> for DbActor {
    type Result = Result<models::CreatedUser, Error>;

    fn handle(
        &mut self,
        payload: models::SignUp,
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

impl Message for models::SignIn {
    type Result = Result<models::Claims, Error>;
}

impl Handler<models::SignIn> for DbActor {
    type Result = Result<models::Claims, Error>;

    fn handle(
        &mut self,
        payload: models::SignIn,
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

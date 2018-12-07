use super::users;
use crate::{
    db::DbActor,
    error::{self, AppResult},
};
use actix_web::actix::{Handler, Message};

impl Message for users::SignUp {
    type Result = AppResult<users::CreatedUser>;
}

impl Handler<users::SignUp> for DbActor {
    type Result = AppResult<users::CreatedUser>;

    fn handle(
        &mut self,
        payload: users::SignUp,
        _: &mut Self::Context,
    ) -> Self::Result {
        self.conn
            .get()
            .map_err(error::service_unavailable)
            .and_then(|conn| users::create(&conn, payload))
    }
}

impl Message for users::SignIn {
    type Result = AppResult<users::Claims>;
}

impl Handler<users::SignIn> for DbActor {
    type Result = AppResult<users::Claims>;

    fn handle(
        &mut self,
        payload: users::SignIn,
        _: &mut Self::Context,
    ) -> Self::Result {
        self.conn
            .get()
            .map_err(error::service_unavailable)
            .and_then(|conn| users::authenticate(&conn, payload))
    }
}

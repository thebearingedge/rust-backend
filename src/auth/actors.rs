use super::models::{NewUser, User};
use crate::db::DbActor;
use actix_web::{
    actix::{Handler, Message},
    error, Error,
};
use bcrypt;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl Message for CreateUser {
    type Result = Result<User, Error>;
}

impl Handler<CreateUser> for DbActor {
    type Result = Result<User, Error>;

    fn handle(
        &mut self,
        message: CreateUser,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::users::dsl::*;

        let conn = self.0.get().unwrap();
        let id = Uuid::new_v4();
        let hashed_password = bcrypt::hash(&message.password, 10).unwrap();
        let new_user = NewUser {
            user_id: id,
            name: message.name,
            email: message.email,
            password: hashed_password,
        };

        diesel::insert_into(users)
            .values(&new_user)
            .get_result::<User>(&conn)
            .map_err(|_| error::ErrorInternalServerError(""))
    }
}

use super::models::{ActiveUser, AuthenticatedUser, CreatedUser, NewUser};
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
    type Result = Result<CreatedUser, Error>;
}

impl Handler<CreateUser> for DbActor {
    type Result = Result<CreatedUser, Error>;

    fn handle(
        &mut self,
        message: CreateUser,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::users::dsl::*;

        let conn = self.conn.get().unwrap();
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
            .returning((user_id, name, email, created_at, updated_at))
            .get_result::<CreatedUser>(&conn)
            .map_err(error::ErrorInternalServerError)
    }
}

#[derive(Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

impl Message for Credentials {
    type Result = Result<AuthenticatedUser, Error>;
}

impl Handler<Credentials> for DbActor {
    type Result = Result<AuthenticatedUser, Error>;

    fn handle(
        &mut self,
        payload: Credentials,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::db::functions::*;
        use crate::schema::users::dsl::*;

        let conn = self.conn.get().unwrap();

        users
            .select((user_id, email, password))
            .filter(lower(email).eq(&payload.email.to_lowercase()))
            .filter(password.is_not_null())
            .first::<ActiveUser>(&conn)
            .optional()
            .map_err(error::ErrorInternalServerError)?
            .and_then(|user| {
                let unhashed = &payload.password;
                let hashed = &user.password.unwrap();
                let is_valid = bcrypt::verify(unhashed, hashed).unwrap();
                if !is_valid {
                    return None;
                }
                Some(AuthenticatedUser {
                    user_id: user.user_id,
                    email: user.email,
                })
            })
            .ok_or(error::ErrorUnauthorized(""))
    }
}

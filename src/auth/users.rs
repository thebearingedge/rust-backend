use super::models;
use bcrypt;
use diesel::{pg::PgConnection, prelude::*, result::Error};
use uuid::Uuid;

pub fn create(
    conn: &PgConnection,
    payload: models::SignUp,
) -> QueryResult<models::CreatedUser> {
    use crate::schema::users::dsl::*;

    let id = Uuid::new_v4();
    let hashed_password = bcrypt::hash(&payload.password, 10).unwrap();
    let new_user = models::NewUser {
        user_id: id,
        name: payload.name,
        email: payload.email,
        password: hashed_password,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .returning((user_id, name, email, created_at, updated_at))
        .get_result::<models::CreatedUser>(conn)
}

pub fn authenticate(
    conn: &PgConnection,
    payload: models::SignIn,
) -> QueryResult<models::Claims> {
    use crate::db::functions::*;
    use crate::schema::users::dsl::*;

    users
        .select((user_id, email, password))
        .filter(lower(email).eq(&payload.email.to_lowercase()))
        .filter(password.is_not_null())
        .first::<models::ActiveUser>(conn)
        .and_then(|user| {
            let unhashed = &payload.password;
            let hashed = &user.password.unwrap();
            let is_valid = bcrypt::verify(unhashed, hashed).unwrap();
            if !is_valid {
                return Err(Error::NotFound);
            }
            Ok(models::Claims {
                user_id: user.user_id,
                email: user.email,
            })
        })
}

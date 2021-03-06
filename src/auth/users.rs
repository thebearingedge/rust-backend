use crate::{error, schema::users};
use actix_web::error::Result;
use bcrypt;
use chrono::{offset::Utc, DateTime};
use diesel::{pg::PgConnection, prelude::*};
use uuid::Uuid;

#[derive(Serialize, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct CreatedUser {
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Queryable)]
pub struct FoundUser {
    pub user_id: Uuid,
    pub email: String,
    pub password: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    pub user_id: Uuid,
    pub email: String,
}

#[derive(Deserialize)]
pub struct SignIn {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct SignUp {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub fn create(conn: &PgConnection, payload: SignUp) -> Result<CreatedUser> {
    use crate::db::functions::*;
    use crate::schema::users::dsl::*;

    let found = users
        .select((user_id, email, password))
        .filter(lower(email).eq(&payload.email.to_lowercase()))
        .first::<FoundUser>(conn)
        .optional()
        .map_err(error::internal_server_error)?;

    if found.is_some() {
        return Err(error::bad_request(format!(
            "Email '{}' is already in use.",
            payload.email
        )));
    }

    let hashed_password = bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST)
        .map_err(error::internal_server_error)?;
    let new_user = NewUser {
        name: payload.name,
        email: payload.email,
        password: hashed_password,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .returning((user_id, name, email, created_at, updated_at))
        .get_result::<CreatedUser>(conn)
        .map_err(error::internal_server_error)
}

pub fn authenticate(conn: &PgConnection, payload: SignIn) -> Result<Claims> {
    use crate::db::functions::*;
    use crate::schema::users::dsl::*;

    let found = users
        .select((user_id, email, password))
        .filter(lower(email).eq(&payload.email.to_lowercase()))
        .filter(password.is_not_null())
        .first::<FoundUser>(conn)
        .optional()
        .map_err(error::internal_server_error)?;

    if found.is_none() {
        return Err(error::unauthorized("Invalid login.".into()));
    }

    let user = found.unwrap();
    let unhashed = &payload.password;
    let hashed = &user.password.unwrap();
    let is_valid = bcrypt::verify(unhashed, hashed)
        .map_err(error::internal_server_error)?;

    if !is_valid {
        return Err(error::unauthorized("Invalid login.".into()));
    }

    Ok(Claims {
        user_id: user.user_id,
        email: user.email,
    })
}

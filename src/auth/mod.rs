use crate::app;
use crate::error;
use actix_web::{AsyncResponder, FutureResponse, HttpResponse, Json, State};
use futures::future::Future;
use jsonwebtoken as jwt;
use std::env;

mod handlers;
mod users;

pub fn sign_up(
    state: State<app::State>,
    Json(payload): Json<users::SignUp>,
) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(payload)
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Created().json(user)),
            Err(error) => Ok(error.to_response()),
        })
        .from_err()
        .responder()
}

#[derive(Serialize)]
struct Token {
    pub token: String,
}

lazy_static! {
    static ref token_secret: String =
        env::var("TOKEN_SECRET").expect("TOKEN_SECRET not set");
}

fn create_token(claims: users::Claims) -> error::AppResult<Token> {
    let token =
        jwt::encode(&jwt::Header::default(), &claims, &token_secret.as_ref())
            .map_err(|err| error::bad_implementation(err.into()))?;

    Ok(Token { token })
}

pub fn sign_in(
    state: State<app::State>,
    Json(payload): Json<users::SignIn>,
) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(payload)
        .and_then(|res| match res.and_then(create_token) {
            Ok(token) => Ok(HttpResponse::Created().json(token)),
            Err(error) => Ok(error.to_response()),
        })
        .from_err()
        .responder()
}

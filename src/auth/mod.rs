use crate::app;
use actix_web::{AsyncResponder, FutureResponse, HttpResponse, Json, State};
use futures::future::Future;
use jsonwebtoken as jwt;
use std::env;

mod handlers;
mod models;
mod users;

pub fn sign_up(
    state: State<app::State>,
    Json(user): Json<models::SignUp>,
) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(user)
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Created().json(user)),
            Err(error) => Ok(HttpResponse::from_error(error)),
        })
        .from_err()
        .responder()
}

#[derive(Serialize)]
struct Token {
    pub token: String,
}

fn create_token(payload: models::Claims) -> self::Token {
    let token_secret = env::var("TOKEN_SECRET").expect("TOKEN_SECRET not set");
    let token =
        jwt::encode(&jwt::Header::default(), &payload, &token_secret.as_ref())
            .unwrap();
    self::Token { token }
}

pub fn sign_in(
    state: State<app::State>,
    Json(credentials): Json<models::SignIn>,
) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(credentials)
        .and_then(|res| match res.map(create_token) {
            Ok(token) => Ok(HttpResponse::Created().json(token)),
            Err(error) => Ok(HttpResponse::from_error(error)),
        })
        .from_err()
        .responder()
}

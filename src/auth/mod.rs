use crate::app;
use actix_web::{AsyncResponder, FutureResponse, HttpResponse, Json, State};
use futures::future::Future;
use jsonwebtoken as jwt;
use std::env;

mod handlers;
mod models;

pub fn sign_up(
    state: State<app::State>,
    user: Json<handlers::CreateUser>,
) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(user.into_inner())
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Created().json(user)),
            Err(error) => Ok(HttpResponse::from_error(error)),
        })
        .responder()
}

fn create_token(payload: models::Claims) -> models::Token {
    let token_secret = env::var("TOKEN_SECRET").expect("TOKEN_SECRET not set");
    let token =
        jwt::encode(&jwt::Header::default(), &payload, &token_secret.as_ref())
            .unwrap();
    models::Token { token }
}

pub fn sign_in(
    state: State<app::State>,
    credentials: Json<handlers::Credentials>,
) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(credentials.into_inner())
        .from_err()
        .and_then(|res| match res.map(create_token) {
            Ok(token) => Ok(HttpResponse::Created().json(token)),
            Err(error) => Ok(HttpResponse::from_error(error)),
        })
        .responder()
}

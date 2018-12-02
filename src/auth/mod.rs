use crate::app;
use actix_web::{AsyncResponder, FutureResponse, HttpResponse, Json, State};
use futures::future::Future;

mod actors;
mod models;

pub fn sign_up(
    state: State<app::State>,
    user: Json<actors::CreateUser>,
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

pub fn sign_in(
    state: State<app::State>,
    credentials: Json<actors::Credentials>,
) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(credentials.into_inner())
        .from_err()
        .and_then(|res| match res {
            Ok(claims) => Ok(HttpResponse::Created().json(claims)),
            Err(error) => Ok(HttpResponse::from_error(error)),
        })
        .responder()
}

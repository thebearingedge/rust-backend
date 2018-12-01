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
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

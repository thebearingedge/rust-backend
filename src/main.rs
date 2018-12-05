#![allow(proc_macro_derive_resolution_fallback)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use actix_web::{actix::System, server};
use dotenv::dotenv;
use listenfd::ListenFd;
use num_cpus;
use std::env;

mod app;
mod auth;
mod db;
mod schema;

fn main() {
    dotenv().ok();

    let system = System::new("rust-backend");
    let db_actor = db::create();
    let server = server::new(move || {
        app::create(app::State {
            db: db_actor.clone(),
        })
    });
    let port = env::var("PORT").expect("PORT not set");

    match ListenFd::from_env().take_tcp_listener(0).unwrap() {
        Some(listener) => server.listen(listener).start(),
        _ => server.bind(format!("127.0.0.1:{}", port)).unwrap().start(),
    };

    println!("Listening on port {}", port);

    system.run();
}

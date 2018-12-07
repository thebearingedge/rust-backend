#![allow(proc_macro_derive_resolution_fallback)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

use actix_web::{actix::System, server::HttpServer};
use dotenv::dotenv;
use env_logger;
use listenfd::ListenFd;
use std::env;

mod app;
mod auth;
mod db;
mod error;
mod schema;

lazy_static! {
    static ref port: String = env::var("PORT").expect("PORT not set");
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let system = System::new("rust-backend");
    let db_addr = db::get_addr();
    let server = HttpServer::new(move || {
        app::create(app::State {
            db: db_addr.clone(),
        })
    });

    match ListenFd::from_env().take_tcp_listener(0).unwrap() {
        Some(listener) => server.listen(listener).start(),
        _ => server.bind(format!("127.0.0.1:{}", *port)).unwrap().start(),
    };

    println!("Listening on port {}", *port);

    system.run();
}

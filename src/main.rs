use std::env;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

use actix_web::server;
use dotenv::dotenv;
use listenfd::ListenFd;

mod app;
mod books;
mod schema;

fn main() {
    dotenv().ok();

    let mut listen_fd = ListenFd::from_env();
    let server = server::new(|| app::create());
    let port = env::var("PORT").expect("PORT environment variable not set");

    match listen_fd.take_tcp_listener(0).unwrap() {
        Some(address) => server.listen(address).run(),
        _ => server.bind(format!("127.0.0.1:{}", port)).unwrap().run(),
    }
}

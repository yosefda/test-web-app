extern crate actix_web;

use actix_web::{server, App, HttpRequest};

fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

fn main() {
    server::new(
        || App::new()
            .resource("/", |r| r.f(index)))
        .bind("0.0.0.0:3000").expect("Can not bind to 0.0.0.0:3000")
        .run();
}
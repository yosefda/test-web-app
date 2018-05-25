extern crate actix;
extern crate actix_web;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod handlers;
mod todo;

use actix_web::{server, App};

fn main() {
    let sys = actix::System::new("test-web-app-api");

    server::new(
        || App::new()
            .resource("/todo", |r| r.post().with(handlers::new_todo)))
        .bind("0.0.0.0:3000")
        .expect("Can not bind with 0.0.0.0:3000")
        .start();

    let _ = sys.run();
}

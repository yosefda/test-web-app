//#[macro_use] extern crate log;
extern crate env_logger;
extern crate hyper;
extern crate futures;

mod service;
mod router;
mod handler;

use hyper::server::{Http};

use service::WebAppApi;





fn main() {
    env_logger::init();

    let addr = "0.0.0.0:3000".parse().expect("Cannot parse the assigned IP.");
    let server = Http::new()
        .bind(&addr, || Ok(WebAppApi)).expect("Cannot bind with the assigned IP.");
    server.run().expect("Cannot run server.");
}

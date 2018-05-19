use actix_web::{server, HttpRequest};
use actix_web::App as ActixWebApp;

pub struct App {}

impl App {
    pub fn run() {
        server::new(
            || ActixWebApp::new()
                .resource("/", |r| r.f(index)))
            .bind("0.0.0.0:3000").expect("Can not bind to 0.0.0.0:3000")
            .run();
    }
}

fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}


mod controller;

use actix_web::App as ActixWebApp;
use actix_web::server;

use self::controller::Controller;

pub struct App {}

impl App {
    pub fn run() {
        server::new(|| ActixWebApp::new().resource("/", |r| r.h(Controller {})))
            .bind("0.0.0.0:3000")
            .expect("Can not bind to 0.0.0.0:3000")
            .run();
    }
}

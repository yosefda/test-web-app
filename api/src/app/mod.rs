mod controller;

use actix_web::App as ActixWebApp;
use actix_web::{server};

use self::controller::TodoController;

pub struct App {}

impl App {
    pub fn run() {
        server::new(|| ActixWebApp::new()
            .resource("/todo", |r| {
                r.post().h(TodoController {})
            })
            .resource("/todo/{id}", |r| {
                r.put().h(TodoController {});
                r.delete().h(TodoController {});
                r.get().h(TodoController {});
            })
        )
            .bind("0.0.0.0:3000")
            .expect("Can not bind to 0.0.0.0:3000")
            .run();
    }
}

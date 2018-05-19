extern crate actix_web;

mod app;

use app::App;

fn main() {
    App::run();
}

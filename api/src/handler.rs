use hyper::{Request, Response};
use hyper::{StatusCode};

pub fn home(_req: Request) -> Response {
    let mut response = Response::new();

    response.set_body("Test Web App Api");

    response
}

pub fn echo(req: Request) -> Response {
    let mut response = Response::new();

    response.set_body(req.body());

    response
}

pub fn not_found(_req: Request) -> Response {
    let mut response = Response::new();

    response.set_status(StatusCode::NotFound);

    response
}
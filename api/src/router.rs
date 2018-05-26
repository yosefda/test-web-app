use hyper::{Request, Response};
use hyper::{Method};
use handler;

pub fn route(req: Request) -> Response {
    match (req.method(), req.path()) {
        (&Method::Get, "/") => {
            return handler::home(req);
        },
        (&Method::Post, "/echo") => {
            return handler::echo(req);
        },
        _ => {
            return handler::not_found(req);
        },
    }
}
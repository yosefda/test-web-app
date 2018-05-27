use hyper::{Request};
use hyper::{Method};
use handler;
use typedef::{ApiResponse};

pub fn route(req: Request) -> ApiResponse {
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
use futures::future::Future;
use futures::future::ok as future_ok;
use hyper::header::ContentLength;
use hyper::server::{Request, Response, Service};
use hyper::Error as HyperError;
use hyper::{Method, StatusCode};

pub struct WebAppApi;

impl Service for WebAppApi {
    // hyper's server types
    type Request = Request;
    type Response = Response;
    type Error = HyperError;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let mut response = Response::new();

        match (req.method(), req.path()) {
            (&Method::Get, "/") => {
                response.set_body("Test Web App Api");
            },
            (&Method::Post, "/echo") => {}
            _ => {
                response.set_status(StatusCode::NotFound);
            },
        };

        Box::new(future_ok(response))
    }
}
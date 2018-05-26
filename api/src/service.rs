use futures::future::Future;
use futures::future::ok as future_ok;
use hyper::server::{Request, Response, Service};
use hyper::Error as HyperError;
use router::route;

pub struct WebAppApi;

impl Service for WebAppApi {
    // hyper's server types
    type Request = Request;
    type Response = Response;
    type Error = HyperError;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        Box::new(future_ok(route(req)))
    }
}
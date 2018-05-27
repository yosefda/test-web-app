use futures::future::Future;
use futures::future::ok as future_fn_ok;
use hyper::server::{Request, Service};
use hyper::Error as HyperError;
use router::route;
use typedef::{ApiResponse};

pub struct WebAppApi;

impl Service for WebAppApi {
    // hyper's server types
    type Request = Request;
    type Response = ApiResponse;
    type Error = HyperError;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        Box::new(future_fn_ok(route(req)))
    }
}
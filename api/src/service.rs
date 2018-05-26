use futures::future::Future;
use futures::future::ok as future_ok;
use hyper::header::ContentLength;
use hyper::server::{Request, Response, Service};
use hyper::Error as HyperError;

pub struct WebAppApi;

const PHRASE: &'static str = "Hello world!";

impl Service for WebAppApi {
    // hyper's server types
    type Request = Request;
    type Response = Response;
    type Error = HyperError;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, _req: Request) -> Self::Future {
        Box::new(future_ok(
            Response::new()
                .with_header(ContentLength(PHRASE.len() as u64))
                .with_body(PHRASE)
        ))
    }
}
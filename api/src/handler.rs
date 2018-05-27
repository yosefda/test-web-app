use hyper::{Request, StatusCode, Chunk, Body};
use futures::Stream;
use typedef::{ApiResponse};

pub fn home(_req: Request) -> ApiResponse {
    let mut response = ApiResponse::new();

    let body: Box<Stream<Item=_, Error=_>> = Box::new(Body::from("Test Web App Api"));
    response.set_body(body);

    response
}

pub fn echo(req: Request) -> ApiResponse {
    let mut response = ApiResponse::new();

    let mapping = req.body()
        .map( |chunk| {
            let uppered = chunk.iter()
                .map(|byte| byte.to_ascii_uppercase())
                .collect::<Vec<u8>>();
            Chunk::from(uppered)
        });
    let body: Box<Stream<Item=_, Error=_>> = Box::new(mapping);
    response.set_body(body);

    response
}

pub fn not_found(_req: Request) -> ApiResponse {
    let mut response = ApiResponse::new();

    response.set_status(StatusCode::NotFound);

    response
}
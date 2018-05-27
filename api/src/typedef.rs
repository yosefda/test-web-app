use hyper::{Response, Chunk, Error as HyperError};
use futures::Stream;

pub type ApiResponse = Response<Box<Stream<Item=Chunk, Error=HyperError>>>;

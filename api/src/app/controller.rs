use actix_web::{HttpRequest, HttpResponse, dev::Handler};

pub struct Controller {}

impl<T> Handler<T> for Controller {
    type Result = HttpResponse;

    fn handle(&mut self, _req: HttpRequest<T>) -> Self::Result {
        HttpResponse::Ok()
            .content_type("application/text")
            .body("Hello world!")
    }
}

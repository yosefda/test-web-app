use actix_web::{HttpRequest, HttpResponse, dev::Handler, http::Method};

pub struct TodoController {}

impl<T> Handler<T> for TodoController {
    type Result = HttpResponse;

    fn handle(&mut self, req: HttpRequest<T>) -> Self::Result {
        match req.method() {
             &Method::POST => HttpResponse::Ok()
                 .content_type("application/text")
                 .body("Hello world! POST"),
            _ => HttpResponse::Ok()
                .content_type("application/text")
                .body("Hello world! OTHER")
        }
    }
}

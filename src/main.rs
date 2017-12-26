extern crate iron;
extern crate router;

use iron::{Iron, IronResult, Request, Response, status, middleware};
use router::Router;

struct Hello;

impl middleware::Handler for Hello {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello, world!")))
    }
}

fn main() {
    Iron::new(Hello).http("0.0.0.0:3000").unwrap();
}

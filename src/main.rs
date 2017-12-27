extern crate iron;
extern crate router;

use iron::middleware::Chain;
use iron::{Iron, IronResult, Request, Response, status, middleware};
use router::Router;

use iron::{BeforeMiddleware, AfterMiddleware};
use iron::headers::{Authorization, Bearer};
use iron::error::{IronError, HttpError};

struct AuthorizationMiddleware;

impl BeforeMiddleware for AuthorizationMiddleware {

    fn before(&self, req: &mut Request) -> IronResult<()> {
        if let Some(&Authorization(Bearer{ ref token })) = req.headers.get() {
            println!("{}", token);
            if token == "hoge" {
                return Ok(())
            }
        }
        Err(IronError::new(HttpError::Header, (status::Unauthorized)))
    }
}

impl AfterMiddleware for AuthorizationMiddleware {

    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        match err.response.status {
            Some(status::Unauthorized) => {
                Ok(Response::with((status::Unauthorized, "401 unauthorized")))
            },
            _ => Err(err)
        }
    }
}

struct Hello;

impl middleware::Handler for Hello {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello, world!")))
    }
}

fn hoge(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "hoge")))
}

fn main() {

    let mut router = Router::new();
    router.get("/hello", Hello, "hello");
    router.post("/hoge", hoge, "hoge");

    let mut chain = Chain::new(router);
    chain.link((AuthorizationMiddleware, AuthorizationMiddleware));

    Iron::new(chain).http("0.0.0.0:3000").unwrap();
}

use iron::prelude::*;
use iron::status;
use iron::Url;
use iron::modifiers::Redirect;

pub fn handle(_: &mut Request) -> IronResult<Response> {
    let url = Url::parse("http://localhost:3000/").unwrap();
    Ok(Response::with((status::Found, Redirect(url))))
}

use iron::prelude::*;
use iron::status;
use iron::Url;
use iron::modifiers::Redirect;

use middleware::session;

pub fn handle(req: &mut Request) -> IronResult<Response> {
    let url = Url::parse("http://localhost:3000/").unwrap();
    session::get(req).insert("message".to_string(), "This is a message");
    Ok(Response::with((status::Found, Redirect(url))))
}

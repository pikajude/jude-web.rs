use iron::prelude::*;
use iron::status;
use templates::*;
use mustache::MapBuilder;

use middleware::session;

pub fn handle(req: &mut Request) -> IronResult<Response> {
    let mut ctx = MapBuilder::new().insert_str("title", "jude.bio");
    if let Some(msg) = session::get(req).remove::<String>("message".to_string()) {
        ctx = ctx.insert_bool("msg?", true).insert_str("msg", msg);
    }
    let body = template("index.html".to_string(), ctx.build());
    Ok(Response::with((status::Ok, body)))
}

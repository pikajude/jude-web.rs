use iron::prelude::*;
use iron::status;
use templates::*;
use mustache::MapBuilder;

use middleware::session;

pub fn handle(req: &mut Request) -> IronResult<Response> {
    let body = template("index.html".to_string(), MapBuilder::new()
        .insert_str("title", "jude.bio")
        .insert_bool("msg?", true)
        .insert_str("msg", "This is a message")
        .build()
    );
    session::get(req).entry("name".to_string()).or_insert("Jude".to_string());
    Ok(Response::with((status::Ok, body)))
}

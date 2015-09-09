use iron::prelude::*;
use iron::status;
use lib::templates::*;
use mustache::MapBuilder;

pub fn handle(_: &mut Request) -> IronResult<Response> {
    let body = template("index.html".to_string(), MapBuilder::new()
        .insert_str("title", "jude.bio")
        .insert_bool("msg?", true)
        .insert_str("msg", "This is a message")
        .build()
    );
    Ok(Response::with((status::Ok, body)))
}

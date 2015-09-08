#![feature(result_expect)]

extern crate iron;
extern crate mustache;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate env_logger;

use iron::prelude::*;
use iron::status;

use mustache::MapBuilder;

mod templates;
use templates::*;

fn main() {
    env_logger::init().unwrap();
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        let res = template("index.html".to_string(), MapBuilder::new()
            .insert("title", &("jude.bio")).ok().unwrap()
            .insert("hasMessage", &true).ok().unwrap()
            .insert("msg", &("foobar")).ok().unwrap()
            .build());

        Ok(Response::with((status::Ok, res)))
    }

    Iron::new(hello_world).http("localhost:3000").unwrap();
    println!("On 3000")
}

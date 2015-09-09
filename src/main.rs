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

macro_rules! object {
    ( $( $key:expr => $val:expr ),* ) => {{
        MapBuilder::new()
            $(.insert($key, &$val).ok().unwrap())*
            .build()
    }};
}

fn main() {
    env_logger::init().unwrap();
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        let res = template("index.html".to_string(), object! {
            "title" => "jude.bio",
            "hasMessage" => true,
            "msg" => "foobar"
        });

        Ok(Response::with((status::Ok, res)))
    }

    Iron::new(hello_world).http("localhost:3000").unwrap();
    println!("On 3000")
}

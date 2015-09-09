#![feature(result_expect)]

extern crate iron;
extern crate logger;
extern crate mustache;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate router;

use logger::Logger;

#[macro_use]
mod lib;
mod handler;

use iron::prelude::*;
use handler::*;

fn main() {
    let logger = Logger::new(None);

    let router = router!(
        get "/" => index::handle,
        get "/foo" => foo::handle
    );

    let mut chain = Chain::new(router);
    chain.link(logger);

    Iron::new(chain).http("localhost:3000").unwrap();
    println!("On 3000")
}

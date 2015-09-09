#![feature(convert)]
#![feature(result_expect)]
#![feature(vec_push_all)]

extern crate iron;
extern crate logger;
extern crate mustache;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate router;
#[macro_use]
extern crate log;
extern crate sodiumoxide;
extern crate rand;
extern crate rustc_serialize;
extern crate cookie;
extern crate env_logger;

use logger::Logger;

#[macro_use]
mod lib;
mod handler;

use iron::prelude::*;
use handler::*;
use lib::middlewares::session;

fn main() {
    env_logger::init().unwrap();
    let logger = Logger::new(None);

    let router = router!(
        get "/" => index::handle,
        get "/foo" => foo::handle
    );

    let mut chain = Chain::new(router);
    chain.link(logger);

    let sw = session::SessionWare::with_key("client-key.aes");
    chain.link(sw);

    Iron::new(chain).http("localhost:3000").unwrap();
    println!("On 3000")
}

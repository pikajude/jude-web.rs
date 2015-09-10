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
pub mod middleware;
pub mod templates;
pub mod handler;

use iron::prelude::*;
use handler::*;
use middleware::session;

/// Runs the jude_web executable.
pub fn main() {
    env_logger::init().unwrap();
    let logger = Logger::new(None);

    let router = router!(
        get "/" => index::handle,
        get "/foo" => foo::handle
    );

    let mut chain = Chain::new(router);
    chain.link(logger);

    let sw = session::with_key_file("client-key.aes");
    chain.link(sw);

    Iron::new(chain).http("localhost:3000").unwrap();
    println!("On 3000")
}

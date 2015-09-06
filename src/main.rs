#![feature(result_expect)]

extern crate iron;
extern crate mustache;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;

#[derive(RustcEncodable, RustcDecodable)]
struct Person {
    name: String,
}

fn main() {
    fn hello_world(req: &mut Request) -> IronResult<Response> {
        let tmpl = mustache::compile_str("{{name}}");
        let mut v = Vec::new();

        let name = req.url.clone().into_generic_url().query_pairs().and_then(|v|
            v.into_iter().find(|&(ref name, _)| name == "name")
        ).map(|(_,b)|b).unwrap_or(String::from("world"));
        let f = Person { name: name };

        tmpl.render(&mut v, &f).expect("Template rendering failed");

        Ok(Response::with((status::Ok, String::from_utf8(v).unwrap())))
    }

    Iron::new(hello_world).http("localhost:3000").unwrap();
    println!("On 3000")
}

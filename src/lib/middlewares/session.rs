use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::{ErrorKind,Read,Write};

use sodiumoxide::crypto::secretbox;

use cookie::{CookieJar,Cookie as CookiePair};

use iron::headers::{Cookie,SetCookie};
use iron::middleware::{AfterMiddleware,BeforeMiddleware};
use iron::prelude::*;
use iron::typemap::*;

use rustc_serialize::json;
use rustc_serialize::json::DecodeResult;

type SessionMap = HashMap<String, Vec<u8>>;

struct SessionMapType;
struct Value {
    sml: SessionMap
}

impl Key for SessionMapType { type Value = Value; }

pub struct SessionWare { key: Vec<u8> }

impl SessionWare {
    pub fn with_key<P: AsRef<Path>>(key_path: P) -> (SessionWare, SessionWare) {
        let secretbox::Key(key) = SessionWare::read_or_create_key(key_path);
        let mut v = vec![];
        v.push_all(&key);
        let w = v.clone();
        (SessionWare { key: v }, SessionWare { key: w })
    }

    fn read_or_create_key<P: AsRef<Path>>(key_path: P) -> secretbox::Key {
        let kp = key_path.as_ref();
        let key_exists = match fs::metadata(kp) {
            Ok(m) => m.is_file(),
            Err(ref e) if e.kind() == ErrorKind::NotFound => false,
            Err(e) => panic!(e)
        };
        if key_exists {
            let mut f: File = File::open(kp).expect("cannot open key_path");
            let mut s = Vec::new();
            f.read_to_end(&mut s).expect("reading failed");
            secretbox::Key::from_slice(s.as_mut_slice()).expect("key is wrong length")
        } else {
            let mut f: File = File::create(kp).expect("cannot write key_path");
            let key = secretbox::gen_key();
            let secretbox::Key(k) = key;
            f.write_all(&k).expect("could not write key_path");
            key
        }
    }
}

fn decode_session(cookie: String) -> DecodeResult<SessionMap> {
    json::decode(&cookie)
}

impl BeforeMiddleware for SessionWare {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let map = req.headers.get::<Cookie>().and_then(|c| {
            let jar = c.to_cookie_jar(self.key.as_slice());
            jar.encrypted().find("_SESSION").and_then(|c| {
                decode_session(c.value).ok()
            })
        }).unwrap_or(HashMap::new());
        debug!(target: "session::get", "{:?}", map);
        req.extensions.insert::<SessionMapType>(Value { sml: map });
        Ok(())
    }
}

impl AfterMiddleware for SessionWare {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let smap = req.extensions.get::<SessionMapType>().expect("SessionMapType not defined, did you link both sides of SessionWare?");
        let hash = json::encode(&smap.sml).expect("JSON encoding failed");
        let jar = req.headers.get::<Cookie>().map(|c| {
            c.to_cookie_jar(self.key.as_slice())
        }).unwrap_or(CookieJar::new(self.key.as_slice()));
        jar.encrypted().add(CookiePair::new("_SESSION".to_string(), hash));
        debug!(target: "session::set", "{:?}", smap.sml);
        let mut r = res;
        r.headers.set(SetCookie::from_cookie_jar(&jar));
        Ok(r)
    }
}

pub fn set_session(req: &mut Request) {
    let mut smap = req.extensions.get_mut::<SessionMapType>().expect("SessionMapType not defined");
    smap.sml.insert("foo".to_string(), vec![1, 2, 3]);
}

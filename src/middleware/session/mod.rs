use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::{Error,ErrorKind,Read,Write};
use std;
use std::fmt;

use sodiumoxide::crypto::secretbox;

use iron::headers::Cookie;
use iron::middleware::{AfterMiddleware,BeforeMiddleware};
use iron::prelude::*;
use iron::typemap::*;
use iron::status;

use rustc_serialize::json;

mod internal;
pub use self::internal::*;

struct SessionMapType;
struct Value {
    sml: SessionMap
}

impl Key for SessionMapType { type Value = Value; }

struct SessionWareBefore { key: Vec<u8> }
struct SessionWareAfter { key: Vec<u8> }

pub type SessionWarePair = (SessionWareBefore, SessionWareAfter);

static PANIC_STR: &'static str = "Session not initialized. Did you link the SessionWare middleware?";

enum KeyLoadError {
    IoError(Error),
    CorruptedKeyError
}

#[derive(Debug)]
/// Represents failure to serialize the session.
pub enum SessionSaveError {
    /// The `session` request extension isn't present, which means that the SessionWare middleware
    /// has probably not been linked.
    SessionAbsent,
    /// The `SessionMap` can't be serialized to JSON.
    SessionEncodeError(json::EncoderError)
}

use self::SessionSaveError::*;

impl std::fmt::Display for SessionSaveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SessionAbsent => write!(f, "SessionAbsent"),
            SessionEncodeError(j) => write!(f, "SessionEncodeError({})", j)
        }
    }
}

impl std::error::Error for SessionSaveError {
    fn description(&self) -> &str {
        match *self {
            SessionAbsent => "SessionMapType was not found on the request",
            SessionEncodeError(_) => "Serializing the session failed"
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            SessionEncodeError(ref j) => Some(j as &std::error::Error),
            _ => None
        }
    }
}

impl From<Error> for KeyLoadError {
    fn from(e: Error) -> KeyLoadError {
        KeyLoadError::IoError(e)
    }
}

impl From<SessionSaveError> for IronError {
    fn from(e: SessionSaveError) -> IronError {
        IronError::new(e, (status::InternalServerError))
    }
}

fn read_or_create_key<P: AsRef<Path>>(key_path: P) -> Result<secretbox::Key, KeyLoadError> {
    let kp = key_path.as_ref();
    let key_exists = match fs::metadata(kp) {
        Ok(m) => m.is_file(),
        Err(ref e) if e.kind() == ErrorKind::NotFound => false,
        Err(e) => panic!(e)
    };
    if key_exists {
        let mut f: File = try!(File::open(kp));
        let mut s = Vec::new();
        f.read_to_end(&mut s).expect("oh no");
        secretbox::Key::from_slice(s.as_mut_slice()).ok_or(KeyLoadError::CorruptedKeyError)
    } else {
        let mut f: File = try!(File::create(kp));
        let key = secretbox::gen_key();
        let secretbox::Key(k) = key;
        try!(f.write_all(&k));
        Ok(key)
    }
}

impl BeforeMiddleware for SessionWareBefore {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let map = req.headers.get::<Cookie>().and_then(|c| {
            let jar = c.to_cookie_jar(self.key.as_slice());
            jar.encrypted().find("_SESSION").and_then(|c| {
                json::decode(&c.value).ok()
            })
        }).unwrap_or(HashMap::new());
        debug!(target: "session::get", "{:?}", map);
        req.extensions.insert::<SessionMapType>(Value { sml: map });
        Ok(())
    }
}

impl AfterMiddleware for SessionWareAfter {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        use iron::headers::SetCookie;
        use cookie::{CookieJar,Cookie as CookiePair};

        let smap = try!(req.extensions.get::<SessionMapType>().ok_or(SessionSaveError::SessionAbsent));
        let hash = try!(json::encode(&smap.sml).map_err(|e|SessionSaveError::SessionEncodeError(e)));
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

/// Retrieve the current user's `Session`.
pub fn get<'a>(req: &'a mut Request) -> Session<'a> {
    let mut map = req.extensions.get_mut::<SessionMapType>().expect(PANIC_STR);
    Session::new(&mut map.sml)
}

/// Creates and returns a new `SessionWare` pair that uses the given `key_file` to encrypt
/// sessions.
///
/// # Examples:
/// ```
/// use jude_web::middleware::session;
///
/// fn main() {
///     let warePair = session::with_key_file("client-key.aes");
///     let mut chain = Chain::new(warePair);
///     Iron::new(chain).http("localhost:3000").unwrap();
///     println!("On 3000!")
/// }
/// ```
pub fn with_key_file<P: AsRef<Path>>(key_file: P) -> SessionWarePair {
    match read_or_create_key(key_file) {
        Err(e) => panic!(e),
        Ok(secretbox::Key(k)) => {
            let mut v = vec![];
            v.push_all(&k);
            self::with_key(v.as_slice())
        }
    }
}

/// Creates and returns a new `SessionWare` pair that uses the given `key` data to encrypt
/// sessions.
///
/// # Examples:
/// ```
/// use jude_web::middleware::session;
///
/// fn main() {
///     let warePair = session::with_key(b"this is a very secret key");
///     let mut chain = Chain::new(warePair);
///     Iron::new(chain).http("localhost:3000").unwrap();
///     println!("On 3000!")
/// }
/// ```
pub fn with_key<K: AsRef<[u8]>>(key: K) -> SessionWarePair {
    let k = Vec::from(key.as_ref());
    let j = k.clone();
    (SessionWareBefore { key: k },
      SessionWareAfter { key: j })
}

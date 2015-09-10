use rustc_serialize::{Decodable,Encodable,json};
use std::collections::HashMap;
use std::collections::hash_map::{Entry as HEntry,OccupiedEntry,VacantEntry};

macro_rules! decode_trace {
    ($e:expr) => {{
        json::decode($e).map_err(|v|
            debug!(target: "session", "Failed to decode session JSON: {}", v)
        ).ok()
    }}
}

macro_rules! encode_trace {
    ($e:expr) => {{
        json::encode($e).map_err(|v|
            debug!(target: "session", "Failed to encode session JSON: {}", v)
        ).ok()
    }}
}

pub type SessionMap = HashMap<String, String>;

#[derive(PartialEq,Eq)]
/// This class wraps `HashMap` and adds a serialize + deserialize step around most of its methods.
pub struct Session<'a> {
    session: &'a mut SessionMap
}

/// Wraps `HashMap::Entry` for encoding/decoding logic.
pub enum Entry<'a> {
    Occupied(OccupiedEntry<'a, String, String>),
    Vacant(VacantEntry<'a, String, String>)
}

fn coerce<V: Decodable>(x: &String) -> V {
    decode_trace!(x).expect("JSON decoding failed")
}

fn encode<V: Encodable>(s: V) -> String {
    encode_trace!(&s).expect("JSON encoding failed")
}

impl<'a> Entry<'a> {
    pub fn or_insert<V: Decodable + Encodable>(self, default: V) -> V {
        use self::Entry::*;

        match self {
            Occupied(e) => coerce(e.get()),
            Vacant(v) => {
                v.insert(encode(&default));
                default
            }
        }
    }

    pub fn or_insert_with<V: Decodable + Encodable, F: FnOnce() -> V>(self, default: F) -> V {
        use self::Entry::*;

        match self {
            Occupied(e) => coerce(e.get()),
            Vacant(v) => {
                let res = default();
                v.insert(encode(&res));
                res
            }
        }
    }
}

impl<'a> Session<'a> {
    pub fn new(m: &'a mut SessionMap) -> Session<'a> {
        Session {
            session: m
        }
    }

    pub fn insert<V: Encodable>(&'a mut self, key: String, val: V) -> Option<V> {
        self.session.insert(key, encode(&val)).map(|_| val)
    }

    pub fn get<V: Decodable>(&'a mut self, key: String) -> Option<V> {
        self.session.get(&key).and_then(|v| decode_trace!(v))
    }

    pub fn entry(&'a mut self, key: String) -> Entry<'a> {
        match self.session.entry(key) {
            HEntry::Occupied(o) => Entry::Occupied(o),
            HEntry::Vacant(v) => Entry::Vacant(v)
        }
    }

    pub fn remove<V: Decodable>(&'a mut self, key: String) -> Option<V> {
        self.session.remove(&key).map(|ref v|coerce(v))
    }

    /// Like `remove`, but discards the return value so you don't need to specify a type parameter.
    pub fn remove_(&'a mut self, key: String) {
        self.session.remove(&key);
    }

    pub fn clear(&'a mut self) {
        self.session.clear();
    }
}

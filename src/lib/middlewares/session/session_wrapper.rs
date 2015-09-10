use rustc_serialize::{Decodable,Encodable,json};
use std::collections::HashMap;
use std::collections::hash_map::{Entry,OccupiedEntry,VacantEntry};

pub type SessionMap = HashMap<String, Vec<u8>>;

pub struct SessionWrapper<'a> {
    session: &'a mut SessionMap
}

pub enum EntryWrapper<'a> {
    Occupied(OccupiedEntry<'a, String, Vec<u8>>),
    Vacant(VacantEntry<'a, String, Vec<u8>>)
}

fn coerce<V: Decodable>(x: &Vec<u8>) -> V {
    json::decode(String::from_utf8(x.to_owned()).expect("val is not UTF8").as_str()).expect("JSON decoding failed")
}

fn encode<V: Encodable>(s: V) -> Vec<u8> {
    json::encode(&s).expect("JSON encoding failed").into_bytes()
}

impl<'a> EntryWrapper<'a> {
    pub fn or_insert<V: Decodable + Encodable>(self, default: V) -> V {
        use self::EntryWrapper::*;

        match self {
            Occupied(e) => coerce(e.get()),
            Vacant(v) => {
                v.insert(encode(&default));
                default
            }
        }
    }

    pub fn or_insert_with<V: Decodable + Encodable, F: FnOnce() -> V>(self, default: F) -> V {
        use self::EntryWrapper::*;

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

impl<'a> SessionWrapper<'a> {
    pub fn new(m: &'a mut SessionMap) -> SessionWrapper<'a> {
        SessionWrapper {
            session: m
        }
    }

    pub fn insert<V: Encodable>(&'a mut self, key: String, val: V) -> Option<V> {
        self.session.insert(key, encode(&val)).map(|_| val)
    }

    pub fn get<V: Decodable>(&'a mut self, key: String) -> Option<V> {
        self.session.get(&key).and_then(|v|
            json::decode(String::from_utf8(v.to_owned()).expect("val is not UTF8").as_str()).ok()
        )
    }

    pub fn entry(&'a mut self, key: String) -> EntryWrapper<'a> {
        match self.session.entry(key) {
            Entry::Occupied(o) => EntryWrapper::Occupied(o),
            Entry::Vacant(v) => EntryWrapper::Vacant(v)
        }
    }
}

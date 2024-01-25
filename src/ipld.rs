use alloc::{collections::BTreeMap, fmt, string::String, vec::Vec};

use cid::Cid;

#[derive(Clone, PartialEq)]
pub enum Ipld {
    Null,
    Bool(bool),
    Integer(i128),
    Float(f64),
    String(String),
    Bytes(Vec<u8>),
    List(Vec<Ipld>),
    Map(BTreeMap<String, Ipld>),
    Link(Cid),
}

impl fmt::Debug for Ipld {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            match self {
                Self::Null => write!(f, "Null"),
                Self::Bool(boolean) => write!(f, "Bool({:?})", boolean),
                Self::Integer(integer) => write!(f, "Integer({:?})", integer),
                Self::Float(float) => write!(f, "Float({:?})", float),
                Self::String(string) => write!(f, "String({:?})", string),
                Self::Bytes(byted) => write!(f, "Bytes({:?})", byted),
                Self::List(list) => write!(f, "List({:#?})", list),
                Self::Map(map) => write!(f, "Map({:#?})", map),
                Self::Link(cid) => write!(f, "Link({})", cid),
            }
        } else {
            match self {
                Self::Null => write!(f, "Null"),
                Self::Bool(boolean) => write!(f, "{:?}", boolean),
                Self::Integer(integer) => write!(f, "{:?}", integer),
                Self::Float(float) => write!(f, "{:?}", float),
                Self::String(string) => write!(f, "{:?}", string),
                Self::Bytes(byted) => write!(f, "{:?}", byted),
                Self::List(list) => write!(f, "{:#?}", list),
                Self::Map(map) => write!(f, "{:#?}", map),
                Self::Link(cid) => write!(f, "{}", cid),
            }
        }
    }
}

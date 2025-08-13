use std::{collections::BTreeMap, ops::{Deref, DerefMut}};
use enum_dispatch::enum_dispatch;
use super::encode::RespEncode;

#[enum_dispatch(RespEncode)]
#[derive(Debug, PartialEq, PartialOrd)]
pub enum RespFrameEnum {
    SimpleString(SimpleString),
    Error(SimpleError),
    Integer(i64),
    BulkString(BulkString),
    Array(RespArray),
    Boolean(bool),
    Double(f64),
    Map(RespMap),
    Set(RespSet),

    Null(RespNull),
    NullArray(RespArrayNull),
    NullBulkString(RespBulkNull),
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct SimpleString(pub String);
impl SimpleString {
    pub fn get_inner(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct BulkString(pub Vec<u8>);

#[derive(Debug, PartialEq, PartialOrd)]
pub struct SimpleError(pub String);

#[derive(Debug, PartialEq, PartialOrd)]
pub struct RespArray(pub Vec<RespFrameEnum>);

#[derive(Debug, PartialEq, PartialOrd)]
pub struct RespMap(pub BTreeMap<String, RespFrameEnum>);

#[derive(Debug, PartialEq, PartialOrd)]
pub struct RespSet(pub Vec<RespFrameEnum>);

#[derive(Debug, PartialEq, PartialOrd)]
pub struct RespNull;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct RespArrayNull;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct RespBulkNull;



impl Deref for SimpleString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for BulkString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for SimpleError {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for RespArray {
    type Target = Vec<RespFrameEnum>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for RespMap {
    type Target = BTreeMap<String, RespFrameEnum>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }

}

impl DerefMut for RespMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for RespSet {
    type Target = Vec<RespFrameEnum>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SimpleString {
    pub fn new(s: impl Into<String>) -> Self {
        SimpleString(s.into())
    }
}

impl SimpleError {
    pub fn new(s: impl Into<String>) -> Self {
        SimpleError(s.into())
    }
}

impl BulkString {
    pub fn new(s: impl Into<Vec<u8>>) -> Self {
        BulkString(s.into())
    }
}

impl RespArray {
    pub fn new(s: impl Into<Vec<RespFrameEnum>>) -> Self {
        RespArray(s.into())
    }
}

impl RespMap {
    pub fn new() -> Self {
        RespMap(BTreeMap::new())
    }
}

impl Default for RespMap {
    fn default() -> Self {
        RespMap::new()
    }
}

impl RespSet {
    pub fn new(s: impl Into<Vec<RespFrameEnum>>) -> Self {
        RespSet(s.into())
    }
}

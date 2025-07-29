use bytes::BytesMut;
use std::ops::Deref;
use tracing_subscriber::fmt::format;

pub enum RespFrameEnum {
    SimpleString(SimpleString),
    Error(SimpleError),
    Integer(i64),
    BulkString(BulkString),
    Array(Vec<RespFrameEnum>),
    Boolean(bool),
    Double(f64),
    BigNumber(i64),
    Map(HashMap<(String, RespFrameEnum)>),
    Set(HashSet<RespFrameEnum>),
    Null(RespNull),
    NullArray(RespArrayNull),
    NullBulkString(RespBulkNull),
}

pub struct SimpleString(String);
pub struct BulkString(Vec<u8>);

pub struct SimpleError(String);

pub struct RespNull;
pub struct RespArrayNull;
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

impl Deref for RespNull {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        "null"
    }
}

impl Deref for RespArrayNull {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        "*null"
    }
}

impl Deref for RespBulkNull {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        "$null"
    }
}

pub trait RespEncode {
    fn encode(&self) -> Vec<u8>;
}

// - integer ":[<+|->]<value>\r\n"
impl RespEncode for i64 {
    fn encode(&self) -> Vec<u8> {
        let sign = if self < 0 { "-" } else { "" };
        format!(":{}{}\r\n", sign, self).into_bytes()
    }
}

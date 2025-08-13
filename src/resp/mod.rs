mod decode;
mod encode;
mod enums;

const BUF_CAP: usize = 4096;
const CRLF: &[u8] = b"\r\n";
const CRLF_LEN: usize = CRLF.len();

pub use enums::*;
pub use encode::*;
pub use decode::*;
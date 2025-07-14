use base64::prelude::*;

use crate::{cli::Base64Format, utils::read_input};

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let buf = read_input(input)?;

    let encoded = match format {
        Base64Format::Standard => BASE64_STANDARD.encode(buf),
        // BASE64_URL_SAFE 使用 = 作为填充字符，当编码数据长度不是 3 的倍数时，使用 = 填充编码结果
        Base64Format::Uri => BASE64_URL_SAFE_NO_PAD.encode(buf),
    };

    Ok(encoded)
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<Vec<u8>> {
    let buf = read_input(input)?;

    // 去除可能的空白字符和换行符
    let decoded_str = String::from_utf8_lossy(&buf).trim().to_string();

    let decoded = match format {
        Base64Format::Standard => BASE64_STANDARD.decode(decoded_str)?,
        Base64Format::Uri => BASE64_URL_SAFE_NO_PAD.decode(decoded_str)?,
    };
    Ok(decoded)
}

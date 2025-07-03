use std::{fs::File, io::Read};

use base64::prelude::*;

use crate::cli::Base64Format;

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let buf = read_input(input)?;

    match format {
        Base64Format::Standard => println!("{}", BASE64_STANDARD.encode(buf)),
        // BASE64_URL_SAFE 使用 = 作为填充字符，当编码数据长度不是 3 的倍数时，使用 = 填充编码结果
        Base64Format::Uri => println!("{}", BASE64_URL_SAFE_NO_PAD.encode(buf)),
    }

    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let buf = read_input(input)?;

    // 去除可能的空白字符和换行符
    let decoded_str = String::from_utf8_lossy(&buf).trim().to_string();

    match format {
        Base64Format::Standard => println!(
            "{}",
            String::from_utf8_lossy(BASE64_STANDARD.decode(decoded_str)?.as_slice())
        ),
        Base64Format::Uri => println!(
            "{}",
            String::from_utf8_lossy(BASE64_URL_SAFE_NO_PAD.decode(decoded_str)?.as_slice())
        ),
    }
    Ok(())
}

fn read_input(input: &str) -> anyhow::Result<Vec<u8>> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}

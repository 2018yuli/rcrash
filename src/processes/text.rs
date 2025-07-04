use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};

use crate::{cli::TextSignFormat, utils::read_input};

struct Blake3 {
    key: [u8; 32],
}

struct Ed25519 {
    key: [u8; 32],
}

pub fn process_sign(input: &str, key: &str, format: TextSignFormat) -> anyhow::Result<()> {
    let buf = read_input(input)?;

    let signed = match format {
        TextSignFormat::Blake3 => todo!("Implement blake3 sign"),
        TextSignFormat::Ed25519 => todo!("Implement ed25519 sign"),
    };

    let signed = BASE64_URL_SAFE_NO_PAD.encode(signed);

    println!("{signed}");

    Ok(())
}

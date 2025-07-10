use std::fs;

use anyhow::Ok;
use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};

use super::text_sign_verify_trait::TextSign;
use crate::{
    cli::TextSignFormat,
    processes::text_sign_verify_trait::TextVerify,
    utils::{get_reader, read_input},
};

struct Blake3 {
    // u8 代表一个字节（byte），有 8 位二进制数据
    key: [u8; 32],
}

struct Ed25519Signer {
    key: [u8; 32],
}
struct Ed25519Verifier {
    key: [u8; 32],
}

pub fn process_sign<'a>(
    input: &'a str,
    key: &'a str,
    format: TextSignFormat,
) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;

    let signed = match format {
        TextSignFormat::Blake3 => {
            let key = fs::read(key)?;
            let key = key.try_into().unwrap();
            let singer = Blake3 { key };
            singer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => todo!(),
    };

    let signed = BASE64_URL_SAFE_NO_PAD.encode(signed);

    println!("{signed}");

    Ok(())
}

pub fn process_verify<'a>(
    input: &'a str,
    signature: &'a str,
    format: TextSignFormat,
) -> anyhow::Result<bool> {
    let input = BASE64_URL_SAFE_NO_PAD.decode(input)?;
    let mut reader = get_reader(std::str::from_utf8(&input).unwrap())?;

    match format {
        TextSignFormat::Blake3 => {
            let signature = fs::read(signature)?;
            let signature = signature.try_into().unwrap();
            let verifier = Blake3 { key: signature };
            verifier.verify(&mut reader, &input)
        }
        TextSignFormat::Ed25519 => todo!(),
    }
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &dyn std::io::Read) -> anyhow::Result<Vec<u8>> {
        todo!()
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, mut reader: impl std::io::Read, signature: &[u8]) -> anyhow::Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::hash(&buf);
        let hash = hash.as_bytes();
        Ok(hash == signature)
    }
}

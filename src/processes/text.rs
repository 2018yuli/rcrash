use std::fs;

use anyhow::Ok;
use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use ed25519_dalek::{
    ed25519::signature::{self, SignerMut},
    Signature, SigningKey, VerifyingKey,
};

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
    key: SigningKey,
}
struct Ed25519Verifier {
    key: VerifyingKey,
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
            let key = &key[..32];
            let key = key.try_into().unwrap();
            let mut singer = Blake3 { key };
            singer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            let key = read_input(key)?;
            let key = SigningKey::from_bytes(key.as_slice().try_into()?);
            let mut singer = Ed25519Signer { key };
            singer.sign(&mut reader)?
        }
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
        TextSignFormat::Ed25519 => {
            let signature = fs::read(signature)?;
            let key = SigningKey::from_bytes(signature.as_slice().try_into()?);
            let verifier = Ed25519Verifier {
                key: key.verifying_key(),
            };
            verifier.verify(&mut reader, &input)
        }
    }
}

impl TextSign for Blake3 {
    fn sign(&mut self, mut reader: impl std::io::Read) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &mut buf).as_bytes().to_vec())
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&mut self, mut reader: impl std::io::Read) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = self.key.sign(&buf);
        Ok(sig.to_bytes().to_vec())
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

impl TextVerify for Ed25519Verifier {
    fn verify(&self, mut reader: impl std::io::Read, signature: &[u8]) -> anyhow::Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let signature = Signature::from_bytes(signature.try_into()?);
        let ret = self.key.verify_strict(&buf, &signature).is_ok();
        Ok(ret)
    }
}

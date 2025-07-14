use std::{fs, path::Path, vec};

use super::text_sign_verify_trait::{KeyLoader, TextSign, TextVerify};
use crate::{
    cli::TextSignFormat, process_generate_password,
    processes::text_sign_verify_trait::KeyGenerator, utils::get_reader,
};
use anyhow::Ok;
use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use ed25519_dalek::{ed25519::signature::SignerMut, Signature, SigningKey, VerifyingKey};
use rand::rngs::OsRng;

pub struct Blake3 {
    // u8 代表一个字节（byte），有 8 位二进制数据
    pub key: [u8; 32],
}

pub struct Ed25519Signer {
    pub key: SigningKey,
}
pub struct Ed25519Verifier {
    pub key: VerifyingKey,
}

pub fn process_sign<'a>(
    input: &'a str,
    key: &'a str,
    format: TextSignFormat,
) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;

    let signed = match format {
        TextSignFormat::Blake3 => {
            let mut singer = Blake3::load(key)?;
            singer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            let mut singer = Ed25519Signer::load(key)?;
            singer.sign(&mut reader)?
        }
    };

    let signed = BASE64_URL_SAFE_NO_PAD.encode(signed);

    println!("{signed}");

    Ok(())
}

pub fn process_verify<'a>(
    input: &'a str,
    key: &'a str,
    signature: &'a str,
    format: TextSignFormat,
) -> anyhow::Result<bool> {
    // let input = BASE64_URL_SAFE_NO_PAD.decode(input)?;
    let mut reader = get_reader(std::str::from_utf8(input.as_bytes()).unwrap())?;

    match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(key)?;
            verifier.verify(&mut reader, signature.as_bytes())
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(key)?;
            verifier.verify(&mut reader, signature.as_bytes())
        }
    }
}

pub fn process_generate_key(output: &str, format: TextSignFormat) -> anyhow::Result<()> {
    let key = match format {
        TextSignFormat::Blake3 => Blake3::generate()?,
        TextSignFormat::Ed25519 => Ed25519Signer::generate()?,
    };

    fs::write(output, &key[0])?;
    Ok(())
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }
    pub fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        let signature = Blake3::new(key);
        Ok(signature)
    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let key = fs::read(path)?;
        Blake3::try_new(&key)
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>> {
        let key = process_generate_password(32, true, true, true, true)?;
        let key = key.try_into()?;
        Ok(vec![key])
    }
}

impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }
    pub fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key = SigningKey::from_bytes(key.try_into()?);
        Ok(Ed25519Signer::new(key))
    }
}

impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let key = fs::read(path)?;
        Ed25519Signer::try_new(&key)
    }
}

impl KeyGenerator for Ed25519Signer {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>> {
        let mut _csprng = OsRng;
        // let sk: SigningKey = SigningKey::generate(&mut csprng);
        // let sk = sk.to_bytes().to_vec();
        Ok(vec![[0u8; 32].to_vec()])
    }
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }
    pub fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key = VerifyingKey::from_bytes(key.try_into()?)?;
        Ok(Ed25519Verifier::new(key))
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let key = fs::read(path)?;
        Ed25519Verifier::try_new(&key)
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
        let hash = blake3::keyed_hash(&self.key, &buf);
        let hash = hash.as_bytes();
        let signed = BASE64_URL_SAFE_NO_PAD.encode(hash);
        Ok(signed.as_bytes() == signature)
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

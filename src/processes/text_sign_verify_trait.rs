use std::{io::Read, path::Path};

pub trait TextSign {
    /// Sign the data from the reader and return the signature.
    // fn sign(&self, reader: &dyn Read) -> anyhow::Result<Vec<u8>>;
    fn sign(&mut self, reader: impl Read) -> anyhow::Result<Vec<u8>>;
}

pub trait TextVerify {
    // fn verify<R: Read>(&self, reader: R, signature: &[u8]) -> anyhow::Result<bool>;
    fn verify(&self, reader: impl Read, signature: &[u8]) -> anyhow::Result<bool>;
}

pub trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self>
    where
        Self: Sized;
}

pub trait KeyGenerator {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>>
    where
        Self: Sized;
}

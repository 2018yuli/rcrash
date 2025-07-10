use std::io::Read;

pub trait TextSign {
    /// Sign the data from the reader and return the signature.
    fn sign(&self, reader: &dyn Read) -> anyhow::Result<Vec<u8>>;
}

pub trait TextVerify {
    // fn verify<R: Read>(&self, reader: R, signature: &[u8]) -> anyhow::Result<bool>;
    fn verify(&self, reader: impl Read, signature: &[u8]) -> anyhow::Result<bool>;
}

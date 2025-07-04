trait TextSign {
    fn sign(&self, data: &str) -> anyhow::Result<Vec<u8>>;
}

trait TextVerify {
    fn verify(&self, data: &str, signature: &[u8]) -> anyhow::Result<bool>;
}

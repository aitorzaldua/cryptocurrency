pub trait hashable {
    fn bytes (&self) -> Vec<u8>;

    fn hash (&self) -> Vec<u8>{
        cryto_hash::digest(crypto_hash::Algorith::SHA256, &self.bytes())
    }
}
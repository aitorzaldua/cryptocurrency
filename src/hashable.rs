use super::*; //para poder usar el tipo Hash

pub trait Hashable {
    fn bytes (&self) -> Vec<u8>;

    fn hash (&self) -> Hash{
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }
}
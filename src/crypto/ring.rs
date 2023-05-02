use crate::Error;

#[inline(always)]
pub fn rsa_pkcs1_sha256(message: &str, private_pem: &[u8]) -> Result<Vec<u8>, Error> {
    use ring::{rand::SystemRandom, signature::{RsaKeyPair, RSA_PKCS1_SHA256},
    };

    let key_pem = pem::parse(private_pem)?;
    let key = RsaKeyPair::from_pkcs8(&key_pem.contents())?;
    let rng = SystemRandom::new();
    let mut signature = vec![0; key.public_modulus_len()];
    key.sign(&RSA_PKCS1_SHA256, &rng, message.as_bytes(), &mut signature)?;
    Ok(signature)
}

#[inline(always)]
pub fn sha256(bytes: &[u8]) -> impl AsRef<[u8]> {
    use ring::digest::{digest, SHA256};
    digest(&SHA256, bytes)
}
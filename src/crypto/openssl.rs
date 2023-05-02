use crate::Error;

#[inline(always)]
pub fn rsa_pkcs1_sha256(message: &str, private_pem: &[u8]) -> Result<Vec<u8>, Error> {
    use openssl::{hash::MessageDigest, pkey::PKey, sign::Signer};

    let key = PKey::private_key_from_pem(private_pem)?;
    let mut signer = Signer::new(MessageDigest::sha256(), &key)?;
    signer.update(message.as_bytes())?;
    Ok(signer.sign_to_vec()?)
}

#[inline(always)]
pub fn sha256(bytes: &[u8]) -> impl AsRef<[u8]> {
    openssl::sha::sha256(bytes)
}
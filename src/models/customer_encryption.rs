/// Contains data about how a user might encrypt their files in Google Cloud Storage.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerEncrypton {
    /// The encryption algorithm.
    pub encryption_algorithm: String,
    /// SHA256 hash value of the encryption key.
    pub key_sha256: String,
}
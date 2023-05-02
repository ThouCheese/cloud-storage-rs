/// Contains information about the encryption used for data in this Bucket.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Encryption {
    /// A Cloud KMS key that will be used to encrypt objects inserted into this bucket, if no
    /// encryption method is specified.
    pub default_kms_key_name: String,
}
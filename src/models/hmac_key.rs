use super::HmacMeta;

/// The `HmacKey` resource represents an HMAC key within Cloud Storage. The resource consists of a
/// secret and `HmacMeta`. HMAC keys can be used as credentials for service accounts. For more
/// information, see HMAC Keys.
///
/// Note that the `HmacKey` resource is only returned when you use `HmacKey::create`. Other
/// methods, such as `HmacKey::read`, return the metadata portion of the HMAC key resource.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HmacKey {
    /// The kind of item this is. For HMAC keys, this is always `storage#hmacKey`.
    pub kind: String,
    /// HMAC key metadata.
    pub metadata: HmacMeta,
    /// HMAC secret key material.
    pub secret: String,
}
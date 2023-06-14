use super::HmacState;

/// Contains information about an Hmac Key.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HmacMeta {
    /// The kind of item this is. For HMAC key metadata, this is always `storage#hmacKeyMetadata`.
    pub kind: String,
    /// The ID of the HMAC key, including the Project ID and the Access ID.
    pub id: String,
    /// The link to this resource.
    pub self_link: String,
    /// The access ID of the HMAC Key.
    pub access_id: String,
    /// The Project ID of the project that owns the service account to which the key authenticates.
    pub project_id: String,
    /// The email address of the key's associated service account.
    pub service_account_email: String,
    /// The state of the key.
    pub state: HmacState,
    /// The creation time of the HMAC key.
    #[serde(with = "time::serde::rfc3339")]
    pub time_created: time::OffsetDateTime,
    /// The last modification time of the HMAC key metadata.
    #[serde(with = "time::serde::rfc3339")]
    pub updated: time::OffsetDateTime,
    /// HTTP 1.1 Entity tag for the HMAC key.
    pub etag: String,
}
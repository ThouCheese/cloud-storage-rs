/// The state of an Hmac Key.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HmacState {
    /// This Hmac key is currently used.
    Active,
    /// This Hmac key has been set to inactive.
    Inactive,
    /// This Hmac key has been permanently deleted.
    Deleted,
}
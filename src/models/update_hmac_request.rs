use super::update_hmac_metadata::UpdateHmacMetadata;

#[derive(serde::Serialize)]
pub(crate) struct UpdateHmacRequest {
    secret: String,
    metadata: UpdateHmacMetadata,
}
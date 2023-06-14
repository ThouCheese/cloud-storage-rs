use super::HmacState;

#[derive(serde::Serialize)]
pub(crate) struct UpdateHmacMetadata {
    pub(crate) state: HmacState,
}
use super::Object;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub(crate) struct RewriteResponse {
    kind: String,
    total_bytes_rewritten: String,
    object_size: String,
    done: bool,
    pub(crate) resource: Object,
}
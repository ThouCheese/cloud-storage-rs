use super::{SourceObject, Object};

/// The request that is supplied to perform `Object::compose`.
#[derive(Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComposeRequest {
    /// The kind of item this is. Will always be `storage#composeRequest`.
    pub kind: String,
    /// The list of source objects that will be concatenated into a single object.
    pub source_objects: Vec<SourceObject>,
    /// Properties of the resulting object.
    pub destination: Option<Object>,
}
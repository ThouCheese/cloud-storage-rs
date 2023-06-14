use super::ObjectPrecondition;

/// A SourceObject represents one of the objects that is to be composed.
#[derive(Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceObject {
    /// The source object's name. All source objects must have the same storage class and reside in
    /// the same bucket.
    pub name: String,
    /// The generation of this object to use as the source.
    pub generation: Option<i64>,
    /// Conditions that must be met for this operation to execute.
    pub object_preconditions: Option<ObjectPrecondition>,
}
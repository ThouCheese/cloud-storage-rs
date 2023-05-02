/// Allows conditional copying of this file.
#[derive(Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectPrecondition {
    /// Only perform the composition if the generation of the source object that would be used
    /// matches this value. If this value and a generation are both specified, they must be the same
    /// value or the call will fail.
    pub if_generation_match: i64,
}
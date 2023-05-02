/// Type of the action.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ActionType {
    /// Deletes a Bucket.
    Delete,
    /// Sets the `storage_class` of a Bucket.
    SetStorageClass,
}
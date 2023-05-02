use super::{StorageClass, ActionType};

/// Represents an action that might be undertaken due to a `Condition`.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    /// Type of the action.
    pub r#type: ActionType,
    /// Target storage class. Required iff the type of the action is SetStorageClass.
    pub storage_class: Option<StorageClass>,
}
use super::{Action, Condition};

/// An element of the lifecyle list.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    /// The action to take.
    pub action: Action,
    /// The condition(s) under which the action will be taken.
    pub condition: Condition,
}
use super::Rule;

/// Contains a set of `Rule` Objects which together describe the way this lifecycle behaves
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lifecycle {
    /// A lifecycle management rule, which is made of an action to take and the condition(s) under
    /// which the action will be taken.
    pub rule: Vec<Rule>,
}
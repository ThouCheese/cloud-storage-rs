/// A condition object associated with a binding.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IamCondition {
    /// Title of the condition. For example, "expires_end_of_2018".
    pub title: String,
    /// Optional description of the condition. For example, "Expires at midnight on 2018-12-31".
    pub description: Option<String>,
    /// [Attribute-based](https://cloud.google.com/iam/docs/conditions-overview#attributes) logic
    /// expression using a subset of the Common Expression Language (CEL). For example,
    /// "request.time < timestamp('2019-01-01T00:00:00Z')".
    pub expression: String,
}
use super::Binding;

/// A representation of the IAM Policiy for a certain bucket.
#[derive(Debug, PartialEq, Default, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IamPolicy {
    /// The [Cloud IAM policy](https://cloud.google.com/iam/docs/policies#versions) version.
    pub version: i32,
    /// The kind of item this is. For policies, this field is ignored in a request and is
    /// `storage#policy` in a response.
    pub kind: Option<String>,
    /// The ID of the resource to which this policy belongs. The response for this field is of the
    /// form `projects/_/buckets/bucket`. This field is ignored in a request.
    pub resource_id: Option<String>,
    /// A list of the bindings for this policy.
    pub bindings: Vec<Binding>,
    /// HTTP 1.1 [Entity tag](https://tools.ietf.org/html/rfc7232#section-2.3) for this policy.
    pub etag: String,
}
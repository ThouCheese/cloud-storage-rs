/// The following enum contains primitive roles and the Cloud Storage permissions that these roles
/// contain. Primitive roles cannot be added at the bucket-level.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum PrimitiveIamRole {
    /// Grants permission to list buckets as well as view bucket metadata, excluding ACLs, when
    /// listing. Also grants permission to list and get HMAC keys in the project.
    #[serde(rename = "role/viewer")]
    Viewer,
    /// Grants permission to create, list, and delete buckets. Grants permission to view bucket
    /// metadata, excluding ACLs, when listing. Grants full control over HMAC keys in a project.
    #[serde(rename = "role/editor")]
    Editor,
    /// Grants permission to create, list, and delete buckets. Also grants permission to view bucket
    /// metadata, excluding ACLs, when listing. Grants full control over HMAC keys in a project.
    #[serde(rename = "role/owner")]
    Owner,
}
/// The request needed to perform the Object::test_iam_permission function.
#[derive(Debug, PartialEq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestIamPermission {
    /// The kind of item this is.
    kind: String,
    /// The permissions held by the caller. Permissions are always of the format
    /// `storage.resource.capability`, where resource is one of buckets or objects. See
    /// [Cloud Storage IAM Permissions]
    /// (https://cloud.google.com/storage/docs/access-control/iam-permissions) for a list of
    /// supported permissions.
    permissions: Vec<String>,
}
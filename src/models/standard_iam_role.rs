/// The following enum contains Cloud Identity and Access Management (Cloud IAM) roles that are
/// associated with Cloud Storage and lists the permissions that are contained in each role. Unless
/// otherwise noted, these roles can be applied either to entire projects or specific buckets.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum StandardIamRole {
    /// Allows users to create objects. Does not give permission to view, delete, or overwrite
    /// objects.
    #[serde(rename = "roles/storage.objectCreator")]
    ObjectCreator,
    /// Grants access to view objects and their metadata, excluding ACLs.
    ///
    /// Can also list the objects in a bucket.
    #[serde(rename = "roles/storage.objectViewer")]
    ObjectViewer,
    /// Grants full control over objects, including listing, creating, viewing, and deleting
    /// objects.
    #[serde(rename = "roles/storage.objectAdmin")]
    ObjectAdmin,
    /// Full control over HMAC keys in a project.
    #[serde(rename = "roles/storage.hmacKeyAdmin")]
    HmacKeyAdmin,
    /// Grants full control of buckets and objects.
    ///
    /// When applied to an individual bucket, control applies only to the specified bucket and
    /// objects within the bucket.
    #[serde(rename = "roles/storage.admin")]
    Admin,
}
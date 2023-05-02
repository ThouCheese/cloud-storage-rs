/// The following enum contains Cloud IAM roles that are equivalent to Access Control List (ACL)
/// permissions. These Cloud IAM roles can only be applied to a bucket, not a project.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum LegacyIamRole {
    /// Grants permission to view objects and their metadata, excluding ACLs.
    #[serde(rename = "roles/storage.legacyObjectReader")]
    LegacyObjectReader,
    /// Grants permission to view and edit objects and their metadata, including ACLs.
    #[serde(rename = "roles/storage.legacyObjectOwner")]
    LegacyObjectOwner,
    /// Grants permission to list a bucket's contents and read bucket metadata, excluding Cloud IAM
    /// policies. Also grants permission to read object metadata, excluding Cloud IAM policies, when
    /// listing objects.
    ///
    /// Use of this role is also reflected in the bucket's ACLs. See
    /// [Cloud IAM relation to ACLs](https://cloud.google.com/storage/docs/access-control/iam#acls)
    /// for  more information.
    #[serde(rename = "roles/storage.legacyBucketReader")]
    LegacyBucketReader,
    /// Grants permission to create, overwrite, and delete objects; list objects in a bucket and
    /// read object metadata, excluding Cloud IAM policies, when listing; and read bucket metadata,
    /// excluding Cloud IAM policies.
    ///
    /// Use of this role is also reflected in the bucket's ACLs. See
    /// [Cloud IAM relation to ACLs](https://cloud.google.com/storage/docs/access-control/iam#acls)
    /// for  more information.
    #[serde(rename = "roles/storage.legacyBucketWriter")]
    LegacyBucketWriter,
    /// Grants permission to create, overwrite, and delete objects; list objects in a bucket and
    /// read object metadata, excluding Cloud IAM policies, when listing; and read and edit bucket
    /// metadata, including Cloud IAM policies.
    ///
    /// Use of this role is also reflected in the bucket's ACLs. See
    /// [Cloud IAM relation to ACLs](https://cloud.google.com/storage/docs/access-control/iam#acls)
    /// for  more information.
    #[serde(rename = "roles/storage.legacyBucketOwner")]
    LegacyBucketOwner,
}
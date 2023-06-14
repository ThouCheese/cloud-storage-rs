use super::{RetentionPolicy, BucketAccessControl, DefaultObjectAccessControl, IamConfiguration, Encryption, Owner, Website, Logging, Versioning, Cors, Lifecycle, StorageClass, Billing, Location};

/// The Buckets resource represents a
/// [bucket](https://cloud.google.com/storage/docs/key-terms#buckets) in Google Cloud Storage. There
/// is a single global namespace shared by all buckets. For more information, see
/// [Bucket Name Requirements](https://cloud.google.com/storage/docs/naming#requirements).
///
/// Buckets contain objects which can be accessed by their own methods. In addition to the
/// [ACL property](https://cloud.google.com/storage/docs/access-control/lists), buckets contain
/// `BucketAccessControls`, for use in fine-grained manipulation of an existing bucket's access
/// controls.
///
/// A bucket is always owned by the project team owners group.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bucket {
    /// The kind of item this is. For buckets, this is always `storage#bucket`.
    pub kind: String,
    /// The ID of the bucket. For buckets, the `id` and `name` properties are the same.
    pub id: String, // should be u64, mumble mumble
    /// The URI of this bucket.
    pub self_link: String,
    /// The project number of the project the bucket belongs to.
    #[serde(deserialize_with = "crate::from_str")]
    pub project_number: u64,
    /// The name of the bucket.
    pub name: String,
    /// The creation time of the bucket in RFC 3339 format.
    #[serde(with = "time::serde::rfc3339")]
    pub time_created: time::OffsetDateTime,
    /// The modification time of the bucket in RFC 3339 format.
    #[serde(with = "time::serde::rfc3339")]
    pub updated: time::OffsetDateTime,
    /// Whether or not to automatically apply an eventBasedHold to new objects added to the bucket.
    pub default_event_based_hold: Option<bool>,
    /// The bucket's retention policy, which defines the minimum age an object in the bucket must
    /// reach before it can be deleted or overwritten.
    pub retention_policy: Option<RetentionPolicy>,
    /// The metadata generation of this bucket.
    #[serde(deserialize_with = "crate::from_str")]
    pub metageneration: i64,
    /// Access controls on the bucket, containing one or more bucketAccessControls Resources. If
    /// iamConfiguration.uniformBucketLevelAccess.enabled is set to true, this field is omitted in
    /// responses, and requests that specify this field fail with a 400 Bad Request response.
    pub acl: Option<Vec<BucketAccessControl>>,
    /// Default access controls to apply to new objects when no ACL is provided. This list contains
    /// one or more defaultObjectAccessControls Resources. If
    /// iamConfiguration.uniformBucketLevelAccess.enabled is set to true, this field is omitted in
    /// responses, and requests that specify this field fail.
    pub default_object_acl: Option<Vec<DefaultObjectAccessControl>>,
    /// The bucket's IAM configuration.
    pub iam_configuration: Option<IamConfiguration>,
    /// Encryption configuration for a bucket.
    pub encryption: Option<Encryption>,
    /// The owner of the bucket. This is always the project team's owner group.
    pub owner: Option<Owner>,
    /// The location of the bucket. Object data for objects in the bucket resides in physical
    /// storage within this region. Defaults to US. See Cloud Storage bucket locations for the
    /// authoritative list.
    pub location: Location,
    /// The type of location that the bucket resides in, as determined by the location property.
    pub location_type: String,
    /// The bucket's website configuration, controlling how the service behaves when accessing
    /// bucket contents as a web site. See the Static Website Examples for more information.
    pub website: Option<Website>,
    /// The bucket's logging configuration, which defines the destination bucket and optional name
    /// prefix for the current bucket's logs.
    pub logging: Option<Logging>,
    /// The bucket's versioning configuration.
    pub versioning: Option<Versioning>,
    /// The bucket's Cross-Origin Resource Sharing (CORS) configuration.
    pub cors: Option<Vec<Cors>>,
    /// The bucket's lifecycle configuration. See
    /// [lifecycle management](https://cloud.google.com/storage/docs/lifecycle) for more
    /// information.
    pub lifecycle: Option<Lifecycle>,
    /// User-provided bucket labels, in key/value pairs.
    pub labels: Option<std::collections::HashMap<String, String>>,
    /// The bucket's default storage class, used whenever no storageClass is specified for a
    /// newly-created object. If storageClass is not specified when the bucket
    /// is created, it defaults to STANDARD. For more information, see storage classes.
    pub storage_class: StorageClass,
    /// The bucket's billing configuration.
    pub billing: Option<Billing>,
    /// HTTP 1.1 [Entity tag](https://tools.ietf.org/html/rfc7232#section-2.3) for the bucket.
    pub etag: String,
}
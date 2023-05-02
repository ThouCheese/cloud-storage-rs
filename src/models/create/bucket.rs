use crate::models::{IamConfiguration, Encryption, Website, Logging, Versioning, Cors, Lifecycle, StorageClass, Billing, Location};
use super::{BucketAccessControl, DefaultObjectAccessControl};

/// A model that can be used to insert new buckets into Google Cloud Storage.
#[derive(Debug, PartialEq, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bucket {
    /// The name of the bucket. See the bucket naming guidelines for more information.
    pub name: String,
    /// Whether or not to automatically apply an eventBasedHold to new objects added to the bucket.
    pub default_event_based_hold: Option<bool>,
    /// Access controls on the bucket, containing one or more `BucketAccessControls` resources. If
    /// `iamConfiguration.uniformBucketLevelAccess.enabled` is set to true, this field is omitted in
    /// responses, and requests that specify this field fail with a `400 Bad Request` response.
    pub acl: Option<Vec<BucketAccessControl>>,
    /// Default access controls to apply to new objects when no ACL is provided. This list defines
    /// an entity and role for one or more `DefaultObjectAccessControls` resources. If
    /// `iamConfiguration.uniformBucketLevelAccess.enabled` is set to true, this field is omitted in
    /// responses, and requests that specify this field fail with a `400 Bad Request` response.
    pub default_object_acl: Option<Vec<DefaultObjectAccessControl>>,
    /// The bucket's IAM configuration.
    pub iam_configuration: Option<IamConfiguration>,
    /// Encryption configuration for a bucket.
    pub encryption: Option<Encryption>,
    /// The location of the bucket. Object data for objects in the bucket resides in physical
    /// storage within this region. Defaults to US. See Cloud Storage bucket locations for the
    /// authoritative list.
    pub location: Location,
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
    /// The bucket's lifecycle configuration. See [lifecycle management](https://cloud.google.com/storage/docs/lifecycle) for more information.
    pub lifecycle: Option<Lifecycle>,
    /// User-provided bucket labels, in key/value pairs.
    pub labels: Option<std::collections::HashMap<String, String>>,
    /// The bucket's default storage class, used whenever no storageClass is specified for a
    /// newly-created object. If storageClass is not specified when the bucket
    /// is created, it defaults to STANDARD. For more information, see storage classes.
    pub storage_class: Option<StorageClass>,
    /// The bucket's billing configuration.
    pub billing: Option<Billing>,
}
use crate::resources::{
    bucket_access_control::{BucketAccessControl, NewBucketAccessControl},
    default_object_access_control::{DefaultObjectAccessControl, NewDefaultObjectAccessControl},
};
pub use crate::resources::{common::Entity, location::*};

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
    pub time_created: chrono::DateTime<chrono::Utc>,
    /// The modification time of the bucket in RFC 3339 format.
    pub updated: chrono::DateTime<chrono::Utc>,
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

/// A model that can be used to insert new buckets into Google Cloud Storage.
#[derive(Debug, PartialEq, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewBucket {
    /// The name of the bucket. See the bucket naming guidelines for more information.
    pub name: String,
    /// Whether or not to automatically apply an eventBasedHold to new objects added to the bucket.
    pub default_event_based_hold: Option<bool>,
    /// Access controls on the bucket, containing one or more `BucketAccessControls` resources. If
    /// `iamConfiguration.uniformBucketLevelAccess.enabled` is set to true, this field is omitted in
    /// responses, and requests that specify this field fail with a `400 Bad Request` response.
    pub acl: Option<Vec<NewBucketAccessControl>>,
    /// Default access controls to apply to new objects when no ACL is provided. This list defines
    /// an entity and role for one or more `DefaultObjectAccessControls` resources. If
    /// `iamConfiguration.uniformBucketLevelAccess.enabled` is set to true, this field is omitted in
    /// responses, and requests that specify this field fail with a `400 Bad Request` response.
    pub default_object_acl: Option<Vec<NewDefaultObjectAccessControl>>,
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
    /// The bucket's lifecycle configuration. See
    /// [lifecycle management](https://cloud.google.com/storage/docs/lifecycle) for more
    /// information.
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

/// Contains information about how files are kept after deletion.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetentionPolicy {
    /// The period of time, in seconds, that objects in the bucket must be retained and cannot be
    /// deleted, overwritten, or made noncurrent. The value must be greater than 0 seconds and less
    /// than 3,155,760,000 seconds.
    #[serde(deserialize_with = "crate::from_str")]
    pub retention_period: u64,
    /// The time from which the retentionPolicy was effective, in RFC 3339 format.
    pub effective_time: chrono::DateTime<chrono::Utc>,
    /// Whether or not the retentionPolicy is locked. If true, the retentionPolicy cannot be removed
    /// and the retention period cannot be reduced.
    pub is_locked: Option<bool>,
}

/// Contains information about the Buckets IAM configuration.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IamConfiguration {
    /// The bucket's uniform bucket-level access configuration.
    ///
    /// Note: iamConfiguration also includes the bucketPolicyOnly field, which uses a legacy name
    /// but has the same functionality as the uniformBucketLevelAccess field. We recommend only
    /// using uniformBucketLevelAccess, as specifying both fields may result in unreliable behavior.
    pub uniform_bucket_level_access: UniformBucketLevelAccess,
}

/// Access that is configured for all objects in one go.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UniformBucketLevelAccess {
    /// Whether or not the bucket uses uniform bucket-level access. If set, access checks only use
    /// bucket-level IAM policies or above.
    pub enabled: bool,
    /// The deadline time for changing iamConfiguration.uniformBucketLevelAccess.enabled from true
    /// to false, in RFC 3339 format.
    ///
    /// iamConfiguration.uniformBucketLevelAccess.enabled may be changed from true to false until
    /// the locked time, after which the field is immutable.
    pub locked_time: Option<chrono::DateTime<chrono::Utc>>,
}

/// Contains information about the encryption used for data in this Bucket.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Encryption {
    /// A Cloud KMS key that will be used to encrypt objects inserted into this bucket, if no
    /// encryption method is specified.
    pub default_kms_key_name: String,
}

/// Contains information about an entity that is able to own a `Bucket`.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    /// The entity, in the form project-owner-projectId.
    pub entity: Entity,
    /// The ID for the entity.
    pub entity_id: Option<String>,
}

/// Contains configuration about how to visit the website linked to this Bucket.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Website {
    /// If the requested object path is missing, the service will ensure the path has a trailing
    /// '/', append this suffix, and attempt to retrieve the resulting object. This allows the
    /// creation of index.html objects to represent directory pages.
    pub main_page_suffix: String,
    /// If the requested object path is missing, and any mainPageSuffix object is missing, if
    /// applicable, the service will return the named object from this bucket as the content for a
    /// 404 Not Found result.
    pub not_found_page: String,
}

/// Contains information of where and how access logs to this bucket are maintained.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Logging {
    /// The destination bucket where the current bucket's logs should be placed.
    pub log_bucket: String,
    /// A prefix for log object names. The default prefix is the bucket name.
    pub log_object_prefix: String,
}

/// Contains information about whether a Bucket keeps track of its version.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Versioning {
    /// While set to true, versioning is fully enabled for this bucket.
    pub enabled: bool,
}

/// Contains information about how OPTIONS requests for this Bucket are handled.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cors {
    /// The list of Origins eligible to receive CORS response headers. Note: "*" is permitted in the
    /// list of origins, and means "any Origin".
    #[serde(default)]
    pub origin: Vec<String>,
    /// The list of HTTP methods on which to include CORS response headers, (GET, OPTIONS, POST,
    /// etc) Note: "*" is permitted in the list of methods, and means "any method".
    #[serde(default)]
    pub method: Vec<String>,
    /// The list of HTTP headers other than the simple response headers to give permission for the
    /// user-agent to share across domains.
    #[serde(default)]
    pub response_header: Vec<String>,
    /// The value, in seconds, to return in the Access-Control-Max-Age header used in preflight
    /// responses.
    pub max_age_seconds: Option<i32>,
}

/// Contains a set of `Rule` Objects which together describe the way this lifecycle behaves
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lifecycle {
    /// A lifecycle management rule, which is made of an action to take and the condition(s) under
    /// which the action will be taken.
    pub rule: Vec<Rule>,
}

/// An element of the lifecyle list.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    /// The action to take.
    pub action: Action,
    /// The condition(s) under which the action will be taken.
    pub condition: Condition,
}

/// Represents an action that might be undertaken due to a `Condition`.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    /// Type of the action.
    pub r#type: ActionType,
    /// Target storage class. Required iff the type of the action is SetStorageClass.
    pub storage_class: Option<StorageClass>,
}

/// Type of the action.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ActionType {
    /// Deletes a Bucket.
    Delete,
    /// Sets the `storage_class` of a Bucket.
    SetStorageClass,
}

/// A rule that might induce an `Action` if met.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Condition {
    /// Age of an object (in days). This condition is satisfied when an object reaches the specified
    /// age.
    pub age: Option<i32>,
    /// A date in `RFC 3339` format with only the date part (for instance, "2013-01-15"). This
    /// condition is satisfied when an object is created before midnight of the specified date in
    /// UTC.
    pub created_before: Option<chrono::NaiveDate>,
    /// Relevant only for versioned objects. If the value is true, this condition matches the live
    /// version of objects; if the value is `false`, it matches noncurrent versions of objects.
    pub is_live: Option<bool>,
    /// Objects having any of the storage classes specified by this condition will be matched.
    /// Values include STANDARD, NEARLINE, COLDLINE, MULTI_REGIONAL, REGIONAL, and
    /// DURABLE_REDUCED_AVAILABILITY.
    pub matches_storage_class: Option<Vec<String>>,
    /// Relevant only for versioned objects. If the value is N, this condition is satisfied when
    /// there are at least N versions (including the live version) newer than this version of the
    /// object.
    #[serde(default, deserialize_with = "crate::from_str_opt")]
    pub num_newer_versions: Option<i32>,
}

/// Contains information about the payment structure of this bucket
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Billing {
    /// When set to true, Requester Pays is enabled for this bucket.
    pub requester_pays: bool,
}

/// The type of storage that is used. Pertains to availability, performance and cost.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StorageClass {
    /// Standard Storage is best for data that is frequently accessed ("hot" data) and/or stored for
    /// only brief periods of time.
    Standard,
    /// Nearline Storage is a low-cost, highly durable storage service for storing infrequently
    /// accessed data.
    Nearline,
    /// Coldline Storage is a very-low-cost, highly durable storage service for data archiving,
    /// online backup, and disaster recovery.
    Coldline,
    /// Equivalent to Standard Storage, except Multi-Regional Storage can only be used for objects
    /// stored in multi-regions or dual-regions.
    MultiRegional,
    /// Equivalent to Standard Storage, except Regional Storage can only be used for objects stored
    /// in regions.
    Regional,
    /// Similar to Standard Storage except:
    ///
    /// DRA has higher pricing for operations.
    /// DRA has lower performance, particularly in terms of availability (DRA has a 99% availability
    /// SLA).
    ///
    /// You can move your data from DRA to other storage classes by performing a storage transfer.
    DurableReducedAvailability,
}

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

/// An association between a role, which comes with a set of permissions, and members who may assume
/// that role.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Binding {
    /// The role to which members belong. Two types of roles are supported: standard IAM roles,
    /// which grant permissions that do not map directly to those provided by ACLs, and legacy IAM
    /// roles, which do map directly to ACL permissions. All roles are of the format
    /// `roles/storage.specificRole.`
    ///
    /// See
    /// [Cloud Storage IAM Roles](https://cloud.google.com/storage/docs/access-control/iam-roles)
    /// for a list of available roles.
    pub role: IamRole,
    /// A collection of identifiers for members who may assume the provided role. Recognized
    /// identifiers are as follows:
    ///
    /// * `allUsers` — A special identifier that represents anyone on the internet; with or without
    ///   a Google account.
    /// * `allAuthenticatedUsers` — A special identifier that represents anyone who is authenticated
    ///   with a Google account or a service account.
    /// * `user:emailid` — An email address that represents a specific account. For example,
    ///   user:alice@gmail.com or user:joe@example.com.
    /// * `serviceAccount:emailid` — An email address that represents a service account. For
    ///   example, serviceAccount:my-other-app@appspot.gserviceaccount.com .
    /// * `group:emailid` — An email address that represents a Google group. For example,
    ///   group:admins@example.com.
    /// * `domain:domain` — A G Suite domain name that represents all the users of that domain. For
    ///   example, domain:google.com or domain:example.com.
    /// * `projectOwner:projectid` — Owners of the given project. For example,
    ///   projectOwner:my-example-project
    /// * `projectEditor:projectid` — Editors of the given project. For example,
    ///   projectEditor:my-example-project
    /// * `projectViewer:projectid` — Viewers of the given project. For example,
    ///   projectViewer:my-example-project
    pub members: Vec<String>,
    /// A condition object associated with this binding. Each role binding can only contain one
    /// condition.
    pub condition: Option<IamCondition>,
}

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

/// All possible roles that can exist in the IAM system. For a more comprehensive version, check
/// [Googles Documentation](https://cloud.google.com/storage/docs/access-control/iam-roles).
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum IamRole {
    /// Standard roles can be applied to either buckets or projects.
    Standard(StandardIamRole),
    /// Primitive roles are roles that must be added on a per-project basis.
    Primitive(PrimitiveIamRole),
    /// Legacy roles are roles that can only be added to an individual bucket.
    Legacy(LegacyIamRole),
}

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

impl Bucket {
    /// Creates a new `Bucket`. There are many options that you can provide for creating a new
    /// bucket, so the `NewBucket` resource contains all of them. Note that `NewBucket` implements
    /// `Default`, so you don't have to specify the fields you're not using. And error is returned
    /// if that bucket name is already taken.
    /// ### Example
    /// ```
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::bucket::{Bucket, NewBucket};
    /// use cloud_storage::bucket::{Location, MultiRegion};
    ///
    /// let new_bucket = NewBucket {
    ///    name: "cloud-storage-rs-doc-1".to_string(), // this is the only mandatory field
    ///    location: Location::Multi(MultiRegion::Eu),
    ///    ..Default::default()
    /// };
    /// let bucket = Bucket::create(&new_bucket).await?;
    /// # bucket.delete().await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "global-client")]
    pub async fn create(new_bucket: &NewBucket) -> crate::Result<Self> {
        crate::CLOUD_CLIENT.bucket().create(new_bucket).await
    }

    /// The synchronous equivalent of `Bucket::create`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(all(feature = "global-client", feature = "sync"))]
    pub fn create_sync(new_bucket: &NewBucket) -> crate::Result<Self> {
        crate::runtime()?.block_on(Self::create(new_bucket))
    }

    /// Returns all `Bucket`s within this project.
    ///
    /// ### Note
    /// When using incorrect permissions, this function fails silently and returns an empty list.
    ///
    /// ### Example
    /// ```
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Bucket;
    ///
    /// let buckets = Bucket::list().await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "global-client")]
    pub async fn list() -> crate::Result<Vec<Self>> {
        crate::CLOUD_CLIENT.bucket().list().await
    }

    /// The synchronous equivalent of `Bucket::list`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(all(feature = "global-client", feature = "sync"))]
    pub fn list_sync() -> crate::Result<Vec<Self>> {
        crate::runtime()?.block_on(Self::list())
    }

    /// Returns a single `Bucket` by its name. If the Bucket does not exist, an error is returned.
    /// ### Example
    /// ```
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Bucket;
    /// # use cloud_storage::bucket::NewBucket;
    /// # let new_bucket = NewBucket {
    /// #   name: "cloud-storage-rs-doc-2".to_string(),
    /// #    ..Default::default()
    /// # };
    /// # let _ = Bucket::create(&new_bucket).await?;
    ///
    /// let bucket = Bucket::read("cloud-storage-rs-doc-2").await?;
    /// # bucket.delete().await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "global-client")]
    pub async fn read(name: &str) -> crate::Result<Self> {
        crate::CLOUD_CLIENT.bucket().read(name).await
    }

    /// The synchronous equivalent of `Bucket::read`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(all(feature = "global-client", feature = "sync"))]
    pub fn read_sync(name: &str) -> crate::Result<Self> {
        crate::runtime()?.block_on(Self::read(name))
    }

    /// Update an existing `Bucket`. If you declare you bucket as mutable, you can edit its fields.
    /// You can then flush your changes to Google Cloud Storage using this method.
    /// ### Example
    /// ```
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::bucket::{Bucket, RetentionPolicy};
    /// # use cloud_storage::bucket::NewBucket;
    /// # let new_bucket = NewBucket {
    /// #   name: "cloud-storage-rs-doc-3".to_string(),
    /// #    ..Default::default()
    /// # };
    /// # let _ = Bucket::create(&new_bucket).await?;
    ///
    /// let mut bucket = Bucket::read("cloud-storage-rs-doc-3").await?;
    /// bucket.retention_policy = Some(RetentionPolicy {
    ///     retention_period: 50,
    ///     effective_time: chrono::Utc::now() + chrono::Duration::seconds(50),
    ///     is_locked: Some(false),
    /// });
    /// bucket.update().await?;
    /// # bucket.delete().await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "global-client")]
    pub async fn update(&self) -> crate::Result<Self> {
        crate::CLOUD_CLIENT.bucket().update(self).await
    }

    /// The synchronous equivalent of `Bucket::update`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(all(feature = "global-client", feature = "sync"))]
    pub fn update_sync(&self) -> crate::Result<Self> {
        crate::runtime()?.block_on(self.update())
    }

    /// Delete an existing `Bucket`. This permanently removes a bucket from Google Cloud Storage.
    /// An error is returned when you don't have sufficient permissions, or when the
    /// `retention_policy` prevents you from deleting your Bucket.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Bucket;
    /// # use cloud_storage::bucket::NewBucket;
    /// # let new_bucket = NewBucket {
    /// #   name: "unnecessary-bucket".to_string(),
    /// #    ..Default::default()
    /// # };
    /// # let _ = Bucket::create(&new_bucket).await?;
    ///
    /// let bucket = Bucket::read("unnecessary-bucket").await?;
    /// bucket.delete().await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "global-client")]
    pub async fn delete(self) -> crate::Result<()> {
        crate::CLOUD_CLIENT.bucket().delete(self).await
    }

    /// The synchronous equivalent of `Bucket::delete`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(all(feature = "global-client", feature = "sync"))]
    pub fn delete_sync(self) -> crate::Result<()> {
        crate::runtime()?.block_on(self.delete())
    }

    /// Returns the [IAM Policy](https://cloud.google.com/iam/docs/) for this bucket.
    /// ### Example
    /// ```
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Bucket;
    /// # use cloud_storage::bucket::NewBucket;
    /// # let new_bucket = NewBucket {
    /// #   name: "cloud-storage-rs-doc-4".to_string(),
    /// #    ..Default::default()
    /// # };
    /// # let _ = Bucket::create(&new_bucket).await?;
    ///
    /// let bucket = Bucket::read("cloud-storage-rs-doc-4").await?;
    /// let policy = bucket.get_iam_policy().await?;
    /// # bucket.delete().await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "global-client")]
    pub async fn get_iam_policy(&self) -> crate::Result<IamPolicy> {
        crate::CLOUD_CLIENT.bucket().get_iam_policy(self).await
    }

    /// The synchronous equivalent of `Bucket::get_iam_policy`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(all(feature = "global-client", feature = "sync"))]
    pub fn get_iam_policy_sync(&self) -> crate::Result<IamPolicy> {
        crate::runtime()?.block_on(self.get_iam_policy())
    }

    /// Updates the [IAM Policy](https://cloud.google.com/iam/docs/) for this bucket.
    /// ### Example
    /// ```
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Bucket;
    /// use cloud_storage::bucket::{IamPolicy, Binding, IamRole, StandardIamRole, Entity};
    /// # use cloud_storage::bucket::NewBucket;
    /// # let new_bucket = NewBucket {
    /// #   name: "cloud-storage-rs-doc-5".to_string(),
    /// #    ..Default::default()
    /// # };
    /// # let _ = Bucket::create(&new_bucket).await?;
    ///
    /// let bucket = Bucket::read("cloud-storage-rs-doc-5").await?;
    /// let iam_policy = IamPolicy {
    ///     version: 1,
    ///     bindings: vec![
    ///         Binding {
    ///             role: IamRole::Standard(StandardIamRole::ObjectViewer),
    ///             members: vec!["allUsers".to_string()],
    ///             condition: None,
    ///         }
    ///     ],
    ///     ..Default::default()
    /// };
    /// let policy = bucket.set_iam_policy(&iam_policy).await?;
    /// # bucket.delete().await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "global-client")]
    pub async fn set_iam_policy(&self, iam: &IamPolicy) -> crate::Result<IamPolicy> {
        crate::CLOUD_CLIENT.bucket().set_iam_policy(self, iam).await
    }

    /// The synchronous equivalent of `Bucket::set_iam_policy`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(all(feature = "global-client", feature = "sync"))]
    pub fn set_iam_policy_sync(&self, iam: &IamPolicy) -> crate::Result<IamPolicy> {
        crate::runtime()?.block_on(self.set_iam_policy(iam))
    }

    /// Checks whether the user provided in the service account has this permission.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Bucket;
    ///
    /// let bucket = Bucket::read("my-bucket").await?;
    /// bucket.test_iam_permission("storage.buckets.get").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "global-client")]
    pub async fn test_iam_permission(&self, permission: &str) -> crate::Result<TestIamPermission> {
        crate::CLOUD_CLIENT
            .bucket()
            .test_iam_permission(self, permission)
            .await
    }

    /// The synchronous equivalent of `Bucket::test_iam_policy`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(all(feature = "global-client", feature = "sync"))]
    pub fn test_iam_permission_sync(&self, permission: &str) -> crate::Result<TestIamPermission> {
        crate::runtime()?.block_on(self.test_iam_permission(permission))
    }

    fn _lock_retention_policy() {
        todo!()
    }
}

#[cfg(all(test, feature = "global-client"))]
mod tests {
    use super::*;
    use crate::resources::common::Role;

    #[tokio::test]
    async fn create() -> Result<(), Box<dyn std::error::Error>> {
        dotenvy::dotenv().ok();
        let base_name = std::env::var("TEST_BUCKET")?;
        // use a more complex bucket in this test.
        let new_bucket = NewBucket {
            name: format!("{}-test-create", base_name),
            default_event_based_hold: Some(true),
            acl: Some(vec![NewBucketAccessControl {
                entity: Entity::AllUsers,
                role: Role::Reader,
            }]),
            default_object_acl: Some(vec![NewDefaultObjectAccessControl {
                entity: Entity::AllUsers,
                role: Role::Reader,
            }]),
            iam_configuration: Some(IamConfiguration {
                uniform_bucket_level_access: UniformBucketLevelAccess {
                    enabled: false,
                    locked_time: None,
                },
            }),
            ..Default::default()
        };
        let bucket = Bucket::create(&new_bucket).await?;
        bucket.delete().await?;
        Ok(())
    }

    #[tokio::test]
    async fn list() -> Result<(), Box<dyn std::error::Error>> {
        Bucket::list().await?;
        Ok(())
    }

    #[tokio::test]
    async fn update() -> Result<(), Box<dyn std::error::Error>> {
        let mut bucket = crate::create_test_bucket("test-update").await;
        bucket.retention_policy = Some(RetentionPolicy {
            retention_period: 50,
            effective_time: chrono::Utc::now() + chrono::Duration::seconds(50),
            is_locked: Some(false),
        });
        bucket.update().await?;
        let updated = Bucket::read(&bucket.name).await?;
        assert_eq!(updated.retention_policy.unwrap().retention_period, 50);
        bucket.delete().await?;
        Ok(())
    }

    // used a lot throughout the other tests, but included for completeness
    #[tokio::test]
    async fn delete() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::create_test_bucket("test-delete").await;
        bucket.delete().await?;
        Ok(())
    }

    #[tokio::test]
    async fn get_iam_policy() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::create_test_bucket("test-get-iam-policy").await;
        bucket.get_iam_policy().await?;
        bucket.delete().await?;
        Ok(())
    }

    #[tokio::test]
    async fn set_iam_policy() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::create_test_bucket("test-set-iam-policy").await;
        let iam_policy = IamPolicy {
            bindings: vec![Binding {
                role: IamRole::Standard(StandardIamRole::ObjectViewer),
                members: vec!["allUsers".to_string()],
                condition: None,
            }],
            ..Default::default()
        };
        bucket.set_iam_policy(&iam_policy).await?;
        assert_eq!(bucket.get_iam_policy().await?.bindings, iam_policy.bindings);
        bucket.delete().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_iam_permission() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::create_test_bucket("test-test-ia-permission").await;
        bucket.test_iam_permission("storage.buckets.get").await?;
        bucket.delete().await?;
        Ok(())
    }

    #[cfg(all(feature = "global-client", feature = "sync"))]
    mod sync {
        use super::*;
        use crate::resources::common::Role;

        #[test]
        fn create() -> Result<(), Box<dyn std::error::Error>> {
            dotenvy::dotenv().ok();
            let base_name = std::env::var("TEST_BUCKET")?;
            // use a more complex bucket in this test.
            let new_bucket = NewBucket {
                name: format!("{}-test-create", base_name),
                default_event_based_hold: Some(true),
                acl: Some(vec![NewBucketAccessControl {
                    entity: Entity::AllUsers,
                    role: Role::Reader,
                }]),
                default_object_acl: Some(vec![NewDefaultObjectAccessControl {
                    entity: Entity::AllUsers,
                    role: Role::Reader,
                }]),
                iam_configuration: Some(IamConfiguration {
                    uniform_bucket_level_access: UniformBucketLevelAccess {
                        enabled: false,
                        locked_time: None,
                    },
                }),
                ..Default::default()
            };
            let bucket = Bucket::create_sync(&new_bucket)?;
            bucket.delete_sync()?;
            Ok(())
        }

        #[test]
        fn list() -> Result<(), Box<dyn std::error::Error>> {
            Bucket::list_sync()?;
            Ok(())
        }

        #[test]
        fn read() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::create_test_bucket_sync("test-read");
            let also_bucket = Bucket::read_sync(&bucket.name)?;
            assert_eq!(bucket, also_bucket);
            bucket.delete_sync()?;
            assert!(also_bucket.delete_sync().is_err());
            Ok(())
        }

        #[test]
        fn update() -> Result<(), Box<dyn std::error::Error>> {
            let mut bucket = crate::create_test_bucket_sync("test-update");
            bucket.retention_policy = Some(RetentionPolicy {
                retention_period: 50,
                effective_time: chrono::Utc::now() + chrono::Duration::seconds(50),
                is_locked: Some(false),
            });
            bucket.update_sync()?;
            let updated = Bucket::read_sync(&bucket.name)?;
            assert_eq!(updated.retention_policy.unwrap().retention_period, 50);
            bucket.delete_sync()?;
            Ok(())
        }

        // used a lot throughout the other tests, but included for completeness
        #[test]
        fn delete() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::create_test_bucket_sync("test-delete");
            bucket.delete_sync()?;
            Ok(())
        }

        #[test]
        fn get_iam_policy() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::create_test_bucket_sync("test-get-iam-policy");
            bucket.get_iam_policy_sync()?;
            bucket.delete_sync()?;
            Ok(())
        }

        #[test]
        fn set_iam_policy() -> Result<(), Box<dyn std::error::Error>> {
            // use crate::resources::iam_policy::{Binding, IamRole, StandardIamRole};

            let bucket = crate::create_test_bucket_sync("test-set-iam-policy");
            let iam_policy = IamPolicy {
                bindings: vec![Binding {
                    role: IamRole::Standard(StandardIamRole::ObjectViewer),
                    members: vec!["allUsers".to_string()],
                    condition: None,
                }],
                ..Default::default()
            };
            bucket.set_iam_policy_sync(&iam_policy)?;
            assert_eq!(bucket.get_iam_policy_sync()?.bindings, iam_policy.bindings);
            bucket.delete_sync()?;
            Ok(())
        }

        #[test]
        fn test_iam_permission() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::create_test_bucket_sync("test-test-ia-permission");
            bucket.test_iam_permission_sync("storage.buckets.get")?;
            bucket.delete_sync()?;
            Ok(())
        }
    }
}

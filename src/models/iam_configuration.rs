use super::UniformBucketLevelAccess;

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
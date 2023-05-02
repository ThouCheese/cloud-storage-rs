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
    #[serde(with = "time::serde::rfc3339::option")]
    pub locked_time: Option<time::OffsetDateTime>,
}
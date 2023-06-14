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
    #[serde(with = "time::serde::rfc3339")]
    pub effective_time: time::OffsetDateTime,
    /// Whether or not the retentionPolicy is locked. If true, the retentionPolicy cannot be removed
    /// and the retention period cannot be reduced.
    pub is_locked: Option<bool>,
}
/// Contains information of where and how access logs to this bucket are maintained.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Logging {
    /// The destination bucket where the current bucket's logs should be placed.
    pub log_bucket: String,
    /// A prefix for log object names. The default prefix is the bucket name.
    pub log_object_prefix: String,
}
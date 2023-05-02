/// Contains information about whether a Bucket keeps track of its version.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Versioning {
    /// While set to true, versioning is fully enabled for this bucket.
    pub enabled: bool,
}
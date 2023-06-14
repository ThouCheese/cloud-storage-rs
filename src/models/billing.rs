/// Contains information about the payment structure of this bucket
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Billing {
    /// When set to true, Requester Pays is enabled for this bucket.
    pub requester_pays: bool,
}
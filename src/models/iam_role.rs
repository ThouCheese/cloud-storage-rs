use super::{StandardIamRole, PrimitiveIamRole, LegacyIamRole};

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
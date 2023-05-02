use super::{IamRole, IamCondition};

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
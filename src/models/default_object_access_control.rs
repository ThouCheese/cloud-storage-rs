use super::{Entity, Role, ProjectTeam};

/// The DefaultObjectAccessControls resources represent the Access Control Lists (ACLs) applied to a
/// new object within Google Cloud Storage when no ACL was provided for that object. ACLs let you
/// specify who has access to your data and to what extent.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultObjectAccessControl {
    /// The kind of item this is. For object access control entries, this is always
    /// storage#objectAccessControl.
    pub kind: String,
    /// The entity holding the permission, in one of the following forms:
    ///
    /// * `user-userId`
    /// * `user-email`
    /// * `group-groupId`
    /// * `group-email`
    /// * `domain-domain`
    /// * `project-team-projectId`
    /// * `allUsers`
    /// * `allAuthenticatedUsers`
    ///
    /// Examples:
    ///
    /// * The user liz@example.com would be user-liz@example.com.
    /// * The group example@googlegroups.com would be group-example@googlegroups.com.
    /// * To refer to all members of the G Suite for Business domain example.com, the entity would
    /// be domain-example.com.
    pub entity: Entity,
    /// The access permission for the entity.
    pub role: Role,
    /// The email address associated with the entity, if any.
    pub email: Option<String>,
    /// The ID for the entity, if any.
    pub entity_id: Option<String>,
    /// The domain associated with the entity, if any.
    pub domain: Option<String>,
    /// The project team associated with the entity, if any.
    pub project_team: Option<ProjectTeam>,
    /// HTTP 1.1 Entity tag for the access-control entry.
    pub etag: String,
    /// The bucket this resource belongs to.
    #[serde(default)]
    pub bucket: String, // this field is not returned by Google, but we populate it manually for the convenience of the end user.
}
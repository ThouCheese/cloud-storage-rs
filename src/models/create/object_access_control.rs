use crate::models::{Entity, Role};

/// Used to create a new `ObjectAccessControl` object.
#[derive(Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectAccessControl {
    /// The entity holding the permission, in one of the following forms:
    ///
    /// user-userId
    /// user-email
    /// group-groupId
    /// group-email
    /// domain-domain
    /// project-team-projectId
    /// allUsers
    /// allAuthenticatedUsers
    ///
    /// Examples:
    ///
    /// The user liz@example.com would be user-liz@example.com.
    /// The group example@googlegroups.com would be group-example@googlegroups.com.
    /// To refer to all members of the G Suite for Business domain example.com, the entity would be
    /// domain-example.com.
    pub entity: Entity,
    /// The access permission for the entity.
    pub role: Role,
}
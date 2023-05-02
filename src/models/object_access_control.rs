use super::{Entity, ProjectTeam, Role};

/// The ObjectAccessControls resources represent the Access Control Lists (ACLs) for objects within
/// Google Cloud Storage. ACLs let you specify who has access to your data and to what extent.
///
/// ```text,ignore
/// Important: The methods for this resource fail with a 400 Bad Request response for buckets with
/// uniform bucket-level access enabled. Use storage.buckets.getIamPolicy and
/// storage.buckets.setIamPolicy to control access instead.
/// ```
///
/// There are two roles that can be assigned to an entity:
///
/// READERs can get an object, though the acl property will not be revealed.
/// OWNERs are READERs, and they can get the acl property, update an object, and call all
/// objectAccessControls methods on the object. The owner of an object is always an OWNER.
///
/// For more information, see Access Control, with the caveat that this API uses READER and OWNER
/// instead of READ and FULL_CONTROL.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectAccessControl {
    /// The kind of item this is. For object access control entries, this is always
    /// `storage#objectAccessControl`.
    pub kind: String,
    /// The ID of the access-control entry.
    pub id: String,
    /// The link to this access-control entry.
    pub self_link: String,
    /// The name of the bucket.
    pub bucket: String,
    /// The name of the object, if applied to an object.
    pub object: String,
    /// The content generation of the object, if applied to an object.
    pub generation: Option<String>,
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
}
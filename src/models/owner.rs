use super::Entity;

/// Contains information about an entity that is able to own a `Bucket`.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    /// The entity, in the form project-owner-projectId.
    pub entity: Entity,
    /// The ID for the entity.
    pub entity_id: Option<String>,
}
use super::Team;

/// Contains information about the team related to this `DefaultObjectAccessControls`
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectTeam {
    /// The project number.
    project_number: String,
    /// The team.
    team: Team,
}
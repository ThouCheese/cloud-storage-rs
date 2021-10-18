use serde::Serializer;
use std::str::FromStr;

/// Contains information about the team related to this `DefaultObjectAccessControls`
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectTeam {
    /// The project number.
    project_number: String,
    /// The team.
    team: Team,
}

/// Any type of team we can encounter.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Team {
    /// The team consists of `Editors`.
    Editors,
    /// The team consists of `Owners`.
    Owners,
    /// The team consists of `Viewers`.
    Viewers,
}

impl std::fmt::Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Team::Editors => write!(f, "editors"),
            Team::Owners => write!(f, "owners"),
            Team::Viewers => write!(f, "viewers"),
        }
    }
}

impl FromStr for Team {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "editors" => Ok(Self::Editors),
            "owners" => Ok(Self::Owners),
            "viewers" => Ok(Self::Viewers),
            _ => Err(format!("Invalid `Team`: {}", s)),
        }
    }
}

/// Any type of role we can encounter.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Role {
    /// Full access.
    Owner,
    /// Write, but not administer.
    Writer,
    /// Only read access.
    Reader,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ListResponse<T> {
    #[serde(default = "Vec::new")]
    pub items: Vec<T>,
    // pub next_page_token: Option<String>,
}

/// An entity is used to represent a user or group of users that often have some kind of permission.
#[derive(Debug, PartialEq, Clone)]
pub enum Entity {
    /// A single user, identified by its id.
    UserId(String),
    /// A single user, identified by its email address.
    UserEmail(String),
    /// A group of users, identified by its id.
    GroupId(String),
    /// A group of users, identified by its email address.
    GroupEmail(String),
    /// All users identifed by an email that ends with the domain, for example `mydomain.rs` in
    /// `me@mydomain.rs`.
    Domain(String),
    /// All users within a project, identified by the `team` name and `project` id.
    Project(Team, String),
    /// All users.
    AllUsers,
    /// All users that are logged in.
    AllAuthenticatedUsers,
}

use Entity::*;

impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UserId(s) => write!(f, "user-{}", s),
            UserEmail(s) => write!(f, "user-{}", s),
            GroupId(s) => write!(f, "group-{}", s),
            GroupEmail(s) => write!(f, "group-{}", s),
            Domain(s) => write!(f, "domain-{}", s),
            Project(team, project_id) => write!(f, "project-{}-{}", team, project_id),
            AllUsers => write!(f, "allUsers"),
            AllAuthenticatedUsers => write!(f, "allAuthenticatedUsers"),
        }
    }
}

impl serde::Serialize for Entity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

struct EntityVisitor;

impl<'de> serde::de::Visitor<'de> for EntityVisitor {
    type Value = Entity;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("an `Entity` resource")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let parts: Vec<&str> = value.split('-').collect();
        let result = match &parts[..] {
            ["user", rest @ ..] if is_email(rest) => UserEmail(rest.join("-")),
            ["user", rest @ ..] => UserId(rest.join("-")),
            ["group", rest @ ..] if is_email(rest) => GroupEmail(rest.join("-")),
            ["group", rest @ ..] => GroupId(rest.join("-")),
            ["domain", rest @ ..] => Domain(rest.join("-")),
            ["project", team, project_id] => {
                Project(Team::from_str(team).unwrap(), project_id.to_string())
            }
            ["allUsers"] => AllUsers,
            ["allAuthenticatedUsers"] => AllAuthenticatedUsers,
            _ => return Err(E::custom(format!("Unexpected `Entity`: {}", value))),
        };
        Ok(result)
    }
}

fn is_email(pattern: &[&str]) -> bool {
    pattern.iter().any(|s| s.contains('@'))
}

impl<'de> serde::Deserialize<'de> for Entity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(EntityVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let entity1 = UserId("some id".to_string());
        assert_eq!(serde_json::to_string(&entity1).unwrap(), "\"user-some id\"");

        let entity2 = UserEmail("some@email".to_string());
        assert_eq!(
            serde_json::to_string(&entity2).unwrap(),
            "\"user-some@email\""
        );

        let entity3 = GroupId("some group id".to_string());
        assert_eq!(
            serde_json::to_string(&entity3).unwrap(),
            "\"group-some group id\""
        );

        let entity4 = GroupEmail("some@group.email".to_string());
        assert_eq!(
            serde_json::to_string(&entity4).unwrap(),
            "\"group-some@group.email\""
        );

        let entity5 = Domain("example.com".to_string());
        assert_eq!(
            serde_json::to_string(&entity5).unwrap(),
            "\"domain-example.com\""
        );

        let entity6 = Project(Team::Viewers, "project id".to_string());
        assert_eq!(
            serde_json::to_string(&entity6).unwrap(),
            "\"project-viewers-project id\""
        );

        let entity7 = AllUsers;
        assert_eq!(serde_json::to_string(&entity7).unwrap(), "\"allUsers\"");

        let entity8 = AllAuthenticatedUsers;
        assert_eq!(
            serde_json::to_string(&entity8).unwrap(),
            "\"allAuthenticatedUsers\""
        );
    }

    #[test]
    fn deserialize() {
        let str1 = "\"user-some id\"";
        assert_eq!(
            serde_json::from_str::<Entity>(str1).unwrap(),
            UserId("some id".to_string())
        );

        let str2 = "\"user-some@email\"";
        assert_eq!(
            serde_json::from_str::<Entity>(str2).unwrap(),
            UserEmail("some@email".to_string())
        );

        let str3 = "\"group-some group id\"";
        assert_eq!(
            serde_json::from_str::<Entity>(str3).unwrap(),
            GroupId("some group id".to_string())
        );

        let str4 = "\"group-some@group.email\"";
        assert_eq!(
            serde_json::from_str::<Entity>(str4).unwrap(),
            GroupEmail("some@group.email".to_string())
        );

        let str5 = "\"domain-example.com\"";
        assert_eq!(
            serde_json::from_str::<Entity>(str5).unwrap(),
            Domain("example.com".to_string())
        );

        let str6 = "\"project-viewers-project id\"";
        assert_eq!(
            serde_json::from_str::<Entity>(str6).unwrap(),
            Project(Team::Viewers, "project id".to_string())
        );

        let str7 = "\"allUsers\"";
        assert_eq!(serde_json::from_str::<Entity>(str7).unwrap(), AllUsers);

        let str8 = "\"allAuthenticatedUsers\"";
        assert_eq!(
            serde_json::from_str::<Entity>(str8).unwrap(),
            AllAuthenticatedUsers
        );
    }
}

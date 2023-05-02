use std::str::FromStr;

use super::Team;

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

impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Entity::UserId(s) => write!(f, "user-{}", s),
            Entity::UserEmail(s) => write!(f, "user-{}", s),
            Entity::GroupId(s) => write!(f, "group-{}", s),
            Entity::GroupEmail(s) => write!(f, "group-{}", s),
            Entity::Domain(s) => write!(f, "domain-{}", s),
            Entity::Project(team, project_id) => write!(f, "project-{}-{}", team, project_id),
            Entity::AllUsers => write!(f, "allUsers"),
            Entity::AllAuthenticatedUsers => write!(f, "allAuthenticatedUsers"),
        }
    }
}

// This uses Display to serialize a entity as a string based enum variant, rather than generating an object
impl serde::Serialize for Entity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl<'de> serde::Deserialize<'de> for Entity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(EntityVisitor)
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
            ["user", rest @ ..] if is_email(rest) => Entity::UserEmail(rest.join("-")),
            ["user", rest @ ..] => Entity::UserId(rest.join("-")),
            ["group", rest @ ..] if is_email(rest) => Entity::GroupEmail(rest.join("-")),
            ["group", rest @ ..] => Entity::GroupId(rest.join("-")),
            ["domain", rest @ ..] => Entity::Domain(rest.join("-")),
            ["project", team, project_id] => {
                Entity::Project(Team::from_str(team).unwrap(), project_id.to_string())
            }
            ["allUsers"] => Entity::AllUsers,
            ["allAuthenticatedUsers"] => Entity::AllAuthenticatedUsers,
            _ => return Err(E::custom(format!("Unexpected `Entity`: {}", value))),
        };
        Ok(result)
    }
}

// Used for EntityVisitor
fn is_email(pattern: &[&str]) -> bool {
    pattern.iter().any(|s| s.contains('@'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let entity1 = Entity::UserId("some id".to_string());
        assert_eq!(serde_json::to_string(&entity1).unwrap(), "\"user-some id\"");

        let entity2 = Entity::UserEmail("some@email".to_string());
        assert_eq!(
            serde_json::to_string(&entity2).unwrap(),
            "\"user-some@email\""
        );

        let entity3 = Entity::GroupId("some group id".to_string());
        assert_eq!(
            serde_json::to_string(&entity3).unwrap(),
            "\"group-some group id\""
        );

        let entity4 = Entity::GroupEmail("some@group.email".to_string());
        assert_eq!(
            serde_json::to_string(&entity4).unwrap(),
            "\"group-some@group.email\""
        );

        let entity5 = Entity::Domain("example.com".to_string());
        assert_eq!(
            serde_json::to_string(&entity5).unwrap(),
            "\"domain-example.com\""
        );

        let entity6 = Entity::Project(Team::Viewers, "project id".to_string());
        assert_eq!(
            serde_json::to_string(&entity6).unwrap(),
            "\"project-viewers-project id\""
        );

        let entity7 = Entity::AllUsers;
        assert_eq!(serde_json::to_string(&entity7).unwrap(), "\"allUsers\"");

        let entity8 = Entity::AllAuthenticatedUsers;
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
            Entity::UserId("some id".to_string())
        );

        let str2 = "\"user-some@email\"";
        assert_eq!(
            serde_json::from_str::<Entity>(str2).unwrap(),
            Entity::UserEmail("some@email".to_string())
        );

        let str3 = "\"group-some group id\"";
        assert_eq!(
            serde_json::from_str::<Entity>(str3).unwrap(),
            Entity::GroupId("some group id".to_string())
        );

        let str4 = "\"group-some@group.email\"";
        assert_eq!(
            serde_json::from_str::<Entity>(str4).unwrap(),
            Entity::GroupEmail("some@group.email".to_string())
        );

        let str5 = "\"domain-example.com\"";
        assert_eq!(
            serde_json::from_str::<Entity>(str5).unwrap(),
            Entity::Domain("example.com".to_string())
        );

        let str6 = "\"project-viewers-project id\"";
        assert_eq!(
            serde_json::from_str::<Entity>(str6).unwrap(),
            Entity::Project(Team::Viewers, "project id".to_string())
        );

        let str7 = "\"allUsers\"";
        assert_eq!(serde_json::from_str::<Entity>(str7).unwrap(), Entity::AllUsers);

        let str8 = "\"allAuthenticatedUsers\"";
        assert_eq!(
            serde_json::from_str::<Entity>(str8).unwrap(),
            Entity::AllAuthenticatedUsers
        );
    }
}

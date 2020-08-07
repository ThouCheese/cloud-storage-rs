/// The topic of a notification
#[derive(Debug, PartialEq)]
pub struct Topic {
    /// The project within which you want to receive notifications
    pub project_id: String,
    /// The topic of those notifications.
    pub topic: String,
}

impl std::fmt::Display for Topic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "//pubsub.googleapis.com/projects/{}/topics/{}",
            self.project_id, self.topic
        )
    }
}

impl serde::Serialize for Topic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

struct TopicVisitor;

impl<'de> serde::de::Visitor<'de> for TopicVisitor {
    type Value = Topic;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("an `Topic` resource")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let mut parts_iter = value.split('/');
        let error = || E::custom(format!("Invalid topic: `{}`", value));
        const START: [&str; 4] = ["", "", "pubsub.googleapis.com", "projects"];
        if parts_iter.clone().take(4).collect::<Vec<_>>() != START {
            return Err(error());
        }
        let project_id = parts_iter.next().ok_or_else(error)?;
        if parts_iter.next() != Some("topics") {
            return Err(error());
        }
        let topic = parts_iter.next().ok_or_else(error)?;
        let result = Topic {
            project_id: project_id.to_string(),
            topic: topic.to_string(),
        };
        Ok(result)
    }
}

impl<'de> serde::Deserialize<'de> for Topic {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(TopicVisitor)
    }
}

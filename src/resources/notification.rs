use crate::error::GoogleResponse;
use crate::resources::common::ListResponse;
pub use crate::resources::topic::Topic;

/// A subscription to receive
/// [Pub/Sub notifications](https://cloud.google.com/storage/docs/pubsub-notifications).
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Notification {
    /// The ID of the notification.
    id: String,
    /// The Pub/Sub topic to which this subscription publishes. Formatted as:
    /// `'//pubsub.googleapis.com/projects/{project-identifier}/topics/{my-topic}'`.
    topic: Topic,
    /// If present, only send notifications about listed event types. If empty, send notifications
    /// for all event types.
    event_types: Option<Vec<String>>,
    /// An optional list of additional attributes to attach to each Pub/Sub message published
    /// for this notification subscription.
    custom_attributes: Option<std::collections::HashMap<String, String>>,
    /// The desired content of the Payload.
    ///
    /// Acceptable values are:
    /// * "JSON_API_V1"
    /// * "NONE"
    payload_format: String,
    /// If present, only apply this notification configuration to object names that begin with this
    /// prefix.
    object_name_prefix: Option<String>,
    /// HTTP 1.1 Entity tag for this subscription notification.
    etag: String,
    /// The canonical URL of this notification.
    #[serde(rename = "selfLink")]
    self_link: String,
    /// The kind of item this is. For notifications, this is always `storage#notification`.   
    kind: String,
}

/// Use this struct to create new notifications.
#[derive(Debug, PartialEq, Default, serde::Serialize)]
pub struct NewNotification {
    /// The Pub/Sub topic to which this subscription publishes. Formatted as:
    /// `'//pubsub.googleapis.com/projects/{project-identifier}/topics/{my-topic}'`.
    topic: String,
    /// If present, only send notifications about listed event types. If empty, send notifications
    /// for all event types.
    event_types: Option<Vec<String>>,
    /// An optional list of additional attributes to attach to each Pub/Sub message published
    /// for this notification subscription.
    custom_attributes: Option<std::collections::HashMap<String, String>>,
    /// The desired content of the Payload.
    payload_format: Option<PayloadFormat>,
    /// If present, only apply this notification configuration to object names that begin with this
    /// prefix.
    object_name_prefix: Option<String>,
}

/// Various ways of having the response formatted.
#[derive(Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PayloadFormat {
    /// Respond with a format as specified in the Json API V1 documentation.
    JsonApiV1,
    /// Do not respond.
    None,
}

impl Notification {
    /// Creates a notification subscription for a given bucket.
    pub fn create(bucket: &str, new_notification: &NewNotification) -> Result<Self, crate::Error> {
        let url = format!("{}/b/{}/notificationConfigs", crate::BASE_URL, bucket);
        let client = reqwest::blocking::Client::new();
        let result: GoogleResponse<Self> = client
            .post(&url)
            .headers(crate::get_headers()?)
            .json(new_notification)
            .send()?
            .json()?;
        match result {
            GoogleResponse::Success(s) => Ok(s),
            GoogleResponse::Error(e) => Err(e.into()),
        }
    }

    /// View a notification configuration.
    pub fn read(bucket: &str, notification: &str) -> Result<Self, crate::Error> {
        let url = format!(
            "{}/b/{}/notificationConfigs/{}",
            crate::BASE_URL,
            bucket,
            notification
        );
        let client = reqwest::blocking::Client::new();
        let result: GoogleResponse<Self> = client
            .get(&url)
            .headers(crate::get_headers()?)
            .send()?
            .json()?;
        match result {
            GoogleResponse::Success(s) => Ok(s),
            GoogleResponse::Error(e) => Err(e.into()),
        }
    }

    /// Retrieves a list of notification subscriptions for a given bucket.}
    pub fn list(bucket: &str) -> Result<Vec<Self>, crate::Error> {
        let url = format!("{}/v1/b/{}/notificationConfigs", crate::BASE_URL, bucket);
        let client = reqwest::blocking::Client::new();
        let result: GoogleResponse<ListResponse<Self>> = client
            .get(&url)
            .headers(crate::get_headers()?)
            .send()?
            .json()?;
        match result {
            GoogleResponse::Success(s) => Ok(s.items),
            GoogleResponse::Error(e) => Err(e.into()),
        }
    }

    /// Permanently deletes a notification subscription.
    pub fn delete(bucket: &str, notification: &str) -> Result<(), crate::Error> {
        let url = format!(
            "{}/b/{}/notificationConfigs/{}",
            crate::BASE_URL,
            bucket,
            notification
        );
        let client = reqwest::blocking::Client::new();
        let response = client.get(&url).headers(crate::get_headers()?).send()?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(crate::Error::Google(response.json()?))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let bucket = crate::read_test_bucket();
        let topic = format!(
            "//pubsub.googleapis.com/projects/{}/topics/{}",
            crate::SERVICE_ACCOUNT.project_id,
            "testing-is-important",
        );
        let new_notification = NewNotification {
            topic,
            payload_format: Some(PayloadFormat::JsonApiV1),
            ..Default::default()
        };
        Notification::create(&bucket.name, &new_notification).unwrap();
    }

    #[test]
    fn read() {
        let bucket = crate::read_test_bucket();
        Notification::read(&bucket.name, "testing-is-important").unwrap();
    }

    #[test]
    fn list() {
        let bucket = crate::read_test_bucket();
        Notification::list(&bucket.name).unwrap();
    }

    #[test]
    fn delete() {
        let bucket = crate::read_test_bucket();
        let topic = format!(
            "//pubsub.googleapis.com/projects/{}/topics/{}",
            crate::SERVICE_ACCOUNT.project_id,
            "testing-is-important",
        );
        let new_notification = NewNotification {
            topic,
            payload_format: Some(PayloadFormat::JsonApiV1),
            ..Default::default()
        };
        Notification::create(&bucket.name, &new_notification).unwrap();
        Notification::delete(&bucket.name, "testing-is-important").unwrap();
    }
}

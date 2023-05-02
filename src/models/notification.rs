use std::collections::HashMap;
use super::{create, Topic};

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
    custom_attributes: Option<HashMap<String, String>>,
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

impl Notification {
    /// Creates a notification subscription for a given bucket.
    pub fn create(bucket: &str, new_notification: &create::Notification) -> Result<Self, crate::Error> {
        let url = format!("{}/b/{}/notificationConfigs", crate::BASE_URL, bucket);
        let result: crate::models::Response<Self> = crate::CLIENT
            .post(&url)
            .headers(crate::get_headers()?)
            .json(new_notification)
            .send()?
            .json()?;
        result
    }

    /// View a notification configuration.
    pub fn read(bucket: &str, notification: &str) -> Result<Self, crate::Error> {
        let url = format!(
            "{}/b/{}/notificationConfigs/{}",
            crate::BASE_URL,
            bucket,
            notification
        );
        let result: crate::models::Response<Self> = crate::CLIENT.get(&url).headers(crate::get_headers()?)
            .send()?
            .json()?;
        result
    }

    /// Retrieves a list of notification subscriptions for a given bucket.}
    pub fn list(bucket: &str) -> Result<Vec<Self>, crate::Error> {
        let url = format!("{}/v1/b/{}/notificationConfigs", crate::BASE_URL, bucket);
        let result: crate::models::Response<ListResponse<Self>> = crate::CLIENT
            .get(&url)
            .headers(crate::get_headers()?)
            .send()?
            .json()?;
        match result {
            crate::models::Response::Success(s) => Ok(s.items),
            crate::models::Response::Error(e) => Err(e.into()),
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
        let response = crate::CLIENT.get(&url).headers(crate::get_headers()?).send()?;
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
        let bucket = crate::global_client::read_test_bucket();
        #[cfg(feature = "dotenv")]
        dotenv::dotenv().ok();
        let service_account = crate::ServiceAccount::default();
        let topic = format!(
            "//pubsub.googleapis.com/projects/{}/topics/{}",
            service_account.project_id,
            "testing-is-important",
        );
        let new_notification = create::Notification {
            topic,
            payload_format: Some(create::PayloadFormat::JsonApiV1),
            ..Default::default()
        };
        Notification::create(&bucket.name, &new_notification).unwrap();
    }

    #[test]
    fn read() {
        let bucket = crate::global_client::read_test_bucket();
        Notification::read(&bucket.name, "testing-is-important").unwrap();
    }

    #[test]
    fn list() {
        let bucket = crate::global_client::read_test_bucket();
        Notification::list(&bucket.name).unwrap();
    }

    #[test]
    fn delete() {
        let bucket = crate::global_client::read_test_bucket();
        #[cfg(feature = "dotenv")]
        dotenv::dotenv().ok();
        let service_account = crate::ServiceAccount::default();
        let topic = format!(
            "//pubsub.googleapis.com/projects/{}/topics/{}",
            service_account.project_id,
            "testing-is-important",
        );
        let new_notification = create::Notification {
            topic,
            payload_format: Some(create::PayloadFormat::JsonApiV1),
            ..Default::default()
        };
        Notification::create(&bucket.name, &new_notification).unwrap();
        Notification::delete(&bucket.name, "testing-is-important").unwrap();
    }
}
use std::collections::HashMap;

use super::PayloadFormat;

/// Use this struct to create new notifications.
#[derive(Debug, PartialEq, Default, serde::Serialize)]
pub struct Notification {
    /// The Pub/Sub topic to which this subscription publishes. Formatted as:
    /// `'//pubsub.googleapis.com/projects/{project-identifier}/topics/{my-topic}'`.
    topic: String,
    /// If present, only send notifications about listed event types. If empty, send notifications
    /// for all event types.
    event_types: Option<Vec<String>>,
    /// An optional list of additional attributes to attach to each Pub/Sub message published
    /// for this notification subscription.
    custom_attributes: Option<HashMap<String, String>>,
    /// The desired content of the Payload.
    payload_format: Option<PayloadFormat>,
    /// If present, only apply this notification configuration to object names that begin with this
    /// prefix.
    object_name_prefix: Option<String>,
}
/// This complex object represents a Bucket that can be used to store and read files in Google Cloud
/// Storage.
pub mod bucket;
/// A Bucket Access Control object can be used to configure access on a bucket-wide level.
pub mod bucket_access_control;
/// Commonly used types.
pub mod common;
/// Default Object Access Control objects can be used the configure access that is used as a
/// fallback in the abscence of more specific data.
pub mod default_object_access_control;
/// An Hmac key is a secret key stored in Cloud Storage.
pub mod hmac_key;
/// A location where a bucket can exists physically.
mod location;
// /// A subscription to receive
// /// [Pub/Sub notifications](https://cloud.google.com/storage/docs/pubsub-notifications).
// pub mod notification;
/// A file
pub mod object;
/// Contains data about to access specific files.
pub mod object_access_control;
/// A deserialized version of the `service-account-********.json` file. Used to authenticate
/// requests.
pub mod service_account;
/// Used for parsing the `service-account-********.json` file.
pub(crate) mod signature;
/// The topic field of a `Notification`
mod topic;

//! Synchronous clients for Google Cloud Storage endpoints.

mod bucket;
mod bucket_access_control;
mod client;
mod default_object_access_control;
mod hmac_key;
mod object;
mod object_access_control;

mod helpers; // for internal use only

pub use client::Client;
pub use bucket::BucketClient;
pub use bucket_access_control::BucketAccessControlClient;
pub use default_object_access_control::DefaultObjectAccessControlClient;
pub use hmac_key::HmacKeyClient;
pub use object::ObjectClient;
pub use object_access_control::ObjectAccessControlClient;
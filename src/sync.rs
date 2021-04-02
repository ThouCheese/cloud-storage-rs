//! Synchronous clients for Google Cloud Storage endpoints.

mod bucket;
mod bucket_access_control;
mod default_object_access_control;
mod hmac_key;
mod object;
mod object_access_control;

pub use bucket::BucketClient;
pub use bucket_access_control::BucketAccessControlClient;
pub use default_object_access_control::DefaultObjectAccessControlClient;
pub use hmac_key::HmacKeyClient;
pub use object::ObjectClient;
pub use object_access_control::ObjectAccessControlClient;

/// The primary synchronous entrypoint to perform operations with Google Cloud Storage.
#[derive(Debug)]
pub struct Client {
    runtime: tokio::runtime::Runtime,
    client: crate::client::Client,
}

impl Client {
    /// Constructs a synchronous client
    pub fn new() -> crate::Result<Self> {
        Ok(Self {
            runtime: crate::runtime()?,
            client: Default::default(),
        })
    }

    /// Synchronous operations on [`Bucket`](crate::bucket::Bucket)s.
    pub fn bucket(&self) -> BucketClient<'_> {
        BucketClient(self)
    }

    /// Synchronous operations on [`BucketAccessControl`](crate::bucket_access_control::BucketAccessControl)s.
    pub fn bucket_access_control(&self) -> BucketAccessControlClient<'_> {
        BucketAccessControlClient(self)
    }

    /// Synchronous operations on [`DefaultObjectAccessControl`](crate::default_object_access_control::DefaultObjectAccessControl)s.
    pub fn default_object_access_control(&self) -> DefaultObjectAccessControlClient<'_> {
        DefaultObjectAccessControlClient(self)
    }

    /// Synchronous operations on [`HmacKey`](crate::hmac_key::HmacKey)s.
    pub fn hmac_key(&self) -> HmacKeyClient<'_> {
        HmacKeyClient(self)
    }

    /// Synchronous operations on [`Object`](crate::object::Object)s.
    pub fn object(&self) -> ObjectClient<'_> {
        ObjectClient(self)
    }

    /// Synchronous operations on [`ObjectAccessControl`](crate::object_access_control::ObjectAccessControl)s.
    pub fn object_access_control(&self) -> ObjectAccessControlClient<'_> {
        ObjectAccessControlClient(self)
    }
}

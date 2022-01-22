//! Synchronous clients for Google Cloud Storage endpoints.

mod bucket;
mod bucket_access_control;
mod default_object_access_control;
mod hmac_key;
mod object;
mod object_access_control;

mod helpers; // for internal use only

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
    /// Constructs a client with the default token provider, where it attemps to obtain the
    /// credentials from the following locations:
    ///
    /// 1. Checks for the environment variable `SERVICE_ACCOUNT`, and if it exists, reads the file
    /// at the path specified there as a credentials json file.
    /// 2. It attemps to do the same with the `GOOGLE_APPLICATION_CREDENTIALS` var.
    /// 3. It reads the `SERVICE_ACCOUNT_JSON` environment variable directly as json and uses that
    /// 4.It attemps to do the same with the `GOOGLE_APPLICATION_CREDENTIALS_JSON` var.
    pub fn new() -> crate::Result<Self> {
        Ok(Self {
            runtime: crate::runtime()?,
            client: crate::Client::default(),
        })
    }

    /// Initializer with a provided refreshable token
    pub fn with_cache(token_cache: impl crate::TokenCache + 'static) -> crate::Result<Self> {
        Ok(Self {
            runtime: crate::runtime()?,
            client: crate::Client::with_cache(token_cache),
        })
    }

    /// Synchronous operations on [`Bucket`](crate::bucket::Bucket)s.
    pub fn bucket(&self) -> BucketClient {
        BucketClient(self)
    }

    /// Synchronous operations on [`BucketAccessControl`](crate::bucket_access_control::BucketAccessControl)s.
    pub fn bucket_access_control(&self) -> BucketAccessControlClient {
        BucketAccessControlClient(self)
    }

    /// Synchronous operations on [`DefaultObjectAccessControl`](crate::default_object_access_control::DefaultObjectAccessControl)s.
    pub fn default_object_access_control(&self) -> DefaultObjectAccessControlClient {
        DefaultObjectAccessControlClient(self)
    }

    /// Synchronous operations on [`HmacKey`](crate::hmac_key::HmacKey)s.
    pub fn hmac_key(&self) -> HmacKeyClient {
        HmacKeyClient(self)
    }

    /// Synchronous operations on [`Object`](crate::object::Object)s.
    pub fn object(&self) -> ObjectClient {
        ObjectClient(self)
    }

    /// Synchronous operations on [`ObjectAccessControl`](crate::object_access_control::ObjectAccessControl)s.
    pub fn object_access_control(&self) -> ObjectAccessControlClient {
        ObjectAccessControlClient(self)
    }
}

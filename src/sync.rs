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

use crate::token::{RefreshableToken, Token};

/// The primary synchronous entrypoint to perform operations with Google Cloud Storage.
#[derive(Debug)]
pub struct Client<R: RefreshableToken> {
    runtime: tokio::runtime::Runtime,
    client: crate::client::Client<R>,
}

impl Client<Token> {
    /// Constructs a synchronous client
    pub fn new() -> crate::Result<Self> {
        Ok(Self {
            runtime: crate::runtime()?,
            client: Default::default(),
        })
    }
}

impl<R> Client<R>
where
    R: RefreshableToken,
{
    /// Initializer with a provided refreshable token
    pub fn with_token_cache(token_cache: R) -> crate::Result<Self> {
        Ok(Self {
            runtime: crate::runtime()?,
            client: crate::Client::with_token_cache(token_cache),
        })
    }
    /// Synchronous operations on [`Bucket`](crate::bucket::Bucket)s.
    pub fn bucket(&self) -> BucketClient<'_, R> {
        BucketClient(self)
    }

    /// Synchronous operations on [`BucketAccessControl`](crate::bucket_access_control::BucketAccessControl)s.
    pub fn bucket_access_control(&self) -> BucketAccessControlClient<'_, R> {
        BucketAccessControlClient(self)
    }

    /// Synchronous operations on [`DefaultObjectAccessControl`](crate::default_object_access_control::DefaultObjectAccessControl)s.
    pub fn default_object_access_control(&self) -> DefaultObjectAccessControlClient<'_, R> {
        DefaultObjectAccessControlClient(self)
    }

    /// Synchronous operations on [`HmacKey`](crate::hmac_key::HmacKey)s.
    pub fn hmac_key(&self) -> HmacKeyClient<'_, R> {
        HmacKeyClient(self)
    }

    /// Synchronous operations on [`Object`](crate::object::Object)s.
    pub fn object(&self) -> ObjectClient<'_, R> {
        ObjectClient(self)
    }

    /// Synchronous operations on [`ObjectAccessControl`](crate::object_access_control::ObjectAccessControl)s.
    pub fn object_access_control(&self) -> ObjectAccessControlClient<'_, R> {
        ObjectAccessControlClient(self)
    }
}

use crate::Error;

use super::{BucketClient, BucketAccessControlClient, DefaultObjectAccessControlClient, HmacKeyClient, ObjectClient, ObjectAccessControlClient};

/// The primary synchronous entrypoint to perform operations with Google Cloud Storage.
#[derive(Debug)]
pub struct Client {
    runtime: tokio::runtime::Runtime,
    client: crate::client::Client,
}

impl Client {
    /// Constructs a client with the default token provider, where it attemps to obtain the credentials from the following locations:
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            runtime: crate::runtime()?,
            client: crate::Client::default(),
        })
    }

    /// Initializer with a provided refreshable token
    pub fn with_cache(token_cache: impl crate::TokenCache + 'static) -> Result<Self, Error> {
        Ok(Self {
            runtime: crate::runtime()?,
            client: crate::Client::with_cache(token_cache),
        })
    }

    /// Synchronous operations on [`Bucket`](crate::bucket::Bucket)s.
    pub fn bucket(&self) -> BucketClient {
        let handle = self.runtime.handle().to_owned();
        let client = self.client.bucket();
        BucketClient {
            runtime: handle,
            client: &client,
        }
    }

    /// Synchronous operations on [`BucketAccessControl`](crate::bucket_access_control::BucketAccessControl)s.
    pub fn bucket_access_control(&self, bucket: &str) -> BucketAccessControlClient {
        BucketAccessControlClient {
            client: &self.client.bucket_access_control(bucket),
            runtime: self.runtime.handle()
        }
    }

    /// Synchronous operations on [`DefaultObjectAccessControl`](crate::default_object_access_control::DefaultObjectAccessControl)s.
    pub fn default_object_access_control(&self, bucket: &str) -> DefaultObjectAccessControlClient {
        DefaultObjectAccessControlClient {
            client: &self.client.default_object_access_control(bucket),
            runtime: self.runtime.handle()
        }
    }

    /// Synchronous operations on [`HmacKey`](crate::hmac_key::HmacKey)s.
    pub fn hmac_key(&self) -> HmacKeyClient {
        HmacKeyClient {
            client: &self.client.hmac_key(),
            runtime: self.runtime.handle()
        }
    }

    /// Synchronous operations on [`Object`](crate::object::Object)s.
    pub fn object(&self) -> ObjectClient {
        ObjectClient {
            client: &self.client.object(),
            runtime: self.runtime.handle()
        }
    }

    /// Synchronous operations on [`ObjectAccessControl`](crate::object_access_control::ObjectAccessControl)s.
    pub fn object_access_control(&self, bucket: &str, object: &str) -> ObjectAccessControlClient {
        ObjectAccessControlClient {
            client: &self.client.object_access_control(bucket, object),
            runtime: self.runtime.handle()
        }
    }
}

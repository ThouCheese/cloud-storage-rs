use crate::Error;

use super::{BucketClient, BucketAccessControlClient, DefaultObjectAccessControlClient, HmacKeyClient, ObjectClient, ObjectAccessControlClient};

/// The primary synchronous entrypoint to perform operations with Google Cloud Storage.
#[derive(Debug)]
pub struct CloudStorageClient {
    runtime: tokio::runtime::Runtime,
    client: crate::client::CloudStorageClient,
}

impl CloudStorageClient {
    /// Constructs a client with the default token provider, where it attemps to obtain the credentials from the following locations:
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            runtime: crate::runtime()?,
            client: crate::CloudStorageClient::default(),
        })
    }

    /// Initializer with a provided refreshable token
    pub fn with_cache(token_cache: impl crate::TokenCache + 'static) -> Result<Self, Error> {
        Ok(Self {
            runtime: crate::runtime()?,
            client: crate::CloudStorageClient::with_cache(token_cache),
        })
    }

    /// Synchronous operations on [`Bucket`](crate::Bucket)s.
    pub fn bucket(&self) -> BucketClient {
        BucketClient {
            client: self.client.bucket(),
            runtime: self.runtime.handle(),
        }
    }

    /// Synchronous operations on [`BucketAccessControl`](crate::models::BucketAccessControl)s.
    pub fn bucket_access_control(&self, bucket: &str) -> BucketAccessControlClient {
        BucketAccessControlClient {
            client: self.client.bucket_access_control(bucket),
            runtime: self.runtime.handle()
        }
    }

    /// Synchronous operations on [`DefaultObjectAccessControl`](crate::models::DefaultObjectAccessControl)s.
    pub fn default_object_access_control(&self, bucket: &str) -> DefaultObjectAccessControlClient {
        DefaultObjectAccessControlClient {
            client: self.client.default_object_access_control(bucket),
            runtime: self.runtime.handle()
        }
    }

    /// Synchronous operations on [`HmacKey`](crate::models::HmacKey)s.
    pub fn hmac_key(&self) -> HmacKeyClient {
        HmacKeyClient {
            client: self.client.hmac_key(),
            runtime: self.runtime.handle()
        }
    }

    /// Synchronous operations on [`Object`](crate::models::Object)s.
    pub fn object(&self, bucket: &str) -> ObjectClient {
        ObjectClient {
            client: self.client.object(bucket),
            runtime: self.runtime.handle()
        }
    }

    /// Synchronous operations on [`ObjectAccessControl`](crate::models::ObjectAccessControl)s.
    pub fn object_access_control(&self, bucket: &str, object: &str) -> ObjectAccessControlClient {
        ObjectAccessControlClient {
            client: self.client.object_access_control(bucket, object),
            runtime: self.runtime.handle()
        }
    }
}

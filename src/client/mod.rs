//! Clients for Google Cloud Storage endpoints.
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

use std::{fmt, sync};
use crate::{Error, token::TokenCache, ServiceAccount};


/// The primary entrypoint to perform operations with Google Cloud Storage.
pub struct CloudStorageClient {
    pub(crate) reqwest: reqwest::Client,
    pub(crate) service_account: crate::ServiceAccount,
    /// Static `Token` struct that caches
    pub(crate) token_cache: sync::Arc<dyn TokenCache>,
}

impl fmt::Debug for CloudStorageClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CloudStorageClient")
            .field("client", &self.reqwest)
            .field("token_cache", &"<opaque>")
            .finish()
    }
}

impl Default for CloudStorageClient {
    fn default() -> Self {
        Self {
            reqwest: Default::default(),
            token_cache: sync::Arc::new(crate::Token::default()),
            service_account: crate::ServiceAccount::default()
        }
    }
}

impl CloudStorageClient {
    /// Constucts a client with given reqwest client
    pub fn with_client(client: reqwest::Client) -> Self {
        Self {
            reqwest: client,
            token_cache: sync::Arc::new(crate::Token::default()),
            service_account: crate::ServiceAccount::default()
        }
    }

    /// Initializer with a provided refreshable token
    pub fn with_cache(token: impl TokenCache + 'static) -> Self {
        Self {
            reqwest: Default::default(),
            token_cache: sync::Arc::new(token),
            service_account: crate::ServiceAccount::default()
        }
    }

    /// Creates a new [`CloudStorageClientBuilder`](crate::CloudStorageClientBuilder)
    pub fn builder() -> CloudStorageClientBuilder {
        CloudStorageClientBuilder::new()
    }

    /// Operations on [`Bucket`](crate::Bucket)s.
    pub fn bucket(&self) -> BucketClient {
        BucketClient {
            bucket_url: "https://storage.googleapis.com/storage/v1/b".to_string(),
            project_id: self.service_account.project_id.clone(),
            client: self,
        }
    }

    /// Operations on [`BucketAccessControl`](crate::models::BucketAccessControl)s.
    pub fn bucket_access_control(&self, bucket: &str) -> BucketAccessControlClient {
        let url = format!("https://storage.googleapis.com/storage/v1/b/{}/acl", crate::percent_encode(bucket));
        BucketAccessControlClient {
            bucket_acl_url: url,
            client: self
        }
    }

    /// Operations on [`DefaultObjectAccessControl`](crate::models::DefaultObjectAccessControl)s.
    pub fn default_object_access_control(&self, bucket: &str) -> DefaultObjectAccessControlClient {
        let url = format!("https://storage.googleapis.com/storage/v1/b/{}/defaultObjectAcl", crate::percent_encode(bucket));
        DefaultObjectAccessControlClient {
            base_url: url,
            bucket: bucket.to_string(),
            client: self
        }
    }

    /// Operations on [`HmacKey`](crate::models::HmacKey)s.
    pub fn hmac_key(&self) -> HmacKeyClient {
        HmacKeyClient {
            hmac_keys_url: format!("https://storage.googleapis.com/storage/v1/projects/{}/hmacKeys", &self.service_account.project_id),
            client_email: self.service_account.client_email.clone(),
            client: self,
        }
    }

    /// Operations on [`Object`](crate::models::Object)s.
    pub fn object(&self, bucket: &str) -> ObjectClient {
        ObjectClient {
            base_url: format!("https://storage.googleapis.com/storage/v1/b/{}/o", crate::percent_encode(bucket)),
            insert_url: format!("https://storage.googleapis.com/upload/storage/v1/b/{}/o", crate::percent_encode(bucket)),
            client: self,
        }
    }

    /// Operations on [`ObjectAccessControl`](crate::models::ObjectAccessControl)s.
    pub fn object_access_control(&self, bucket: &str, object: &str,) -> ObjectAccessControlClient {
        ObjectAccessControlClient {
            acl_url: format!("https://storage.googleapis.com/storage/v1/b/{}/o/{}/acl", crate::percent_encode(bucket), crate::percent_encode(object)),
            client: self
        }
    }

    pub(crate) async fn get_headers(&self) -> Result<reqwest::header::HeaderMap, Error> {
        let mut result = reqwest::header::HeaderMap::new();
        let token = self.token_cache.get(&self.reqwest, self.service_account.client_email.clone(), self.service_account.private_key.as_bytes()).await?;
        result.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", token).parse().unwrap(),
        );
        Ok(result)
    }
}

/// A [`CloudStorageClientBuilder`] can be used to create a [`CloudStorageClient`] with custom configuration.
#[derive(Default)]
pub struct CloudStorageClientBuilder {
    client: Option<reqwest::Client>,
    /// Static `Token` struct that caches
    token_cache: Option<sync::Arc<dyn crate::TokenCache>>,
    service_account: Option<ServiceAccount>
}

impl CloudStorageClientBuilder {
    /// Constructs a new ClientBuilder
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns a `Client` that uses this `ClientBuilder` configuration.
    pub fn build(self) -> CloudStorageClient {
        CloudStorageClient {
            reqwest: self.client.unwrap_or_default(),
            token_cache: self.token_cache.unwrap_or(sync::Arc::new(crate::Token::default())),
            service_account: self.service_account.unwrap_or_default()
        }
    }

    /// Sets refreshable token
    pub fn with_cache(&mut self, token: impl TokenCache + 'static) -> &mut Self {
        self.token_cache = Some(sync::Arc::new(token));
        self
    }

    /// Sets service account
    pub fn with_service_account(&mut self, service_account: crate::ServiceAccount) -> &mut Self {
        self.service_account = Some(service_account);
        self
    }

    /// Sets internal [reqwest Client](https://docs.rs/reqwest/latest/reqwest/struct.Client.html)
    pub fn with_reqwest_client(&mut self, reqwest_client: reqwest::Client) -> &mut Self {
        self.client = Some(reqwest_client);
        self
    }
}

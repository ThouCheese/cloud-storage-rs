//! Clients for Google Cloud Storage endpoints.

use std::{fmt, sync};

use crate::token::TokenCache;

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

/// The primary entrypoint to perform operations with Google Cloud Storage.
pub struct Client {
    client: reqwest::Client,
    /// Static `Token` struct that caches
    token_cache: sync::Arc<dyn crate::TokenCache + Send>,
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client")
            .field("client", &self.client)
            .field("token_cache", &"<opaque>")
            .finish()
    }
}

impl Default for Client {
    fn default() -> Self {
        Self {
            client: Default::default(),
            token_cache: sync::Arc::new(crate::Token::default()),
        }
    }
}

impl Client {
    /// Constructs a client with the default token provider, where it attemps to obtain the
    /// credentials from the following locations:
    ///
    /// 1. Checks for the environment variable `SERVICE_ACCOUNT`, and if it exists, reads the file
    /// at the path specified there as a credentials json file.
    /// 2. It attemps to do the same with the `GOOGLE_APPLICATION_CREDENTIALS` var.
    /// 3. It reads the `SERVICE_ACCOUNT_JSON` environment variable directly as json and uses that
    /// 4. It attemps to do the same with the `GOOGLE_APPLICATION_CREDENTIALS_JSON` var.
    pub fn new() -> Self {
        Default::default()
    }

    /// Initializer with a provided refreshable token
    pub fn with_cache(token: impl TokenCache + Send + 'static) -> Self {
        Self {
            client: Default::default(),
            token_cache: sync::Arc::new(token),
        }
    }

    /// Creates a new [ClientBuilder]
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    /// Operations on [`Bucket`](crate::bucket::Bucket)s.
    pub fn bucket(&self) -> BucketClient<'_> {
        BucketClient(self)
    }

    /// Operations on [`BucketAccessControl`](crate::bucket_access_control::BucketAccessControl)s.
    pub fn bucket_access_control(&self) -> BucketAccessControlClient<'_> {
        BucketAccessControlClient(self)
    }

    /// Operations on [`DefaultObjectAccessControl`](crate::default_object_access_control::DefaultObjectAccessControl)s.
    pub fn default_object_access_control(&self) -> DefaultObjectAccessControlClient<'_> {
        DefaultObjectAccessControlClient(self)
    }

    /// Operations on [`HmacKey`](crate::hmac_key::HmacKey)s.
    pub fn hmac_key(&self) -> HmacKeyClient<'_> {
        HmacKeyClient(self)
    }

    /// Operations on [`Object`](crate::object::Object)s.
    pub fn object(&self) -> ObjectClient<'_> {
        ObjectClient(self)
    }

    /// Operations on [`ObjectAccessControl`](crate::object_access_control::ObjectAccessControl)s.
    pub fn object_access_control(&self) -> ObjectAccessControlClient<'_> {
        ObjectAccessControlClient(self)
    }

    async fn get_headers(&self) -> crate::Result<reqwest::header::HeaderMap> {
        let mut result = reqwest::header::HeaderMap::new();
        let token = self.token_cache.get(&self.client).await?;
        result.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", token).parse().unwrap(),
        );
        Ok(result)
    }
}

/// A ClientBuilder can be used to create a Client with custom configuration.
#[derive(Default)]
pub struct ClientBuilder {
    client: Option<reqwest::Client>,
    /// Static `Token` struct that caches
    token_cache: Option<sync::Arc<dyn crate::TokenCache + Send>>,
}

impl ClientBuilder {
    /// Constructs a new ClientBuilder
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns a `Client` that uses this `ClientBuilder` configuration.
    pub fn build(self) -> Client {
        Client {
            client: self.client.unwrap_or_default(),
            token_cache: self
                .token_cache
                .unwrap_or(sync::Arc::new(crate::Token::default())),
        }
    }

    /// Sets refreshable token
    pub fn with_cache(self, token: impl TokenCache + Send + 'static) -> Self {
        ClientBuilder {
            token_cache: Some(sync::Arc::new(token)),
            ..self
        }
    }

    /// Sets internal [reqwest Client](https://docs.rs/reqwest/latest/reqwest/struct.Client.html)
    pub fn with_reqwest_client(self, reqwest_client: reqwest::Client) -> Self {
        ClientBuilder {
            client: Some(reqwest_client),
            ..self
        }
    }
}

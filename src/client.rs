//! Clients for Google Cloud Storage endpoints.

use std::fmt;

use crate::token::{Token, TokenCache};

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
pub struct Client<R: TokenCache> {
    client: reqwest::Client,

    /// Static `Token` struct that caches
    token_cache: R,
}

impl<R> fmt::Debug for Client<R>
where
    R: TokenCache,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client")
            .field("client", &self.client)
            .field("token_cache", &"<opaque>")
            .finish()
    }
}

impl Default for Client<Token> {
    fn default() -> Self {
        Self {
            client: Default::default(),
            token_cache: Token::new("https://www.googleapis.com/auth/devstorage.full_control"),
        }
    }
}

impl Client<Token> {
    /// Constructs a client
    pub fn new() -> Self {
        Default::default()
    }
}

impl<R> Client<R>
where
    R: TokenCache,
{
    /// Initializer with a provided refreshable token
    pub fn with_token_cache(token_cache: R) -> Self {
        Self {
            client: Default::default(),
            token_cache,
        }
    }

    /// Operations on [`Bucket`](crate::bucket::Bucket)s.
    pub fn bucket(&self) -> BucketClient<'_, R> {
        BucketClient(self)
    }

    /// Operations on [`BucketAccessControl`](crate::bucket_access_control::BucketAccessControl)s.
    pub fn bucket_access_control(&self) -> BucketAccessControlClient<'_, R> {
        BucketAccessControlClient(self)
    }

    /// Operations on [`DefaultObjectAccessControl`](crate::default_object_access_control::DefaultObjectAccessControl)s.
    pub fn default_object_access_control(&self) -> DefaultObjectAccessControlClient<'_, R> {
        DefaultObjectAccessControlClient(self)
    }

    /// Operations on [`HmacKey`](crate::hmac_key::HmacKey)s.
    pub fn hmac_key(&self) -> HmacKeyClient<'_, R> {
        HmacKeyClient(self)
    }

    /// Operations on [`Object`](crate::object::Object)s.
    pub fn object(&self) -> ObjectClient<'_, R> {
        ObjectClient(self)
    }

    /// Operations on [`ObjectAccessControl`](crate::object_access_control::ObjectAccessControl)s.
    pub fn object_access_control(&self) -> ObjectAccessControlClient<'_, R> {
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

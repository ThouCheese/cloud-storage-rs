use crate::{Error, models::{HmacKey, HmacMeta, HmacState}};

/// Operations on [`HmacKey`](HmacKey)s.
#[derive(Debug)]
pub struct HmacKeyClient<'a> {
    pub(crate) client: crate::client::HmacKeyClient<'a>,
    pub(crate) runtime: &'a tokio::runtime::Handle,
}

impl<'a> HmacKeyClient<'a> {
    /// Creates a new HMAC key for the specified service account.
    ///
    /// The authenticated user must have `storage.hmacKeys.create` permission for the project in
    /// which the key will be created.
    ///
    /// For general information about HMAC keys in Cloud Storage, see
    /// [HMAC Keys](https://cloud.google.com/storage/docs/authentication/hmackeys).
    /// ### Example
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::sync::CloudStorageClient;
    /// # use cloud_storage::models::HmacKey;
    ///
    /// let client = CloudStorageClient::new()?;
    /// let hmac_key = client.hmac_key().create()?;
    /// # use cloud_storage::models::HmacState;
    /// # client.hmac_key().update(&hmac_key.metadata.access_id, HmacState::Inactive)?;
    /// # client.hmac_key().delete(&hmac_key.metadata.access_id)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create(&self) -> Result<HmacKey, Error> {
        self.runtime.block_on(self.client.create())
    }

    /// Retrieves a list of HMAC keys matching the criteria. Since the HmacKey is secret, this does
    /// not return a `HmacKey`, but a `HmacMeta`. This is a redacted version of a `HmacKey`, but
    /// with the secret data omitted.
    ///
    /// The authenticated user must have `storage.hmacKeys.list` permission for the project in which
    /// the key exists.
    ///
    /// For general information about HMAC keys in Cloud Storage, see
    /// [HMAC Keys](https://cloud.google.com/storage/docs/authentication/hmackeys).
    /// ### Example
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::sync::CloudStorageClient;
    /// # use cloud_storage::models::HmacKey;
    ///
    /// let client = CloudStorageClient::new()?;
    /// let all_hmac_keys = client.hmac_key().list()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> Result<Vec<HmacMeta>, Error> {
        self.runtime.block_on(self.client.list())
    }

    /// Retrieves an HMAC key's metadata. Since the HmacKey is secret, this does not return a
    /// `HmacKey`, but a `HmacMeta`. This is a redacted version of a `HmacKey`, but with the secret
    /// data omitted.
    ///
    /// The authenticated user must have `storage.hmacKeys.get` permission for the project in which
    /// the key exists.
    ///
    /// For general information about HMAC keys in Cloud Storage, see
    /// [HMAC Keys](https://cloud.google.com/storage/docs/authentication/hmackeys).
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::sync::CloudStorageClient;
    /// # use cloud_storage::models::HmacKey;
    ///
    /// let client = CloudStorageClient::new()?;
    /// let key = client.hmac_key().read("some identifier")?;
    /// # Ok(())
    /// # }
    pub fn read(&self, access_id: &str) -> Result<HmacMeta, Error> {
        self.runtime
            .block_on(self.client.read(access_id))
    }

    /// Updates the state of an HMAC key. See the HMAC Key resource descriptor for valid states.
    /// Since the HmacKey is secret, this does not return a `HmacKey`, but a `HmacMeta`. This is a
    /// redacted version of a `HmacKey`, but with the secret data omitted.
    ///
    /// The authenticated user must have `storage.hmacKeys.update` permission for the project in
    /// which the key exists.
    ///
    /// For general information about HMAC keys in Cloud Storage, see
    /// [HMAC Keys](https://cloud.google.com/storage/docs/authentication/hmackeys).
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::sync::CloudStorageClient;
    /// # use cloud_storage::models::{HmacKey, HmacState};
    ///
    /// let client = CloudStorageClient::new()?;
    /// let key = client.hmac_key().update("your key", HmacState::Active)?;
    /// # Ok(())
    /// # }
    pub fn update(&self, access_id: &str, state: HmacState) -> Result<HmacMeta, Error> {
        self.runtime
            .block_on(self.client.update(access_id, state))
    }

    /// Deletes an HMAC key. Note that a key must be set to `Inactive` first.
    ///
    /// The authenticated user must have storage.hmacKeys.delete permission for the project in which
    /// the key exists.
    ///
    /// For general information about HMAC keys in Cloud Storage, see
    /// [HMAC Keys](https://cloud.google.com/storage/docs/authentication/hmackeys).
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::sync::CloudStorageClient;
    /// # use cloud_storage::models::{HmacKey, HmacState};
    ///
    /// let client = CloudStorageClient::new()?;
    /// let key = client.hmac_key().update("your key", HmacState::Inactive)?; // this is required.
    /// client.hmac_key().delete(&key.access_id)?;
    /// # Ok(())
    /// # }
    pub fn delete(&self, access_id: &str) -> Result<(), Error> {
        self.runtime
            .block_on(self.client.delete(access_id))
    }
}

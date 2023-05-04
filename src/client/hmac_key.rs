use crate::{Error, models::{HmacKey, HmacMeta, Response, ListResponse, HmacState, UpdateHmacMetadata}};

/// Operations on [`HmacKey`](HmacKey)s.
#[derive(Debug)]
pub struct HmacKeyClient<'a> {
    pub(crate) client: &'a super::CloudStorageClient,
    pub(crate) hmac_keys_url: String,
    pub(crate) client_email: String,
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
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::CloudStorageClient;
    /// # use cloud_storage::models::HmacKey;
    /// let cloud_storage_client = CloudStorageClient::default();
    /// let client = cloud_storage_client.hmac_key();
    /// let hmac_key = client.create().await?;
    /// # use cloud_storage::models::HmacState;
    /// # client.update(&hmac_key.metadata.access_id, HmacState::Inactive).await?;
    /// # client.delete(&hmac_key.metadata.access_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self) -> Result<HmacKey, Error> {
        use reqwest::header::CONTENT_LENGTH;

        let query = [("serviceAccountEmail", &self.client_email)];
        let mut headers = self.client.get_headers().await?;
        headers.insert(CONTENT_LENGTH, 0.into());
        let result: crate::models::Response<HmacKey> = self.client.reqwest
            .post(&self.hmac_keys_url)
            .headers(headers)
            .query(&query)
            .send()
            .await?
            .json()
            .await?;
        Ok(result?)
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
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::CloudStorageClient;
    /// # use cloud_storage::models::HmacKey;
    ///
    /// let client = CloudStorageClient::default();
    /// let all_hmac_keys = client.hmac_key().list().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self) -> Result<Vec<HmacMeta>, Error> {
        let response = self.client.reqwest
            .get(&self.hmac_keys_url)
            .headers(self.client.get_headers().await?)
            .send()
            .await?
            .text()
            .await?;
        let result: Result<Response<ListResponse<HmacMeta>>, serde_json::Error> = serde_json::from_str(&response);
        let single_result: Result<Response<HmacMeta>, serde_json::Error> = serde_json::from_str(&response);
        // todo: test this with one hmac key

        // This function rquires more complicated error handling because when there is only one
        // entry, Google will return the response `{ "kind": "storage#hmacKeysMetadata" }` instead
        // of a list with one element. This breaks the parser.
        match result {
            Ok(parsed) => match parsed {
                crate::models::Response::Success(s) => Ok(s.items),
                crate::models::Response::Error(e) => Err(e.into()),
            },
            Err(_) => Ok(vec![single_result??]),
        }
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
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::CloudStorageClient;
    /// # use cloud_storage::models::HmacKey;
    ///
    /// let client = CloudStorageClient::default();
    /// let key = client.hmac_key().read("some identifier").await?;
    /// # Ok(())
    /// # }
    pub async fn read(&self, access_id: &str) -> Result<HmacMeta, Error> {
        let url = format!("{}/{}",self.hmac_keys_url,access_id);
        let result: crate::models::Response<HmacMeta> = self.client.reqwest
            .get(&url)
            .headers(self.client.get_headers().await?)
            .send()
            .await?
            .json()
            .await?;
        Ok(result?)
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
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::CloudStorageClient;
    /// # use cloud_storage::models::{HmacKey, HmacState};
    ///
    /// let client = CloudStorageClient::default();
    /// let key = client.hmac_key().update("your key", HmacState::Active).await?;
    /// # Ok(())
    /// # }
    pub async fn update(&self, access_id: &str, state: HmacState) -> Result<HmacMeta, Error> {
        let url = format!(
            "{}/{}",
            self.hmac_keys_url,
            access_id
        );
        serde_json::to_string(&UpdateHmacMetadata { state })?;
        let result: Response<HmacMeta> = self.client.reqwest
            .put(&url)
            .headers(self.client.get_headers().await?)
            .json(&UpdateHmacMetadata { state })
            .send()
            .await?
            .json()
            .await?;
        Ok(result?)
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
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::CloudStorageClient;
    /// # use cloud_storage::models::{HmacKey, HmacState};
    ///
    /// let client = CloudStorageClient::default();
    /// let key = client.hmac_key().update("your key", HmacState::Inactive).await?; // this is required.
    /// client.hmac_key().delete(&key.access_id).await?;
    /// # Ok(())
    /// # }
    pub async fn delete(&self, access_id: &str) -> Result<(), Error> {
        let url = format!(
            "{}/{}",
            self.hmac_keys_url,
            access_id
        );
        let response = self.client.reqwest
            .delete(&url)
            .headers(self.client.get_headers().await?)
            .send()
            .await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(crate::Error::Google(response.json().await?))
        }
    }
}

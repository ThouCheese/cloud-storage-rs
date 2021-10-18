use crate::{
    error::GoogleResponse,
    hmac_key::{HmacKey, HmacMeta, HmacState},
};

/// Operations on [`HmacKey`](HmacKey)s.
#[derive(Debug)]
pub struct HmacKeyClient<'a>(pub(super) &'a super::Client);

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
    /// use cloud_storage::Client;
    /// use cloud_storage::hmac_key::HmacKey;
    ///
    /// let client = Client::default();
    /// let hmac_key = client.hmac_key().create().await?;
    /// # use cloud_storage::hmac_key::HmacState;
    /// # client.hmac_key().update(&hmac_key.metadata.access_id, HmacState::Inactive).await?;
    /// # client.hmac_key().delete(&hmac_key.metadata.access_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self) -> crate::Result<HmacKey> {
        use reqwest::header::CONTENT_LENGTH;

        let url = format!(
            "{}/projects/{}/hmacKeys",
            crate::BASE_URL,
            crate::SERVICE_ACCOUNT.project_id
        );
        let query = [("serviceAccountEmail", &crate::SERVICE_ACCOUNT.client_email)];
        let mut headers = self.0.get_headers().await?;
        headers.insert(CONTENT_LENGTH, 0.into());
        let result: GoogleResponse<HmacKey> = self
            .0
            .client
            .post(&url)
            .headers(headers)
            .query(&query)
            .send()
            .await?
            .json()
            .await?;
        match result {
            GoogleResponse::Success(s) => Ok(s),
            GoogleResponse::Error(e) => Err(e.into()),
        }
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
    /// use cloud_storage::Client;
    /// use cloud_storage::hmac_key::HmacKey;
    ///
    /// let client = Client::default();
    /// let all_hmac_keys = client.hmac_key().list().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self) -> crate::Result<Vec<HmacMeta>> {
        let url = format!(
            "{}/projects/{}/hmacKeys",
            crate::BASE_URL,
            crate::SERVICE_ACCOUNT.project_id
        );
        let response = self
            .0
            .client
            .get(&url)
            .headers(self.0.get_headers().await?)
            .send()
            .await?
            .text()
            .await?;
        let result: Result<GoogleResponse<crate::hmac_key::ListResponse>, _> =
            serde_json::from_str(&response);

        // This function rquires more complicated error handling because when there is only one
        // entry, Google will return the response `{ "kind": "storage#hmacKeysMetadata" }` instead
        // of a list with one element. This breaks the parser.
        match result {
            Ok(parsed) => match parsed {
                GoogleResponse::Success(s) => Ok(s.items),
                GoogleResponse::Error(e) => Err(e.into()),
            },
            Err(_) => Ok(vec![]),
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
    /// use cloud_storage::Client;
    /// use cloud_storage::hmac_key::HmacKey;
    ///
    /// let client = Client::default();
    /// let key = client.hmac_key().read("some identifier").await?;
    /// # Ok(())
    /// # }
    pub async fn read(&self, access_id: &str) -> crate::Result<HmacMeta> {
        let url = format!(
            "{}/projects/{}/hmacKeys/{}",
            crate::BASE_URL,
            crate::SERVICE_ACCOUNT.project_id,
            access_id
        );
        let result: GoogleResponse<HmacMeta> = self
            .0
            .client
            .get(&url)
            .headers(self.0.get_headers().await?)
            .send()
            .await?
            .json()
            .await?;
        match result {
            GoogleResponse::Success(s) => Ok(s),
            GoogleResponse::Error(e) => Err(e.into()),
        }
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
    /// use cloud_storage::Client;
    /// use cloud_storage::hmac_key::{HmacKey, HmacState};
    ///
    /// let client = Client::default();
    /// let key = client.hmac_key().update("your key", HmacState::Active).await?;
    /// # Ok(())
    /// # }
    pub async fn update(&self, access_id: &str, state: HmacState) -> crate::Result<HmacMeta> {
        let url = format!(
            "{}/projects/{}/hmacKeys/{}",
            crate::BASE_URL,
            crate::SERVICE_ACCOUNT.project_id,
            access_id
        );
        serde_json::to_string(&crate::hmac_key::UpdateMeta { state })?;
        let result: GoogleResponse<HmacMeta> = self
            .0
            .client
            .put(&url)
            .headers(self.0.get_headers().await?)
            .json(&crate::hmac_key::UpdateMeta { state })
            .send()
            .await?
            .json()
            .await?;
        match result {
            GoogleResponse::Success(s) => Ok(s),
            GoogleResponse::Error(e) => Err(e.into()),
        }
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
    /// use cloud_storage::Client;
    /// use cloud_storage::hmac_key::{HmacKey, HmacState};
    ///
    /// let client = Client::default();
    /// let key = client.hmac_key().update("your key", HmacState::Inactive).await?; // this is required.
    /// client.hmac_key().delete(&key.access_id).await?;
    /// # Ok(())
    /// # }
    pub async fn delete(&self, access_id: &str) -> crate::Result<()> {
        let url = format!(
            "{}/projects/{}/hmacKeys/{}",
            crate::BASE_URL,
            crate::SERVICE_ACCOUNT.project_id,
            access_id
        );
        let response = self
            .0
            .client
            .delete(&url)
            .headers(self.0.get_headers().await?)
            .send()
            .await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(crate::Error::Google(response.json().await?))
        }
    }
}

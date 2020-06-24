use crate::error::GoogleResponse;

/// The `HmacKey` resource represents an HMAC key within Cloud Storage. The resource consists of a
/// secret and `HmacMeta`. HMAC keys can be used as credentials for service accounts. For more
/// information, see HMAC Keys.
///
/// Note that the `HmacKey` resource is only returned when you use `HmacKey::create`. Other
/// methods, such as `HmacKey::read`, return the metadata portion of the HMAC key resource.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HmacKey {
    /// The kind of item this is. For HMAC keys, this is always `storage#hmacKey`.
    pub kind: String,
    /// HMAC key metadata.
    pub metadata: HmacMeta,
    /// HMAC secret key material.
    pub secret: String,
}

/// Contains information about an Hmac Key.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HmacMeta {
    /// The kind of item this is. For HMAC key metadata, this is always `storage#hmacKeyMetadata`.
    pub kind: String,
    /// The ID of the HMAC key, including the Project ID and the Access ID.
    pub id: String,
    /// The link to this resource.
    pub self_link: String,
    /// The access ID of the HMAC Key.
    pub access_id: String,
    /// The Project ID of the project that owns the service account to which the key authenticates.
    pub project_id: String,
    /// The email address of the key's associated service account.
    pub service_account_email: String,
    /// The state of the key.
    pub state: HmacState,
    /// The creation time of the HMAC key.
    pub time_created: chrono::DateTime<chrono::Utc>,
    /// The last modification time of the HMAC key metadata.
    pub updated: chrono::DateTime<chrono::Utc>,
    /// HTTP 1.1 Entity tag for the HMAC key.
    pub etag: String,
}

/// The state of an Hmac Key.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HmacState {
    /// This Hmac key is currently used.
    Active,
    /// This Hmac key has been set to inactive.
    Inactive,
    /// This Hmac key has been permanently deleted.
    Deleted,
}

#[derive(serde::Deserialize)]
struct ListResponse {
    items: Vec<HmacMeta>,
}

#[derive(serde::Serialize)]
struct UpdateRequest {
    secret: String,
    metadata: UpdateMeta,
}

#[derive(serde::Serialize)]
struct UpdateMeta {
    state: HmacState,
}

impl HmacKey {
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
    /// use cloud_storage::hmac_key::HmacKey;
    ///
    /// let hmac_key = HmacKey::create()?;
    /// # use cloud_storage::hmac_key::HmacState;
    /// # HmacKey::update(&hmac_key.metadata.access_id, HmacState::Inactive)?;
    /// # HmacKey::delete(&hmac_key.metadata.access_id)?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "sync")]
    pub fn create() -> Result<Self, crate::Error> {
        use reqwest::header::CONTENT_LENGTH;

        let url = format!(
            "{}/projects/{}/hmacKeys",
            crate::BASE_URL,
            crate::SERVICE_ACCOUNT.project_id
        );
        let query = [("serviceAccountEmail", &crate::SERVICE_ACCOUNT.client_email)];
        let mut headers = crate::get_headers()?;
        headers.insert(CONTENT_LENGTH, 0.into());
        let client = reqwest::blocking::Client::new();
        let result: GoogleResponse<Self> = client
            .post(&url)
            .headers(headers)
            .query(&query)
            .send()?
            .json()?;
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
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::hmac_key::HmacKey;
    ///
    /// let all_hmac_keys = HmacKey::list()?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "sync")]
    pub fn list() -> Result<Vec<HmacMeta>, crate::Error> {
        let url = format!(
            "{}/projects/{}/hmacKeys",
            crate::BASE_URL,
            crate::SERVICE_ACCOUNT.project_id
        );
        let client = reqwest::blocking::Client::new();
        let result: GoogleResponse<ListResponse> = client
            .get(&url)
            .headers(crate::get_headers()?)
            .send()?
            .json()?;
        match result {
            GoogleResponse::Success(s) => Ok(s.items),
            GoogleResponse::Error(e) => Err(e.into()),
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
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::hmac_key::HmacKey;
    ///
    /// let key = HmacKey::read("some identifier")?;
    /// # Ok(())
    /// # }
    #[cfg(feature = "sync")]
    pub fn read(access_id: &str) -> Result<HmacMeta, crate::Error> {
        let url = format!(
            "{}/projects/{}/hmacKeys/{}",
            crate::BASE_URL,
            crate::SERVICE_ACCOUNT.project_id,
            access_id
        );
        let client = reqwest::blocking::Client::new();
        let result: GoogleResponse<HmacMeta> = client
            .get(&url)
            .headers(crate::get_headers()?)
            .send()?
            .json()?;
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
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::hmac_key::{HmacKey, HmacState};
    ///
    /// let key = HmacKey::update("your key", HmacState::Active)?;
    /// # Ok(())
    /// # }
    #[cfg(feature = "sync")]
    pub fn update(access_id: &str, state: HmacState) -> Result<HmacMeta, crate::Error> {
        let url = format!(
            "{}/projects/{}/hmacKeys/{}",
            crate::BASE_URL,
            crate::SERVICE_ACCOUNT.project_id,
            access_id
        );
        let client = reqwest::blocking::Client::new();
        serde_json::to_string(&UpdateMeta { state })?;
        let result: GoogleResponse<HmacMeta> = client
            .put(&url)
            .headers(crate::get_headers()?)
            .json(&UpdateMeta { state })
            .send()?
            .json()?;
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
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::hmac_key::{HmacKey, HmacState};
    ///
    /// let key = HmacKey::update("your key", HmacState::Inactive)?; // this is required.
    /// HmacKey::delete(&key.access_id)?;
    /// # Ok(())
    /// # }
    #[cfg(feature = "sync")]
    pub fn delete(access_id: &str) -> Result<(), crate::Error> {
        let url = format!(
            "{}/projects/{}/hmacKeys/{}",
            crate::BASE_URL,
            crate::SERVICE_ACCOUNT.project_id,
            access_id
        );
        let client = reqwest::blocking::Client::new();
        let response = client.delete(&url).headers(crate::get_headers()?).send()?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(crate::Error::Google(response.json()?))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "sync")]
    mod sync {
        use super::*;

        fn get_test_hmac() -> HmacMeta {
            match HmacKey::create() {
                Ok(key) => key.metadata,
                Err(_) => HmacKey::list().unwrap().pop().unwrap(),
            }
        }

        fn remove_test_hmac(access_id: &str) {
            HmacKey::update(access_id, HmacState::Inactive).unwrap();
            HmacKey::delete(access_id).unwrap();
        }

        #[test]
        fn create() -> Result<(), Box<dyn std::error::Error>> {
            let key = HmacKey::create()?;
            remove_test_hmac(&key.metadata.access_id);
            Ok(())
        }

        #[test]
        fn list() -> Result<(), Box<dyn std::error::Error>> {
            HmacKey::list()?;
            Ok(())
        }

        #[test]
        fn read() -> Result<(), Box<dyn std::error::Error>> {
            let key = get_test_hmac();
            HmacKey::read(&key.access_id)?;
            remove_test_hmac(&key.access_id);
            Ok(())
        }

        #[test]
        fn update() -> Result<(), Box<dyn std::error::Error>> {
            let key = get_test_hmac();
            HmacKey::update(&key.access_id, HmacState::Inactive)?;
            HmacKey::delete(&key.access_id)?;
            Ok(())
        }


        #[test]
        fn delete() -> Result<(), Box<dyn std::error::Error>> {
            let key = get_test_hmac();
            HmacKey::update(&key.access_id, HmacState::Inactive)?;
            HmacKey::delete(&key.access_id)?;
            Ok(())
        }
    }
}
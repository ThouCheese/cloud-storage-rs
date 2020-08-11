#![allow(unused_imports)]
#![allow(dead_code)]

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

#[derive(Debug, serde::Deserialize)]
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
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::hmac_key::HmacKey;
    ///
    /// let hmac_key = HmacKey::create().await?;
    /// # use cloud_storage::hmac_key::HmacState;
    /// # HmacKey::update(&hmac_key.metadata.access_id, HmacState::Inactive).await?;
    /// # HmacKey::delete(&hmac_key.metadata.access_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create() -> crate::Result<Self> {
        use reqwest::header::CONTENT_LENGTH;

        let url = format!(
            "{}/projects/{}/hmacKeys",
            crate::BASE_URL,
            crate::SERVICE_ACCOUNT.project_id
        );
        let query = [("serviceAccountEmail", &crate::SERVICE_ACCOUNT.client_email)];
        let mut headers = crate::get_headers().await?;
        headers.insert(CONTENT_LENGTH, 0.into());
        let result: GoogleResponse<Self> = crate::CLIENT
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

    /// The synchronous equivalent of `HmacKey::create`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    #[tokio::main]
    pub async fn create_sync() -> crate::Result<Self> {
        Self::create().await
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
    /// use cloud_storage::hmac_key::HmacKey;
    ///
    /// let all_hmac_keys = HmacKey::list().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list() -> crate::Result<Vec<HmacMeta>> {
        let url = format!(
            "{}/projects/{}/hmacKeys",
            crate::BASE_URL,
            crate::SERVICE_ACCOUNT.project_id
        );
        let response = crate::CLIENT
            .get(&url)
            .headers(crate::get_headers().await?)
            .send()
            .await?
            .text()
            .await?;
        let result: Result<GoogleResponse<ListResponse>, _> = serde_json::from_str(&response);

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

    /// The async equivalent of `HmacKey::list`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    #[tokio::main]
    pub async fn list_sync() -> crate::Result<Vec<HmacMeta>> {
        Self::list().await
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
    /// use cloud_storage::hmac_key::HmacKey;
    ///
    /// let key = HmacKey::read("some identifier").await?;
    /// # Ok(())
    /// # }
    pub async fn read(access_id: &str) -> crate::Result<HmacMeta> {
        let url = format!(
            "{}/projects/{}/hmacKeys/{}",
            crate::BASE_URL,
            crate::SERVICE_ACCOUNT.project_id,
            access_id
        );
        let result: GoogleResponse<HmacMeta> = crate::CLIENT
            .get(&url)
            .headers(crate::get_headers().await?)
            .send()
            .await?
            .json()
            .await?;
        match result {
            GoogleResponse::Success(s) => Ok(s),
            GoogleResponse::Error(e) => Err(e.into()),
        }
    }

    /// The synchronous equivalent of `HmacKey::read`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    #[tokio::main]
    pub async fn read_sync(access_id: &str) -> crate::Result<HmacMeta> {
        Self::read(access_id).await
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
    /// use cloud_storage::hmac_key::{HmacKey, HmacState};
    ///
    /// let key = HmacKey::update("your key", HmacState::Active).await?;
    /// # Ok(())
    /// # }
    pub async fn update(access_id: &str, state: HmacState) -> crate::Result<HmacMeta> {
        let url = format!(
            "{}/projects/{}/hmacKeys/{}",
            crate::BASE_URL,
            crate::SERVICE_ACCOUNT.project_id,
            access_id
        );
        serde_json::to_string(&UpdateMeta { state })?;
        let result: GoogleResponse<HmacMeta> = crate::CLIENT
            .put(&url)
            .headers(crate::get_headers().await?)
            .json(&UpdateMeta { state })
            .send()
            .await?
            .json()
            .await?;
        match result {
            GoogleResponse::Success(s) => Ok(s),
            GoogleResponse::Error(e) => Err(e.into()),
        }
    }

    /// The synchronous equivalent of `HmacKey::update`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    #[tokio::main]
    pub async fn update_sync(access_id: &str, state: HmacState) -> crate::Result<HmacMeta> {
        Self::update(access_id, state).await
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
    /// use cloud_storage::hmac_key::{HmacKey, HmacState};
    ///
    /// let key = HmacKey::update("your key", HmacState::Inactive).await?; // this is required.
    /// HmacKey::delete(&key.access_id).await?;
    /// # Ok(())
    /// # }
    pub async fn delete(access_id: &str) -> crate::Result<()> {
        let url = format!(
            "{}/projects/{}/hmacKeys/{}",
            crate::BASE_URL,
            crate::SERVICE_ACCOUNT.project_id,
            access_id
        );
        let response = crate::CLIENT
            .delete(&url)
            .headers(crate::get_headers().await?)
            .send()
            .await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(crate::Error::Google(response.json().await?))
        }
    }

    /// The synchronous equivalent of `HmacKey::delete`.
    #[tokio::main]
    #[cfg(feature = "sync")]
    pub async fn delete_sync(access_id: &str) -> crate::Result<()> {
        Self::delete(access_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn get_test_hmac() -> HmacMeta {
        match HmacKey::create().await {
            Ok(key) => key.metadata,
            Err(_) => HmacKey::list().await.unwrap().pop().unwrap(),
        }
    }

    async fn remove_test_hmac(access_id: &str) {
        HmacKey::update(access_id, HmacState::Inactive)
            .await
            .unwrap();
        HmacKey::delete(access_id).await.unwrap();
    }

    #[tokio::test]
    async fn create() -> Result<(), Box<dyn std::error::Error>> {
        let key = HmacKey::create().await?;
        remove_test_hmac(&key.metadata.access_id).await;
        Ok(())
    }

    #[tokio::test]
    async fn list() -> Result<(), Box<dyn std::error::Error>> {
        HmacKey::list().await?;
        Ok(())
    }

    #[tokio::test]
    async fn read() -> Result<(), Box<dyn std::error::Error>> {
        let key = get_test_hmac().await;
        HmacKey::read(&key.access_id).await?;
        remove_test_hmac(&key.access_id).await;
        Ok(())
    }

    #[tokio::test]
    async fn update() -> Result<(), Box<dyn std::error::Error>> {
        let key = get_test_hmac().await;
        HmacKey::update(&key.access_id, HmacState::Inactive).await?;
        HmacKey::delete(&key.access_id).await?;
        Ok(())
    }

    #[tokio::test]
    async fn delete() -> Result<(), Box<dyn std::error::Error>> {
        let key = get_test_hmac().await;
        HmacKey::update(&key.access_id, HmacState::Inactive).await?;
        HmacKey::delete(&key.access_id).await?;
        Ok(())
    }

    #[tokio::test]
    async fn clear_keys() -> Result<(), Box<dyn std::error::Error>> {
        let keys = HmacKey::list().await?;
        for key in &keys {
            if key.state != HmacState::Inactive {
                HmacKey::update(&key.access_id, HmacState::Inactive).await?;
            }
            HmacKey::delete(&key.access_id).await?;
        }
        Ok(())
    }

    #[cfg(feature = "sync")]
    mod sync {
        use super::*;

        fn get_test_hmac() -> HmacMeta {
            match HmacKey::create_sync() {
                Ok(key) => key.metadata,
                Err(_) => HmacKey::list_sync().unwrap().pop().unwrap(),
            }
        }

        fn remove_test_hmac(access_id: &str) {
            HmacKey::update_sync(access_id, HmacState::Inactive).unwrap();
            HmacKey::delete_sync(access_id).unwrap();
        }

        #[test]
        fn create() -> Result<(), Box<dyn std::error::Error>> {
            let key = HmacKey::create_sync()?;
            remove_test_hmac(&key.metadata.access_id);
            Ok(())
        }

        #[test]
        fn list() -> Result<(), Box<dyn std::error::Error>> {
            HmacKey::list_sync()?;
            Ok(())
        }

        #[test]
        fn read() -> Result<(), Box<dyn std::error::Error>> {
            let key = get_test_hmac();
            HmacKey::read_sync(&key.access_id)?;
            remove_test_hmac(&key.access_id);
            Ok(())
        }

        #[test]
        fn update() -> Result<(), Box<dyn std::error::Error>> {
            let key = get_test_hmac();
            HmacKey::update_sync(&key.access_id, HmacState::Inactive)?;
            HmacKey::delete_sync(&key.access_id)?;
            Ok(())
        }

        #[test]
        fn delete() -> Result<(), Box<dyn std::error::Error>> {
            let key = get_test_hmac();
            HmacKey::update_sync(&key.access_id, HmacState::Inactive)?;
            HmacKey::delete_sync(&key.access_id)?;
            Ok(())
        }
    }
}

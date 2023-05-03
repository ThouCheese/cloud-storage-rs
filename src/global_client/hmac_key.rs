use crate::{models::{HmacKey, HmacMeta, HmacState}, Error};

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
    pub async fn create() -> Result<Self, Error> {
        crate::CLOUD_CLIENT.hmac_key().create().await
    }

    /// The synchronous equivalent of `HmacKey::create`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn create_sync() -> Result<Self, Error> {
        crate::runtime()?.block_on(Self::create())
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
    pub async fn list() -> Result<Vec<HmacMeta>, Error> {
        crate::CLOUD_CLIENT.hmac_key().list().await
    }

    /// The synchronous equivalent of `HmacKey::list`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn list_sync() -> Result<Vec<HmacMeta>, Error> {
        crate::runtime()?.block_on(Self::list())
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
    pub async fn read(access_id: &str) -> Result<HmacMeta, Error> {
        crate::CLOUD_CLIENT.hmac_key().read(access_id).await
    }

    /// The synchronous equivalent of `HmacKey::read`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn read_sync(access_id: &str) -> Result<HmacMeta, Error> {
        crate::runtime()?.block_on(Self::read(access_id))
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
    pub async fn update(access_id: &str, state: HmacState) -> Result<HmacMeta, Error> {
        crate::CLOUD_CLIENT
            .hmac_key()
            .update(access_id, state)
            .await
    }

    /// The synchronous equivalent of `HmacKey::update`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn update_sync(access_id: &str, state: HmacState) -> Result<HmacMeta, Error> {
        crate::runtime()?.block_on(Self::update(access_id, state))
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
    pub async fn delete(access_id: &str) -> Result<(), Error> {
        crate::CLOUD_CLIENT.hmac_key().delete(access_id).await
    }

    /// The synchronous equivalent of `HmacKey::delete`.
    #[cfg(feature = "sync")]
    pub fn delete_sync(access_id: &str) -> Result<(), Error> {
        crate::runtime()?.block_on(Self::delete(access_id))
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
        let keys = HmacKey::list().await?;
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

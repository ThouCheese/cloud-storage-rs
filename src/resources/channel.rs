pub struct Channel {
    pub id: String,
    pub resourceId: String,
}

impl Channel {
    /// Stop receiving object change notifications through this channel.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    #[tokio::main]
    pub async fn stop(&self) -> Result<(), crate::Error> {
        self.stop_async().await
    }

    pub async fn stop_async(&self) -> Result<(), crate::Error> {
        let url = format!("{}/channels/stop", crate::BASE_URL);
        let response = create::CLIENT
            .post(&url)
            .headers(crate::get_headers().await?)
            .send()
            .await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(crate::Error::Google(response.json().await?))
        }
    }
}

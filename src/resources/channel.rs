

pub struct Channel {
    pub id: String,
    pub resourceId: String,
}

impl Channel {
    /// Stop receiving object change notifications through this channel.
    #[cfg(feature = "sync")]
    #[tokio::main]
    pub async fn stop(&self) -> Result<(), crate::Error> {
        self.stop_async().await
    }

    pub async fn stop_async(&self) -> Result<(), crate::Error> {
        let url = format!("{}/channels/stop", crate::BASE_URL);
        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .headers(crate::get_headers_async().await?)
            .send().await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(crate::Error::Google(response.json().await?))
        }
    }
}

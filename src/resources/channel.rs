

pub struct Channel {
    pub id: String,
    pub resourceId: String,
}

impl Channel {
    /// Stop receiving object change notifications through this channel.
    pub fn stop(&self) -> Result<(), crate::Error> {
        let url = format!("{}/channels/stop", crate::BASE_URL);
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(&url)
            .headers(crate::get_headers()?)
            .send()?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(crate::Error::Google(response.json()?))
        }
    }
}

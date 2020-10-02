/// A set of parameters that can be used to customise signed urls.
#[derive(Default)]
pub struct DownloadOptions {
    pub(crate) content_disposition: Option<String>,
}

impl DownloadOptions {
    /// Create a new instance of `DownloadOptions`. Equivalent to `DownloadOptions::default()`.
    ///
    /// ### Example
    /// ```rust
    /// use cloud_storage::DownloadOptions;
    ///
    /// let opts = DownloadOptions::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new instance of `DownloadOptions`. Equivalent to `DownloadOptions::default()`.
    ///
    /// ### Example
    /// ```rust
    /// use cloud_storage::DownloadOptions;
    ///
    /// let opts = DownloadOptions::new()
    ///     .content_disposition("attachment");
    /// ```
    pub fn content_disposition(mut self, content_disposition: &str) -> Self {
        self.content_disposition = Some(content_disposition.to_string());
        self
    }
}

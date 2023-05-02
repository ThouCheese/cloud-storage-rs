use super::ErrorReason;

/// Google Error structure
#[derive(Debug, serde::Deserialize)]
#[serde(rename = "camelCase")]
pub struct Error {
    /// The scope of the error. Example values include: global and push.
    pub domain: String,
    /// Example values include `invalid`, `invalidParameter`, and `required`.
    pub reason: ErrorReason,
    /// Description of the error.
    ///
    /// Example values include `Invalid argument`, `Login required`, and `Required parameter:
    /// project`.
    pub message: String,
    /// The location or part of the request that caused the error. Use with `location` to pinpoint
    /// the error. For example, if you specify an invalid value for a parameter, the `locationType`
    /// will be parameter and the location will be the name of the parameter.
    ///
    /// Example values include `header` and `parameter`.
    pub location_type: Option<String>,
    /// The specific item within the `locationType` that caused the error. For example, if you
    /// specify an invalid value for a parameter, the `location` will be the name of the parameter.
    ///
    /// Example values include: `Authorization`, `project`, and `projection`.
    pub location: Option<String>,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}

impl Error {
    /// Check what was the reason of error
    pub fn is_reason(&self, reason: &ErrorReason) -> bool {
        self.reason == *reason
    }
}
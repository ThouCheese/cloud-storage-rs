use super::{ErrorList, Error, ErrorReason};

/// The structure of a error response returned by Google.
#[derive(Debug, serde::Deserialize)]
#[serde(rename = "camelCase")]
pub struct ErrorResponse {
    /// A container for the error information.
    pub error: ErrorList,
}

impl ErrorResponse {
    /// Return list of errors returned by Google
    pub fn errors(&self) -> &[Error] {
        &self.error.errors
    }

    /// Check whether errors contain given reason
    pub fn errors_has_reason(&self, reason: &ErrorReason) -> bool {
        self.errors()
            .iter()
            .any(|google_error| google_error.is_reason(reason))
    }
}

impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "{:?}", self)
    }
}
impl std::error::Error for ErrorResponse {}
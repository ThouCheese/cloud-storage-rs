use super::Error;

/// A container for the error information.
#[derive(Debug, serde::Deserialize)]
#[serde(rename = "camelCase")]
pub struct ErrorList {
    /// A container for the error details.
    pub errors: Vec<Error>,
    /// An HTTP status code value, without the textual description.
    ///
    /// Example values include: 400 (Bad Request), 401 (Unauthorized), and 404 (Not Found).
    pub code: u16,
    /// Description of the error. Same as errors.message.
    pub message: String,
}
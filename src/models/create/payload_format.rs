/// Various ways of having the response formatted.
#[derive(Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PayloadFormat {
    /// Respond with a format as specified in the Json API V1 documentation.
    JsonApiV1,
    /// Do not respond.
    None,
}
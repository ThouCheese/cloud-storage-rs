/// Contains information about how OPTIONS requests for this Bucket are handled.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cors {
    /// The list of Origins eligible to receive CORS response headers. Note: "*" is permitted in the
    /// list of origins, and means "any Origin".
    #[serde(default)]
    pub origin: Vec<String>,
    /// The list of HTTP methods on which to include CORS response headers, (GET, OPTIONS, POST,
    /// etc) Note: "*" is permitted in the list of methods, and means "any method".
    #[serde(default)]
    pub method: Vec<String>,
    /// The list of HTTP headers other than the simple response headers to give permission for the
    /// user-agent to share across domains.
    #[serde(default)]
    pub response_header: Vec<String>,
    /// The value, in seconds, to return in the Access-Control-Max-Age header used in preflight
    /// responses.
    pub max_age_seconds: Option<i32>,
}
/// Represents any of the ways storing something in Google Cloud Storage can fail
#[derive(Debug)]
pub struct Error {
    msg: String,
}

impl Error {
    pub(crate) fn new(msg: &str) -> Error {
        Error {
            msg: msg.to_string(),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self {
            msg: format!("network error: {}", err),
        }
    }
}

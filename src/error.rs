use crate::models::ErrorResponse;

/// Represents any of the ways storing something in Google Cloud Storage can fail.
#[derive(Debug)]
pub enum Error {
    /// If the error is caused by a non 2xx response by Google, this variant is returned.
    Google(ErrorResponse),
    /// If another network error causes something to fail, this variant is used.
    Reqwest(reqwest::Error),
    /// If we encounter a problem decoding the private key, this variant is used.
    #[cfg(feature = "ring")]
    Pem(pem::PemError),
    /// If we encounter a problem parsing the private key, this variant is used.
    #[cfg(feature = "ring")]
    KeyRejected(ring::error::KeyRejected),
    /// If we encounter a problem signing a request, this variant is used.
    #[cfg(feature = "ring")]
    Signing(ring::error::Unspecified),
    /// If we encouter a SSL error, for example an invalid certificate, this variant is used.
    #[cfg(feature = "openssl")]
    Ssl(openssl::error::ErrorStack),
    /// If we have problems creating or parsing a json web token, this variant is used.
    Jwt(jsonwebtoken::errors::Error),
    /// If we cannot deserialize one of the repsonses sent by Google, this variant is used.
    Serialization(serde_json::error::Error),
    /// If another failure causes the error, this variant is populated.
    Other(String),
}

impl Error {
    pub(crate) fn new(msg: &str) -> Error {
        Error::Other(msg.to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Google(e) => Some(e),
            Self::Reqwest(e) => Some(e),
            #[cfg(feature = "openssl")]
            Self::Ssl(e) => Some(e),
            #[cfg(feature = "ring")]
            Self::Pem(e) => Some(e),
            #[cfg(feature = "ring")]
            Self::KeyRejected(e) => Some(e),
            #[cfg(feature = "ring")]
            Self::Signing(e) => Some(e),
            Self::Jwt(e) => Some(e),
            Self::Serialization(e) => Some(e),
            Self::Other(_) => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err)
    }
}

#[cfg(feature = "openssl")]
impl From<openssl::error::ErrorStack> for Error {
    fn from(err: openssl::error::ErrorStack) -> Self {
        Self::Ssl(err)
    }
}

#[cfg(feature = "ring")]
impl From<pem::PemError> for Error {
    fn from(err: pem::PemError) -> Self {
        Self::Pem(err)
    }
}

#[cfg(feature = "ring")]
impl From<ring::error::KeyRejected> for Error {
    fn from(err: ring::error::KeyRejected) -> Self {
        Self::KeyRejected(err)
    }
}

#[cfg(feature = "ring")]
impl From<ring::error::Unspecified> for Error {
    fn from(err: ring::error::Unspecified) -> Self {
        Self::Signing(err)
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        Self::Jwt(err)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Self {
        Self::Serialization(err)
    }
}

impl From<reqwest::header::InvalidHeaderValue> for Error {
    fn from(err: reqwest::header::InvalidHeaderValue) -> Self {
        Self::Other(err.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Other(err.to_string())
    }
}

impl From<crate::models::ErrorResponse> for Error {
    fn from(err: crate::models::ErrorResponse) -> Self {
        Self::Google(err)
    }
}
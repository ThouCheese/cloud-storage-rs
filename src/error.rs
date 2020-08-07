/// Represents any of the ways storing something in Google Cloud Storage can fail.
#[derive(Debug)]
pub enum Error {
    /// If the error is caused by a non 2xx response by Google, this variant is returned.
    Google(GoogleErrorResponse),
    /// If another network error causes something to fail, this variant is used.
    Reqwest(reqwest::Error),
    /// If we encouter a SSL error, for example an invalid certificate, this variant is used.
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
            Self::Ssl(e) => Some(e),
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

impl From<openssl::error::ErrorStack> for Error {
    fn from(err: openssl::error::ErrorStack) -> Self {
        Self::Ssl(err)
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

#[derive(Debug, serde::Deserialize)]
#[serde(rename = "camelCase")]
#[serde(untagged)]
pub(crate) enum GoogleResponse<T> {
    Success(T),
    Error(GoogleErrorResponse),
}

// impl<T> std::ops::Try for GoogleResponse<T> {
//     type Ok = T;
//     type Error = Error;

//     fn into_result(self) -> Result<Self::Ok, Error> {
//         match self {
//             GoogleResponse::Success(t) => Ok(t),
//             GoogleResponse::Error(error) => Err(Error::Google(error)),
//         }
//     }

//     fn from_error(_a: Error) -> Self {
//         unimplemented!()
//     }

//     fn from_ok(t: T) -> Self {
//         GoogleResponse::Success(t)
//     }
// }

/// The structure of a error response returned by Google.
#[derive(Debug, serde::Deserialize)]
#[serde(rename = "camelCase")]
pub struct GoogleErrorResponse {
    error: ErrorList,
}

impl std::fmt::Display for GoogleErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "{:?}", self)
    }
}

impl std::error::Error for GoogleErrorResponse {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename = "camelCase")]
struct ErrorList {
    errors: Vec<GoogleError>,
    code: u16,
    message: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename = "camelCase")]
struct GoogleError {
    domain: String,
    reason: Reason,
    message: String,
    location_type: Option<String>,
    location: Option<String>,
}

impl From<GoogleErrorResponse> for Error {
    fn from(err: GoogleErrorResponse) -> Self {
        Self::Other(format!(
            "got error response from Google: {}",
            err.error.message
        ))
    }
}

/// Google provides a list of codes, but testing indicates that this list is not exhaustive.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Reason {
    /// When requesting a download using alt=media URL parameter, the direct URL path to use is
    /// prefixed by /download. If this is omitted, the service will issue this redirect with the
    /// appropriate media download path in the Location header.
    MediaDownloadRedirect,
    /// The conditional request would have been successful, but the condition was false, so no body
    /// was sent.
    NotModified,
    /// Resource temporarily located elsewhere according to the Location header. Among other
    /// reasons, this can occur when cookie-based authentication is being used, e.g., when using the
    /// Storage Browser, and it receives a request to download content.
    TemporaryRedirect,
    // /// Indicates an incomplete resumable upload and provides the range of bytes already received by
    // /// Cloud Storage. Responses with this status do not contain a body.
    // ResumeIncomplete,

    // <bad requests>
    /// Undocumeten variant that is sometimes returned by Google.
    Invalid,
    /// The request cannot be completed based on your current Cloud Storage settings. For example,
    /// you cannot lock a retention policy if the requested bucket doesn't have a retention policy,
    /// and you cannot set ACLs if the requested bucket has Bucket Policy Only enabled.
    BadRequest,
    /// The retention period on a locked bucket cannot be reduced.
    BadRequestException,
    /// Bad Cloud KMS key.
    CloudKmsBadKey,
    /// Cloud KMS key name cannot be changed.
    CloudKmsCannotChangeKeyName,
    /// Resource's Cloud KMS decryption key not found.
    CloudKmsDecryptionKeyNotFound,
    /// Cloud KMS key is disabled, destroyed, or scheduled to be destroyed.
    CloudKmsDisabledKey,
    /// Cloud KMS encryption key not found.
    CloudKmsEncryptionKeyNotFound,
    /// Cloud KMS key location not allowed.
    CloudKmsKeyLocationNotAllowed,
    /// Missing an encryption algorithm, or the provided algorithm is not "AE256."
    CustomerEncryptionAlgorithmIsInvalid,
    /// Missing an encryption key, or it is not Base64 encoded, or it does not meet the required
    /// length of the encryption algorithm.
    CustomerEncryptionKeyFormatIsInvalid,
    /// The provided encryption key is incorrect.
    CustomerEncryptionKeyIsIncorrect,
    /// Missing a SHA256 hash of the encryption key, or it is not Base64 encoded, or it does not
    /// match the encryption key.
    CustomerEncryptionKeySha256IsInvalid,
    /// The value for the alt URL parameter was not recognized.
    InvalidAltValue,
    /// The value for one of fields in the request body was invalid.
    InvalidArgument,
    /// The value for one of the URL parameters was invalid. In addition to normal URL parameter
    /// validation, any URL parameters that have a corresponding value in provided JSON request
    /// bodies must match if they are both specified. If using JSONP, you will get this error if you
    /// provide an alt parameter that is not json.
    InvalidParameter,
    /// Uploads or normal API request was sent to a `/download/*` path. Use the same path, but
    /// without the /download prefix.
    NotDownload,
    /// Downloads or normal API request was sent to an `/upload/*` path. Use the same path, but
    /// without the `/upload` prefix.
    NotUpload,
    /// Could not parse the body of the request according to the provided Content-Type.
    ParseError,
    /// Channel id must match the following regular expression: `[A-Za-z0-9\\-_\\+/=]+`.
    #[serde(rename = "push.channelIdInvalid")]
    PushChannelIdInvalid,
    /// `storage.objects.watchAll`'s id property must be unique across channels.
    #[serde(rename = "push.channelIdNotUnique")]
    PushChannelIdNotUnique,
    /// `storage.objects.watchAll`'s address property must contain a valid URL.
    #[serde(rename = "push.webhookUrlNoHostOrAddress")]
    PushWebhookUrlNoHostOrAddress,
    /// `storage.objects.watchAll`'s address property must be an HTTPS URL.
    #[serde(rename = "push.webhookUrlNotHttps")]
    PushWebhookUrlNotHttps,
    /// A required URL parameter or required request body JSON property is missing.
    Required,
    /// The resource is encrypted with a customer-supplied encryption key, but the request did not
    /// provide one.
    ResourceIsEncryptedWithCustomerEncryptionKey,
    /// The resource is not encrypted with a customer-supplied encryption key, but the request
    /// provided one.
    ResourceNotEncryptedWithCustomerEncryptionKey,
    /// A request was made to an API version that has been turned down. Clients will need to update
    /// to a supported version.
    TurnedDown,
    /// The user project specified in the request does not match the user project specifed in an
    /// earlier, related request.
    UserProjectInconsistent,
    /// The user project specified in the request is invalid, either because it is a malformed
    /// project id or because it refers to a non-existent project.
    UserProjectInvalid,
    /// The requested bucket has Requester Pays enabled, the requester is not an owner of the
    /// bucket, and no user project was present in the request.
    UserProjectMissing,
    /// storage.objects.insert must be invoked as an upload rather than a metadata.
    WrongUrlForUpload,
    // </bad requests>

    // <unauthorized>
    /// Access to a Requester Pays bucket requires authentication.
    #[serde(rename = "AuthenticationRequiredRequesterPays")]
    AuthenticationRequiredRequesterPays,
    /// This error indicates a problem with the authorization provided in the request to Cloud
    /// Storage. The following are some situations where that will occur:
    ///
    /// * The OAuth access token has expired and needs to be refreshed. This can be avoided by
    ///   refreshing the access token early, but code can also catch this error, refresh the token
    ///   and retry automatically.
    /// * Multiple non-matching authorizations were provided; choose one mode only.
    /// * The OAuth access token's bound project does not match the project associated with the
    ///   provided developer key.
    /// * The Authorization header was of an unrecognized format or uses an unsupported credential
    ///   type.
    AuthError,
    /// When downloading content from a cookie-authenticated site, e.g., using the Storage Browser,
    /// the response will redirect to a temporary domain. This error will occur if access to said
    /// domain occurs after the domain expires. Issue the original request again, and receive a new
    /// redirect.
    LockedDomainExpired,
    /// Requests to storage.objects.watchAll will fail unless you verify you own the domain.
    #[serde(rename = "push.webhookUrlUnauthorized")]
    PushWebhookUrlUnauthorized,
    // /// Access to a non-public method that requires authorization was made, but none was provided in
    // /// the Authorization header or through other means.
    // Required,
    // </unauthorized>

    // <forbidden>
    ///  The account associated with the project that owns the bucket or object has been disabled. Check the Google Cloud Console to see if there is a problem with billing, and if not, contact account support.
    AccountDisabled,
    /// The Cloud Storage JSON API is restricted by law from operating with certain countries.
    CountryBlocked,
    ///  According to access control policy, the current user does not have access to perform the requested action. This code applies even if the resource being acted on doesn't exist.
    Forbidden,
    ///  According to access control policy, the current user does not have access to perform the requested action. This code applies even if the resource being acted on doesn't exist.
    InsufficientPermissions,
    ///  Object overwrite or deletion is not allowed due to an active hold on the object.
    ObjectUnderActiveHold,
    ///  The Cloud Storage rate limit was exceeded. Retry using exponential backoff.
    RateLimitExceeded,
    ///  Object overwrite or deletion is not allowed until the object meets the retention period set by the retention policy on the bucket.
    RetentionPolicyNotMet,
    ///  Requests to this API require SSL.
    SslRequired,
    ///  Calls to storage.channels.stop require that the caller own the channel.
    StopChannelCallerNotOwner,
    ///  This error implies that for the project associated with the OAuth token or the developer key provided, access to Cloud Storage JSON API is not enabled. This is most commonly because Cloud Storage JSON API is not enabled in the Google Cloud Console, though there are other cases where the project is blocked or has been deleted when this can occur.
    #[serde(rename = "UsageLimits.accessNotConfigured")]
    UsageLimitsAccessNotConfigured,
    /// The requester is not authorized to use the project specified in their request. The
    /// requester must have either the serviceusage.services.use permission or the Editor role for
    /// the specified project.
    #[serde(rename = "UserProjectAccessDenied")]
    UserProjectAccessDenied,
    /// There is a problem with the project used in the request that prevents the operation from
    /// completing successfully. One issue could be billing. Check the billing page to see if you
    /// have a past due balance or if the credit card (or other payment mechanism) on your account is expired. For project creation, see the Projects page in the Google Cloud Console. For other problems, see the Resources and Support page.
    #[serde(rename = "UserProjectAccountProblem")]
    UserProjectAccountProblem,
    /// The developer-specified per-user rate quota was exceeded. If you are the developer, then
    /// you can view these quotas at Quotas pane in the Google Cloud Console.
    UserRateLimitExceeded,
    /// Seems to indicate the same thing
    // NONEXHAUST
    QuotaExceeded,
    // </forbidden>
    /// Either there is no API method associated with the URL path of the request, or the request
    /// refers to one or more resources that were not found.
    NotFound,
    /// Either there is no API method associated with the URL path of the request, or the request
    /// refers to one or more resources that were not found.
    MethodNotAllowed,
    /// The request timed out. Please try again using truncated exponential backoff.
    UploadBrokenConnection,
    /// A request to change a resource, usually a storage.*.update or storage.*.patch method, failed
    /// to commit the change due to a conflicting concurrent change to the same resource. The
    /// request can be retried, though care should be taken to consider the new state of the
    /// resource to avoid blind overwriting of other agent's changes.
    Conflict,
    /// You have attempted to use a resumable upload session that is no longer available. If the
    /// reported status code was not successful and you still wish to upload the file, you must
    /// start a new session.
    Gone,
    // /// You must provide the Content-Length HTTP header. This error has no response body.
    // LengthRequired,

    // <precondition failed>
    /// At least one of the pre-conditions you specified did not hold.
    ConditionNotMet,
    /// Request violates an OrgPolicy constraint.
    OrgPolicyConstraintFailed,
    // </precondition failed>
    /// The Cloud Storage JSON API supports up to 5 TB objects.
    ///
    /// This error may, alternatively, arise if copying objects between locations and/or storage
    /// classes can not complete within 30 seconds. In this case, use the `Object::rewrite` method
    /// instead.
    UploadTooLarge,
    /// The requested Range cannot be satisfied.
    RequestedRangeNotSatisfiable,
    /// A [Cloud Storage JSON API usage limit](https://cloud.google.com/storage/quotas) was
    /// exceeded. If your application tries to use more than its limit, additional requests will
    /// fail. Throttle your client's requests, and/or use truncated exponential backoff.
    #[serde(rename = "usageLimits.rateLimitExceeded")]
    UsageLimitsRateLimitExceeded,

    // <internal server error>
    /// We encountered an internal error. Please try again using truncated exponential backoff.
    BackendError,
    /// We encountered an internal error. Please try again using truncated exponential backoff.
    InternalError,
    // </internal server error>
    /// May be returned by Google, meaning undocumented.
    // NONEXHAUST
    GatewayTimeout,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename = "camelCase")]
enum BadRequest {}

#[derive(Debug, serde::Deserialize)]
#[serde(rename = "camelCase")]
enum Unauthorized {}

#[derive(Debug, serde::Deserialize)]
#[serde(rename = "camelCase")]
enum Forbidden {}

#[derive(Debug, serde::Deserialize)]
#[serde(rename = "camelCase")]
enum PreconditionFailed {}

#[derive(Debug, serde::Deserialize)]
#[serde(rename = "camelCase")]
enum InternalServerError {}

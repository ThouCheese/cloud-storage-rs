use std::collections::HashMap;

use crate::Error;

use super::{CustomerEncrypton, Owner, ObjectAccessControl};

/// A resource representing a file in Google Cloud Storage.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Object {
    /// The kind of item this is. For objects, this is always `storage#object`.
    pub kind: String,
    /// The ID of the object, including the bucket name, object name, and generation number.
    pub id: String,
    /// The link to this object.
    pub self_link: String,
    /// The name of the object. Required if not specified by URL parameter.
    pub name: String,
    /// The name of the bucket containing this object.
    pub bucket: String,
    /// The content generation of this object. Used for object versioning.
    #[serde(deserialize_with = "crate::from_str")]
    pub generation: i64,
    /// The version of the metadata for this object at this generation. Used for preconditions and
    /// for detecting changes in metadata. A metageneration number is only meaningful in the context
    /// of a particular generation of a particular object.
    #[serde(deserialize_with = "crate::from_str")]
    pub metageneration: i64,
    /// Content-Type of the object data. If an object is stored without a Content-Type, it is served
    /// as application/octet-stream.
    pub content_type: Option<String>,
    /// The creation time of the object in RFC 3339 format.
    #[serde(with = "time::serde::rfc3339")]
    pub time_created: time::OffsetDateTime,
    /// The modification time of the object metadata in RFC 3339 format.
    #[serde(with = "time::serde::rfc3339")]
    pub updated: time::OffsetDateTime,
    /// The deletion time of the object in RFC 3339 format. Returned if and only if this version of
    /// the object is no longer a live version, but remains in the bucket as a noncurrent version.
    #[serde(default, with = "time::serde::rfc3339::option")]
    pub time_deleted: Option<time::OffsetDateTime>,
    /// Whether or not the object is subject to a temporary hold.
    pub temporary_hold: Option<bool>,
    /// Whether or not the object is subject to an event-based hold.
    pub event_based_hold: Option<bool>,
    /// The earliest time that the object can be deleted, based on a bucket's retention policy, in
    /// RFC 3339 format.
    #[serde(default, with = "time::serde::rfc3339::option")]
    pub retention_expiration_time: Option<time::OffsetDateTime>,
    /// Storage class of the object.
    pub storage_class: String,
    /// The time at which the object's storage class was last changed. When the object is initially
    /// created, it will be set to timeCreated.
    #[serde(with = "time::serde::rfc3339")]
    pub time_storage_class_updated: time::OffsetDateTime,
    /// Content-Length of the data in bytes.
    #[serde(deserialize_with = "crate::from_str")]
    pub size: u64,
    /// MD5 hash of the data; encoded using base64. For more information about using the MD5 hash,
    /// see Hashes and ETags: Best Practices.
    pub md5_hash: Option<String>,
    /// Media download link.
    pub media_link: String,
    /// Content-Encoding of the object data.
    pub content_encoding: Option<String>,
    /// Content-Disposition of the object data.
    pub content_disposition: Option<String>,
    /// Content-Language of the object data.
    pub content_language: Option<String>,
    /// Cache-Control directive for the object data. If omitted, and the object is accessible to all
    /// anonymous users, the default will be public, max-age=3600.
    pub cache_control: Option<String>,
    /// User-provided metadata, in key/value pairs.
    pub metadata: Option<HashMap<String, String>>,
    /// Access controls on the object, containing one or more objectAccessControls Resources. If
    /// iamConfiguration.uniformBucketLevelAccess.enabled is set to true, this field is omitted in
    /// responses, and requests that specify this field fail.
    pub acl: Option<Vec<ObjectAccessControl>>,
    /// The owner of the object. This will always be the uploader of the object. If
    /// `iamConfiguration.uniformBucketLevelAccess.enabled` is set to true, this field does not
    /// apply, and is omitted in responses.
    pub owner: Option<Owner>,
    /// CRC32c checksum, as described in RFC 4960, Appendix B; encoded using base64 in big-endian
    /// byte order. For more information about using the CRC32c checksum, see Hashes and ETags: Best
    /// Practices.
    pub crc32c: String,
    /// Number of underlying components that make up a composite object. Components are accumulated
    /// by compose operations, counting 1 for each non-composite source object and componentCount
    /// for each composite source object. Note: componentCount is included in the metadata for
    /// composite objects only.
    #[serde(default, deserialize_with = "crate::from_str_opt")]
    pub component_count: Option<i32>,
    /// HTTP 1.1 Entity tag for the object.
    pub etag: String,
    /// Metadata of customer-supplied encryption key, if the object is encrypted by such a key.
    pub customer_encryption: Option<CustomerEncrypton>,
    /// Cloud KMS Key used to encrypt this object, if the object is encrypted by such a key.
    pub kms_key_name: Option<String>,

    #[serde(skip)]
    pub(crate) private_key: Option<String>,
    #[serde(skip)]
    pub(crate) client_email: Option<String>,
}

impl Object {
    /// Creates a [Signed Url](https://cloud.google.com/storage/docs/access-control/signed-urls)
    /// which is valid for `duration` seconds, and lets the posessor download the file contents
    /// without any authentication.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::{CloudStorageClient, models::{Object, ComposeRequest}};
    ///
    /// let client = CloudStorageClient::default();
    /// let obj1 = client.object("my_bucket").read("file1", None).await?;
    /// let url = obj1.download_url(50)?;
    /// // url is now a url to which an unauthenticated user can make a request to download a file
    /// // for 50 seconds.
    /// # Ok(())
    /// # }
    /// ```
    pub fn download_url(&self, duration: u32) -> Result<String, Error> {
        self.sign(&self.name, duration, "GET", None, &HashMap::new())
    }

    /// Creates a [Signed Url](https://cloud.google.com/storage/docs/access-control/signed-urls)
    /// which is valid for `duration` seconds, and lets the posessor download the file contents
    /// without any authentication.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::{CloudStorageClient, models::{Object, ComposeRequest}};
    ///
    /// let client = CloudStorageClient::default();
    /// let obj1 = client.object("my_bucket").read("file1", None).await?;
    /// let url = obj1.download_url(50)?;
    /// // url is now a url to which an unauthenticated user can make a request to download a file
    /// // for 50 seconds.
    /// # Ok(())
    /// # }
    /// ```
    pub fn download_url_with(
        &self,
        duration: u32,
        opts: crate::DownloadOptions,
    ) -> Result<String, Error> {
        self.sign(
            &self.name,
            duration,
            "GET",
            opts.content_disposition,
            &HashMap::new(),
        )
    }

    /// Creates a [Signed Url](https://cloud.google.com/storage/docs/access-control/signed-urls)
    /// which is valid for `duration` seconds, and lets the posessor upload data to a blob
    /// without any authentication.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::{CloudStorageClient, models::{Object, ComposeRequest}};
    ///
    /// let client = CloudStorageClient::default();
    /// let obj1 = client.object("my_bucket").read("file1", None).await?;
    /// let url = obj1.upload_url(50)?;
    /// // url is now a url to which an unauthenticated user can make a PUT request to upload a file
    /// // for 50 seconds.
    /// # Ok(())
    /// # }
    /// ```
    pub fn upload_url(&self, duration: u32) -> Result<String, Error> {
        self.sign(&self.name, duration, "PUT", None, &HashMap::new())
    }

    /// Creates a [Signed Url](https://cloud.google.com/storage/docs/access-control/signed-urls)
    /// which is valid for `duration` seconds, and lets the posessor upload data and custom metadata
    /// to a blob without any authentication.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::{CloudStorageClient, models::{Object, ComposeRequest}};
    /// # use std::collections::HashMap;
    ///
    /// let client = CloudStorageClient::default();
    /// let obj1 = client.object("my_bucket").read("file1", None).await?;
    /// let mut custom_metadata = HashMap::new();
    /// custom_metadata.insert(String::from("field"), String::from("value"));
    /// let (url, headers) = obj1.upload_url_with(50, custom_metadata)?;
    /// // url is now a url to which an unauthenticated user can make a PUT request to upload a file
    /// // for 50 seconds. Note that the user must also include the returned headers in the PUT request
    /// # Ok(())
    /// # }
    /// ```
    pub fn upload_url_with(&self, duration: u32, custom_metadata: HashMap<String, String>) -> Result<(String, HashMap<String, String>), Error> {
        let url = self.sign(&self.name, duration, "PUT", None, &custom_metadata)?;
        let mut headers = HashMap::new();
        for (k, v) in custom_metadata.iter() {
            headers.insert(format!("x-goog-meta-{}", k), v.to_string());
        }
        Ok((url, headers))
    }

    /// Creates a [Signed Url](https://cloud.google.com/storage/docs/access-control/signed-urls)
    /// which is valid for `duration` seconds, and lets the posessor upload new file contents.
    /// without any authentication.
    /// ### Example
    /// ```ignore
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::{CloudStorageClient, models::{Object, ComposeRequest}};
    /// # use std::collections::HashMap;
    /// let client = CloudStorageClient::default();
    /// let obj1 = client.object("my_bucket").read("file1", None).await?;
    /// let mut custom_metadata = HashMap::new();
    /// custom_metadata.insert(String::from("field"), String::from("value"));
    /// let url = obj1.sign(obj1.name, 50, "POST", None, custom_metadata)?;
    /// # Ok(())
    /// # }
    /// ```
    fn sign(
        &self,
        file_path: &str,
        duration: u32,
        http_verb: &str,
        content_disposition: Option<String>,
        custom_metadata: &HashMap<String, String>,
    ) -> Result<String, Error> {
        let client_email = self.client_email.clone().expect("client_email not configured");
        if duration > 604800 {
            let msg = format!(
                "duration may not be greater than 604800, but was {}",
                duration
            );
            return Err(crate::Error::Other(msg));
        }

        // 0 Sort and construct the canonical headers
        let mut headers = vec![("host".to_string(), "storage.googleapis.com".to_string())];
        // Add custom metadata headers, guaranteed unique by HashMap input
        for (k, v) in custom_metadata.iter() {
            headers.push((format!("x-goog-meta-{}", k), v.to_string()));
        }
        headers.sort_unstable_by(|(k1, _), (k2, _)| k1.cmp(k2));
        let canonical_headers: String = headers
            .iter()
            .map(|(k, v)| format!("{}:{}", k.to_lowercase(), v.to_lowercase()))
            .collect::<Vec<String>>()
            .join("\n");
        let signed_headers = headers
            .iter()
            .map(|(k, _)| k.to_lowercase())
            .collect::<Vec<String>>()
            .join(";");

        // 1 construct the canonical request
        let issue_date = time::OffsetDateTime::now_utc();
        let file_path = self.path_to_resource(file_path);
        let query_string = Self::get_canonical_query_string(
            &issue_date,
            duration,
            &signed_headers,
            content_disposition,
            &client_email
        );
        let canonical_request = self.get_canonical_request(
            &file_path,
            &query_string,
            http_verb,
            &canonical_headers,
            &signed_headers,
        );

        // 2 get hex encoded SHA256 hash the canonical request
        let hex_hash = hex::encode(crate::crypto::sha256(canonical_request.as_bytes()).as_ref());

        // 3 construct the string to sign
        let string_to_sign = format!(
            "{signing_algorithm}\n\
            {current_datetime}\n\
            {credential_scope}\n\
            {hashed_canonical_request}",
            signing_algorithm = "GOOG4-RSA-SHA256",
            current_datetime = issue_date.format(crate::ISO_8601_BASIC_FORMAT).unwrap(),
            credential_scope = Self::get_credential_scope(&issue_date),
            hashed_canonical_request = hex_hash,
        );

        // 4 sign the string to sign with RSA - SHA256
        let signature = hex::encode(crate::crypto::rsa_pkcs1_sha256(&string_to_sign, self.private_key.clone().expect("No Private Key in Object").as_bytes())?);

        // 5 construct the signed url
        Ok(format!(
            "https://storage.googleapis.com{path_to_resource}?\
            {query_string}&\
            X-Goog-Signature={request_signature}",
            path_to_resource = file_path,
            query_string = query_string,
            request_signature = signature,
        ))
    }

    #[inline(always)]
    fn get_canonical_request(
        &self,
        path: &str,
        query_string: &str,
        http_verb: &str,
        headers: &str,
        signed_headers: &str,
    ) -> String {
        format!(
            "{http_verb}\n\
            {path_to_resource}\n\
            {canonical_query_string}\n\
            {canonical_headers}\n\
            \n\
            {signed_headers}\n\
            {payload}",
            http_verb = http_verb,
            path_to_resource = path,
            canonical_query_string = query_string,
            canonical_headers = headers,
            signed_headers = signed_headers,
            payload = "UNSIGNED-PAYLOAD",
        )
    }

    #[inline(always)]
    fn get_canonical_query_string(
        date: &time::OffsetDateTime,
        exp: u32,
        headers: &str,
        content_disposition: Option<String>,
        client_email: &str
    ) -> String {
        let credential = format!(
            "{authorizer}/{scope}",
            authorizer = client_email,
            scope = Self::get_credential_scope(date),
        );

        let disposition = match content_disposition {
            Some(cd) => format!("&response-content-disposition={}", cd),
            None => "".to_string()
        };

        let s = format!(
            "X-Goog-Algorithm={algo}&\
            X-Goog-Credential={cred}&\
            X-Goog-Date={date}&\
            X-Goog-Expires={exp}&\
            X-Goog-SignedHeaders={signed}\
            {disposition}",
            algo = "GOOG4-RSA-SHA256",
            cred = crate::percent_encode(&credential),
            date = date.format(crate::ISO_8601_BASIC_FORMAT).unwrap(),
            exp = exp,
            disposition = disposition,
            signed = crate::percent_encode(headers),
        );
        s
    }

    #[inline(always)]
    fn path_to_resource(&self, path: &str) -> String {
        format!(
            "/{bucket}/{file_path}",
            bucket = self.bucket,
            file_path = crate::percent_encode_noslash(path),
        )
    }

    #[inline(always)]
    fn get_credential_scope(date: &time::OffsetDateTime) -> String {
        format!(
            "{}/auto/storage/goog4_request",
            date.format(time::macros::format_description!("[year][month][day]")).unwrap(),
        )
    }
}
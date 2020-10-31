use crate::error::{Error, GoogleResponse};
pub use crate::resources::bucket::Owner;
use crate::resources::common::ListResponse;
use crate::resources::object_access_control::ObjectAccessControl;
use futures::{stream, Stream, TryStream};
use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};

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
    pub time_created: chrono::DateTime<chrono::Utc>,
    /// The modification time of the object metadata in RFC 3339 format.
    pub updated: chrono::DateTime<chrono::Utc>,
    /// The deletion time of the object in RFC 3339 format. Returned if and only if this version of
    /// the object is no longer a live version, but remains in the bucket as a noncurrent version.
    pub time_deleted: Option<chrono::DateTime<chrono::Utc>>,
    /// Whether or not the object is subject to a temporary hold.
    pub temporary_hold: Option<bool>,
    /// Whether or not the object is subject to an event-based hold.
    pub event_based_hold: Option<bool>,
    /// The earliest time that the object can be deleted, based on a bucket's retention policy, in
    /// RFC 3339 format.
    pub retention_expiration_time: Option<chrono::DateTime<chrono::Utc>>,
    /// Storage class of the object.
    pub storage_class: String,
    /// The time at which the object's storage class was last changed. When the object is initially
    /// created, it will be set to timeCreated.
    pub time_storage_class_updated: chrono::DateTime<chrono::Utc>,
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
    pub metadata: Option<std::collections::HashMap<String, String>>,
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
}

/// Contains data about how a user might encrypt their files in Google Cloud Storage.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerEncrypton {
    /// The encryption algorithm.
    pub encryption_algorithm: String,
    /// SHA256 hash value of the encryption key.
    pub key_sha256: String,
}

/// The request that is supplied to perform `Object::compose`.
#[derive(Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComposeRequest {
    /// The kind of item this is. Will always be `storage#composeRequest`.
    pub kind: String,
    /// The list of source objects that will be concatenated into a single object.
    pub source_objects: Vec<SourceObject>,
    /// Properties of the resulting object.
    pub destination: Option<Object>,
}

/// A SourceObject represents one of the objects that is to be composed.
#[derive(Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceObject {
    /// The source object's name. All source objects must have the same storage class and reside in
    /// the same bucket.
    pub name: String,
    /// The generation of this object to use as the source.
    pub generation: Option<i64>,
    /// Conditions that must be met for this operation to execute.
    pub object_preconditions: Option<ObjectPrecondition>,
}

/// Allows conditional copying of this file.
#[derive(Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectPrecondition {
    /// Only perform the composition if the generation of the source object that would be used
    /// matches this value. If this value and a generation are both specified, they must be the same
    /// value or the call will fail.
    pub if_generation_match: i64,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ObjectList {
    kind: String,
    items: Vec<Object>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct RewriteResponse {
    kind: String,
    total_bytes_rewritten: String,
    object_size: String,
    done: bool,
    resource: Object,
}

impl Object {
    /// Create a new object.
    /// Upload a file as that is loaded in memory to google cloud storage, where it will be
    /// interpreted according to the mime type you specified.
    /// ## Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # fn read_cute_cat(_in: &str) -> Vec<u8> { vec![0, 1] }
    /// use cloud_storage::Object;
    ///
    /// let file: Vec<u8> = read_cute_cat("cat.png");
    /// Object::create("cat-photos", file, "recently read cat.png", "image/png").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(
        bucket: &str,
        file: Vec<u8>,
        filename: &str,
        mime_type: &str,
    ) -> crate::Result<Self> {
        use reqwest::header::{CONTENT_LENGTH, CONTENT_TYPE};

        // has its own url for some reason
        const BASE_URL: &str = "https://www.googleapis.com/upload/storage/v1/b";
        let url = &format!(
            "{}/{}/o?uploadType=media&name={}",
            BASE_URL,
            percent_encode(&bucket),
            percent_encode(&filename),
        );
        let mut headers = crate::get_headers().await?;
        headers.insert(CONTENT_TYPE, mime_type.parse()?);
        headers.insert(CONTENT_LENGTH, file.len().to_string().parse()?);
        let response = crate::CLIENT
            .post(url)
            .headers(headers)
            .body(file)
            .send()
            .await?;
        if response.status() == 200 {
            Ok(serde_json::from_str(&response.text().await?)?)
        } else {
            Err(Error::new(&response.text().await?))
        }
    }

    /// The synchronous equivalent of `Object::create`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    #[tokio::main]
    pub async fn create_sync(
        bucket: &str,
        file: Vec<u8>,
        filename: &str,
        mime_type: &str,
    ) -> crate::Result<Self> {
        Self::create(bucket, file, filename, mime_type).await
    }

    /// Create a new object. This works in the same way as `Object::create`, except it does not need
    /// to load the entire file in ram.
    /// ## Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Object;
    ///
    /// let file = reqwest::Client::new()
    ///     .get("https://my_domain.rs/nice_cat_photo.png")
    ///     .send()
    ///     .await?
    ///     .bytes_stream();
    /// Object::create_streamed("cat-photos", file, 10, "recently read cat.png", "image/png").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_streamed<S>(
        bucket: &str,
        stream: S,
        length: impl Into<Option<u64>>,
        filename: &str,
        mime_type: &str,
    ) -> crate::Result<Self>
    where
        S: TryStream + Send + Sync + 'static,
        S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
        bytes::Bytes: From<S::Ok>,
    {
        use reqwest::header::{CONTENT_LENGTH, CONTENT_TYPE};

        // has its own url for some reason
        const BASE_URL: &str = "https://www.googleapis.com/upload/storage/v1/b";
        let url = &format!(
            "{}/{}/o?uploadType=media&name={}",
            BASE_URL,
            percent_encode(&bucket),
            percent_encode(&filename),
        );
        let mut headers = crate::get_headers().await?;
        headers.insert(CONTENT_TYPE, mime_type.parse()?);
        if let Some(length) = length.into() {
            headers.insert(CONTENT_LENGTH, length.into());
        }

        let body = reqwest::Body::wrap_stream(stream);
        let response = crate::CLIENT
            .post(url)
            .headers(headers)
            .body(body)
            .send()
            .await?;
        if response.status() == 200 {
            Ok(serde_json::from_str(&response.text().await?)?)
        } else {
            Err(Error::new(&response.text().await?))
        }
    }

    /// The async equivalent of `Object::create_streamed`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    #[tokio::main]
    pub async fn create_streamed_sync<R: std::io::Read + Send + 'static>(
        bucket: &str,
        mut file: R,
        length: impl Into<Option<u64>>,
        filename: &str,
        mime_type: &str,
    ) -> crate::Result<Self> {
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| Error::Other(e.to_string()))?;

        let stream = stream::once(async { Ok::<_, Error>(buffer) });

        Self::create_streamed(bucket, stream, length, filename, mime_type).await
    }

    /// Obtain a list of objects within this Bucket.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Object;
    ///
    /// let all_objects = Object::list("my_bucket").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(
        bucket: &str,
    ) -> Result<impl Stream<Item = Result<Vec<Self>, Error>> + '_, Error> {
        Self::list_from(bucket, None).await
    }

    /// The async equivalent of `Object::list`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    #[tokio::main]
    pub async fn list_sync(bucket: &str) -> Result<Vec<Self>, Error> {
        use futures::TryStreamExt;

        Self::list_from(bucket, None).await?.try_concat().await
    }

    /// Obtain a list of objects by prefix within this Bucket .
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Object;
    ///
    /// let all_objects = Object::list_prefix("my_bucket", "prefix/").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_prefix<'a>(
        bucket: &'a str,
        prefix: &'a str,
    ) -> Result<impl Stream<Item = Result<Vec<Self>, Error>> + 'a, Error> {
        Self::list_from(bucket, Some(prefix)).await
    }

    /// The async equivalent of `Object::list_prefix`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    #[tokio::main]
    pub async fn list_prefix_sync(bucket: &str, prefix: &str) -> Result<Vec<Self>, Error> {
        use futures::TryStreamExt;

        Self::list_from(bucket, Some(prefix))
            .await?
            .try_concat()
            .await
    }

    async fn list_from<'a>(
        bucket: &'a str,
        prefix: Option<&'a str>,
    ) -> Result<impl Stream<Item = Result<Vec<Self>, Error>> + 'a, Error> {
        #[derive(Clone)]
        enum ListState {
            Start,
            HasMore(String),
            Done,
        }
        use ListState::*;

        Ok(stream::unfold(ListState::Start, move |state| async move {
            let url = format!("{}/b/{}/o", crate::BASE_URL, percent_encode(bucket));
            let headers = match crate::get_headers().await {
                Ok(h) => h,
                Err(e) => return Some((Err(e), state)),
            };

            let mut query = match state.clone() {
                HasMore(page_token) => vec![("pageToken", page_token)],
                Done => return None,
                Start => vec![],
            };

            if let Some(prefix) = prefix {
                query.push(("prefix", prefix.to_string()));
            };

            let response = crate::CLIENT
                .get(&url)
                .query(&query)
                .headers(headers)
                .send()
                .await;
            let response = match response {
                Ok(r) => r,
                Err(e) => return Some((Err(e.into()), state)),
            };

            let json = match response.json().await {
                Ok(json) => json,
                Err(e) => return Some((Err(e.into()), state)),
            };

            let result: GoogleResponse<ListResponse<Self>> = json;

            let response_body = match result {
                GoogleResponse::Success(success) => success,
                GoogleResponse::Error(e) => return Some((Err(e.into()), state)),
            };

            let items = response_body.items;

            let next_state = if let Some(page_token) = response_body.next_page_token {
                HasMore(page_token)
            } else {
                Done
            };

            Some((Ok(items), next_state))
        }))
    }

    /// Obtains a single object with the specified name in the specified bucket.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Object;
    ///
    /// let object = Object::read("my_bucket", "path/to/my/file.png").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn read(bucket: &str, file_name: &str) -> crate::Result<Self> {
        let url = format!(
            "{}/b/{}/o/{}",
            crate::BASE_URL,
            percent_encode(bucket),
            percent_encode(file_name),
        );
        let result: GoogleResponse<Self> = crate::CLIENT
            .get(&url)
            .headers(crate::get_headers().await?)
            .send()
            .await?
            .json()
            .await?;
        match result {
            GoogleResponse::Success(s) => Ok(s),
            GoogleResponse::Error(e) => Err(e.into()),
        }
    }

    /// The synchronous equivalent of `Object::read`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    #[tokio::main]
    pub async fn read_sync(bucket: &str, file_name: &str) -> crate::Result<Self> {
        Self::read(bucket, file_name).await
    }

    /// Download the content of the object with the specified name in the specified bucket.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Object;
    ///
    /// let bytes = Object::download("my_bucket", "path/to/my/file.png").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download(bucket: &str, file_name: &str) -> Result<Vec<u8>, Error> {
        let url = format!(
            "{}/b/{}/o/{}?alt=media",
            crate::BASE_URL,
            percent_encode(bucket),
            percent_encode(file_name),
        );
        Ok(crate::CLIENT
            .get(&url)
            .headers(crate::get_headers().await?)
            .send()
            .await?
            .bytes()
            .await?
            .to_vec())
    }

    /// The synchronous equivalent of `Object::download`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    #[tokio::main]
    pub async fn download_sync(bucket: &str, file_name: &str) -> crate::Result<Vec<u8>> {
        Self::download(bucket, file_name).await
    }

    /// Download the content of the object with the specified name in the specified bucket, without
    /// allocating the whole file into a vector.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Object;
    /// use futures::StreamExt;
    /// use std::fs::File;
    /// use std::io::Write;
    ///
    /// let mut stream = Object::download_streamed("my_bucket", "path/to/my/file.png").await?;
    /// let mut file = File::create("file.png").unwrap();
    /// for part in stream.next().await {
    ///     file.write_all(&part.unwrap()).unwrap();
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download_streamed(
        bucket: &str,
        file_name: &str,
    ) -> crate::Result<impl Stream<Item = crate::Result<Vec<u8>>>> {
        use futures::StreamExt;

        let url = format!(
            "{}/b/{}/o/{}?alt=media",
            crate::BASE_URL,
            percent_encode(bucket),
            percent_encode(file_name),
        );
        Ok(crate::CLIENT
            .get(&url)
            .headers(crate::get_headers().await?)
            .send()
            .await?
            .bytes_stream()
            .map(|res| Ok(res.map(|r| r.to_vec())?)))
    }

    /// Obtains a single object with the specified name in the specified bucket.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Object;
    ///
    /// let mut object = Object::read("my_bucket", "path/to/my/file.png").await?;
    /// object.content_type = Some("application/xml".to_string());
    /// object.update().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(&self) -> crate::Result<Self> {
        let url = format!(
            "{}/b/{}/o/{}",
            crate::BASE_URL,
            percent_encode(&self.bucket),
            percent_encode(&self.name),
        );
        let result: GoogleResponse<Self> = crate::CLIENT
            .put(&url)
            .headers(crate::get_headers().await?)
            .json(&self)
            .send()
            .await?
            .json()
            .await?;
        match result {
            GoogleResponse::Success(s) => Ok(s),
            GoogleResponse::Error(e) => Err(e.into()),
        }
    }

    /// The synchronous equivalent of `Object::download`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    #[tokio::main]
    pub async fn update_sync(&self) -> crate::Result<Self> {
        self.update().await
    }

    /// Deletes a single object with the specified name in the specified bucket.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Object;
    ///
    /// Object::delete("my_bucket", "path/to/my/file.png").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(bucket: &str, file_name: &str) -> Result<(), Error> {
        let url = format!(
            "{}/b/{}/o/{}",
            crate::BASE_URL,
            percent_encode(bucket),
            percent_encode(file_name),
        );
        let response = crate::CLIENT
            .delete(&url)
            .headers(crate::get_headers().await?)
            .send()
            .await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::Google(response.json().await?))
        }
    }

    /// The synchronous equivalent of `Object::delete`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    #[tokio::main]
    pub async fn delete_sync(bucket: &str, file_name: &str) -> Result<(), Error> {
        Self::delete(bucket, file_name).await
    }

    /// Obtains a single object with the specified name in the specified bucket.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::object::{Object, ComposeRequest, SourceObject};
    ///
    /// let obj1 = Object::read("my_bucket", "file1").await?;
    /// let obj2 = Object::read("my_bucket", "file2").await?;
    /// let compose_request = ComposeRequest {
    ///     kind: "storage#composeRequest".to_string(),
    ///     source_objects: vec![
    ///         SourceObject {
    ///             name: obj1.name.clone(),
    ///             generation: None,
    ///             object_preconditions: None,
    ///         },
    ///         SourceObject {
    ///             name: obj2.name.clone(),
    ///             generation: None,
    ///             object_preconditions: None,
    ///         },
    ///     ],
    ///     destination: None,
    /// };
    /// let obj3 = Object::compose("my_bucket", &compose_request, "test-concatted-file").await?;
    /// // obj3 is now a file with the content of obj1 and obj2 concatted together.
    /// # Ok(())
    /// # }
    /// ```
    pub async fn compose(
        bucket: &str,
        req: &ComposeRequest,
        destination_object: &str,
    ) -> crate::Result<Self> {
        let url = format!(
            "{}/b/{}/o/{}/compose",
            crate::BASE_URL,
            percent_encode(&bucket),
            percent_encode(&destination_object)
        );
        let result: GoogleResponse<Self> = crate::CLIENT
            .post(&url)
            .headers(crate::get_headers().await?)
            .json(req)
            .send()
            .await?
            .json()
            .await?;
        match result {
            GoogleResponse::Success(s) => Ok(s),
            GoogleResponse::Error(e) => Err(e.into()),
        }
    }

    /// The synchronous equivalent of `Object::compose`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    #[tokio::main]
    pub async fn compose_sync(
        bucket: &str,
        req: &ComposeRequest,
        destination_object: &str,
    ) -> crate::Result<Self> {
        Self::compose(bucket, req, destination_object).await
    }

    /// Copy this object to the target bucket and path
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::object::{Object, ComposeRequest};
    ///
    /// let obj1 = Object::read("my_bucket", "file1").await?;
    /// let obj2 = obj1.copy("my_other_bucket", "file2").await?;
    /// // obj2 is now a copy of obj1.
    /// # Ok(())
    /// # }
    /// ```
    pub async fn copy(&self, destination_bucket: &str, path: &str) -> crate::Result<Self> {
        use reqwest::header::CONTENT_LENGTH;

        let url = format!(
            "{base}/b/{sBucket}/o/{sObject}/copyTo/b/{dBucket}/o/{dObject}",
            base = crate::BASE_URL,
            sBucket = percent_encode(&self.bucket),
            sObject = percent_encode(&self.name),
            dBucket = percent_encode(&destination_bucket),
            dObject = percent_encode(&path),
        );
        let mut headers = crate::get_headers().await?;
        headers.insert(CONTENT_LENGTH, "0".parse()?);
        let result: GoogleResponse<Self> = crate::CLIENT
            .post(&url)
            .headers(headers)
            .send()
            .await?
            .json()
            .await?;
        match result {
            GoogleResponse::Success(s) => Ok(s),
            GoogleResponse::Error(e) => Err(e.into()),
        }
    }

    /// The synchronous equivalent of `Object::copy`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    #[tokio::main]
    pub async fn copy_sync(&self, destination_bucket: &str, path: &str) -> crate::Result<Self> {
        self.copy(destination_bucket, path).await
    }

    /// Moves a file from the current location to the target bucket and path.
    ///
    /// ## Limitations
    /// This function does not yet support rewriting objects to another
    /// * Geographical Location,
    /// * Encryption,
    /// * Storage class.
    /// These limitations mean that for now, the rewrite and the copy methods do the same thing.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::object::Object;
    ///
    /// let obj1 = Object::read("my_bucket", "file1").await?;
    /// let obj2 = obj1.rewrite("my_other_bucket", "file2").await?;
    /// // obj2 is now a copy of obj1.
    /// # Ok(())
    /// # }
    /// ```
    pub async fn rewrite(&self, destination_bucket: &str, path: &str) -> crate::Result<Self> {
        use reqwest::header::CONTENT_LENGTH;

        let url = format!(
            "{base}/b/{sBucket}/o/{sObject}/rewriteTo/b/{dBucket}/o/{dObject}",
            base = crate::BASE_URL,
            sBucket = percent_encode(&self.bucket),
            sObject = percent_encode(&self.name),
            dBucket = percent_encode(destination_bucket),
            dObject = percent_encode(path),
        );
        let mut headers = crate::get_headers().await?;
        headers.insert(CONTENT_LENGTH, "0".parse()?);
        let result: GoogleResponse<RewriteResponse> = crate::CLIENT
            .post(&url)
            .headers(headers)
            .send()
            .await?
            .json()
            .await?;
        match result {
            GoogleResponse::Success(s) => Ok(s.resource),
            GoogleResponse::Error(e) => Err(e.into()),
        }
    }

    /// The synchronous equivalent of `Object::rewrite`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    #[tokio::main]
    pub async fn rewrite_sync(&self, destination_bucket: &str, path: &str) -> crate::Result<Self> {
        self.rewrite(destination_bucket, path).await
    }

    /// Creates a [Signed Url](https://cloud.google.com/storage/docs/access-control/signed-urls)
    /// which is valid for `duration` seconds, and lets the posessor download the file contents
    /// without any authentication.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::object::{Object, ComposeRequest};
    ///
    /// let obj1 = Object::read("my_bucket", "file1").await?;
    /// let url = obj1.download_url(50)?;
    /// // url is now a url to which an unauthenticated user can make a request to download a file
    /// // for 50 seconds.
    /// # Ok(())
    /// # }
    /// ```
    pub fn download_url(&self, duration: u32) -> crate::Result<String> {
        self.sign(&self.name, duration, "GET", None)
    }

    /// Creates a [Signed Url](https://cloud.google.com/storage/docs/access-control/signed-urls)
    /// which is valid for `duration` seconds, and lets the posessor download the file contents
    /// without any authentication.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::object::{Object, ComposeRequest};
    ///
    /// let obj1 = Object::read("my_bucket", "file1").await?;
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
    ) -> crate::Result<String> {
        self.sign(&self.name, duration, "GET", opts.content_disposition)
    }

    // /// Creates a [Signed Url](https://cloud.google.com/storage/docs/access-control/signed-urls)
    // /// which is valid for `duration` seconds, and lets the posessor upload new file contents.
    // /// without any authentication.
    // pub fn upload_url(&self, duration: u32) -> crate::Result<String> {
    //     self.sign(&self.name, duration, "POST")
    // }

    #[inline(always)]
    fn sign(
        &self,
        file_path: &str,
        duration: u32,
        http_verb: &str,
        content_disposition: Option<String>,
    ) -> crate::Result<String> {
        use openssl::sha;

        if duration > 604800 {
            let msg = format!(
                "duration may not be greater than 604800, but was {}",
                duration
            );
            return Err(Error::Other(msg));
        }

        // 0 Sort and construct the canonical headers
        let mut headers = vec![];
        headers.push(("host".to_string(), "storage.googleapis.com".to_string()));
        headers.sort_unstable_by(|(k1, _), (k2, _)| k1.cmp(&k2));
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
        let issue_date = chrono::Utc::now();
        let file_path = self.path_to_resource(file_path);
        let query_string = Self::get_canonical_query_string(
            &issue_date,
            duration,
            &signed_headers,
            content_disposition,
        );
        let canonical_request =
            self.get_canonical_request(&file_path, &query_string, http_verb, &canonical_headers);

        // 2 get hex encoded SHA256 hash the canonical request
        let hash = sha::sha256(canonical_request.as_bytes());
        let hex_hash = hex::encode(hash);

        // 3 construct the string to sign
        let string_to_sign = format!(
            "{signing_algorithm}\n\
            {current_datetime}\n\
            {credential_scope}\n\
            {hashed_canonical_request}",
            signing_algorithm = "GOOG4-RSA-SHA256",
            current_datetime = issue_date.format("%Y%m%dT%H%M%SZ"),
            credential_scope = Self::get_credential_scope(&issue_date),
            hashed_canonical_request = hex_hash,
        );

        // 4 sign the string to sign with RSA - SHA256
        let buffer = Self::sign_str(&string_to_sign)?;
        let signature = hex::encode(&buffer);

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
            // canonical_headers = "host:storage.googleapis.com",
            canonical_headers = headers,
            signed_headers = "host",
            payload = "UNSIGNED-PAYLOAD",
        )
    }

    #[inline(always)]
    fn get_canonical_query_string(
        date: &chrono::DateTime<chrono::Utc>,
        exp: u32,
        headers: &str,
        content_disposition: Option<String>,
    ) -> String {
        let credential = format!(
            "{authorizer}/{scope}",
            authorizer = crate::SERVICE_ACCOUNT.client_email,
            scope = Self::get_credential_scope(date),
        );
        let mut s = format!(
            "X-Goog-Algorithm={algo}&\
            X-Goog-Credential={cred}&\
            X-Goog-Date={date}&\
            X-Goog-Expires={exp}&\
            X-Goog-SignedHeaders={signed}",
            algo = "GOOG4-RSA-SHA256",
            cred = percent_encode(&credential),
            date = date.format("%Y%m%dT%H%M%SZ"),
            exp = exp,
            // signed="host",
            signed = headers,
        );
        if let Some(cd) = content_disposition {
            s.push_str(&format!("&response-content-disposition={}", cd));
        }
        s
    }

    #[inline(always)]
    fn path_to_resource(&self, path: &str) -> String {
        format!(
            "/{bucket}/{file_path}",
            bucket = self.bucket,
            file_path = percent_encode_noslash(path),
        )
    }

    #[inline(always)]
    fn get_credential_scope(date: &chrono::DateTime<chrono::Utc>) -> String {
        format!("{}/henk/storage/goog4_request", date.format("%Y%m%d"))
    }

    #[inline(always)]
    fn sign_str(message: &str) -> Result<Vec<u8>, Error> {
        use openssl::{hash::MessageDigest, pkey::PKey, sign::Signer};

        let key = PKey::private_key_from_pem(crate::SERVICE_ACCOUNT.private_key.as_bytes())?;
        let mut signer = Signer::new(MessageDigest::sha256(), &key)?;
        signer.update(message.as_bytes())?;
        Ok(signer.sign_to_vec()?)
    }
}

const ENCODE_SET: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'*')
    .remove(b'-')
    .remove(b'.')
    .remove(b'_');

const NOSLASH_ENCODE_SET: &AsciiSet = &ENCODE_SET.remove(b'/').remove(b'~');

// We need to be able to percent encode stuff, but without touching the slashes in filenames. To
// this end we create an implementation that does this, without touching the slashes.
fn percent_encode_noslash(input: &str) -> String {
    utf8_percent_encode(input, NOSLASH_ENCODE_SET).to_string()
}

fn percent_encode(input: &str) -> String {
    utf8_percent_encode(input, ENCODE_SET).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{StreamExt, TryStreamExt};

    #[tokio::test]
    async fn create() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::read_test_bucket().await;
        Object::create(&bucket.name, vec![0, 1], "test-create", "text/plain").await?;
        Ok(())
    }

    #[tokio::test]
    async fn create_streamed() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::read_test_bucket().await;
        let stream = stream::iter([0u8, 1].iter())
            .map(Ok::<_, Box<dyn std::error::Error + Send + Sync>>)
            .map_ok(|&b| bytes::BytesMut::from(&[b][..]));
        Object::create_streamed(
            &bucket.name,
            stream,
            2,
            "test-create-streamed",
            "text/plain",
        )
        .await?;
        Ok(())
    }

    #[tokio::test]
    async fn list() -> Result<(), Box<dyn std::error::Error>> {
        let test_bucket = crate::read_test_bucket().await;
        let _v: Vec<Object> = Object::list(&test_bucket.name).await?.try_concat().await?;
        Ok(())
    }

    async fn flattened_list_prefix_stream(
        bucket: &str,
        prefix: &str,
    ) -> Result<Vec<Object>, Box<dyn std::error::Error>> {
        Ok(Object::list_prefix(bucket, prefix)
            .await?
            .try_concat()
            .await?)
    }

    #[tokio::test]
    async fn list_prefix() -> Result<(), Box<dyn std::error::Error>> {
        let test_bucket = crate::read_test_bucket().await;

        let prefix_names = [
            "test-list-prefix/1",
            "test-list-prefix/2",
            "test-list-prefix/sub/1",
            "test-list-prefix/sub/2",
        ];

        for name in &prefix_names {
            Object::create(&test_bucket.name, vec![0, 1], name, "text/plain").await?;
        }

        let list = flattened_list_prefix_stream(&test_bucket.name, "test-list-prefix/").await?;
        assert_eq!(list.len(), 4);
        let list = flattened_list_prefix_stream(&test_bucket.name, "test-list-prefix/sub").await?;
        assert_eq!(list.len(), 2);
        Ok(())
    }

    #[tokio::test]
    async fn read() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::read_test_bucket().await;
        Object::create(&bucket.name, vec![0, 1], "test-read", "text/plain").await?;
        Object::read(&bucket.name, "test-read").await?;
        Ok(())
    }

    #[tokio::test]
    async fn download() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::read_test_bucket().await;
        let content = b"hello world";
        Object::create(
            &bucket.name,
            content.to_vec(),
            "test-download",
            "application/octet-stream",
        )
        .await?;

        let data = Object::download(&bucket.name, "test-download").await?;
        assert_eq!(data, content);

        Ok(())
    }

    #[tokio::test]
    async fn download_streamed() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::read_test_bucket().await;
        let content = b"hello world";
        Object::create(
            &bucket.name,
            content.to_vec(),
            "test-download",
            "application/octet-stream",
        )
        .await?;

        let mut result = Object::download_streamed(&bucket.name, "test-download").await?;
        let mut data = Vec::new();
        for part in result.next().await {
            data.extend(part?);
        }
        // let data = data.next().await.flat_map(|part| part.into_iter()).collect();
        assert_eq!(data, content);

        Ok(())
    }

    #[tokio::test]
    async fn download_streamed_large() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::read_test_bucket().await;
        let content = vec![5u8; 1_000_000];
        Object::create(
            &bucket.name,
            content.to_vec(),
            "test-download-large",
            "application/octet-stream",
        )
        .await?;

        let mut result = Object::download_streamed(&bucket.name, "test-download-large").await?;
        let mut data: Vec<u8> = Vec::new();
        while let Some(part) = result.next().await {
            data.extend(part?);
        }
        assert_eq!(data, content);

        Ok(())
    }

    #[tokio::test]
    async fn update() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::read_test_bucket().await;
        let mut obj = Object::create(&bucket.name, vec![0, 1], "test-update", "text/plain").await?;
        obj.content_type = Some("application/xml".to_string());
        obj.update().await?;
        Ok(())
    }

    #[tokio::test]
    async fn delete() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::read_test_bucket().await;
        Object::create(&bucket.name, vec![0, 1], "test-delete", "text/plain").await?;

        Object::delete(&bucket.name, "test-delete").await?;

        let list: Vec<_> = flattened_list_prefix_stream(&bucket.name, "test-delete").await?;
        assert!(list.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn delete_nonexistent() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::read_test_bucket().await;

        let nonexistent_object = "test-delete-nonexistent";

        let delete_result = Object::delete(&bucket.name, nonexistent_object).await;

        if let Err(Error::Google(google_error_response)) = delete_result {
            assert!(google_error_response.to_string().contains(&format!(
                "No such object: {}/{}",
                bucket.name, nonexistent_object
            )));
        } else {
            panic!("Expected a Google error, instead got {:?}", delete_result);
        }

        Ok(())
    }

    #[tokio::test]
    async fn compose() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::read_test_bucket().await;
        let obj1 = Object::create(&bucket.name, vec![0, 1], "test-compose-1", "text/plain").await?;
        let obj2 = Object::create(&bucket.name, vec![2, 3], "test-compose-2", "text/plain").await?;
        let compose_request = ComposeRequest {
            kind: "storage#composeRequest".to_string(),
            source_objects: vec![
                SourceObject {
                    name: obj1.name.clone(),
                    generation: None,
                    object_preconditions: None,
                },
                SourceObject {
                    name: obj2.name.clone(),
                    generation: None,
                    object_preconditions: None,
                },
            ],
            destination: None,
        };
        let obj3 = Object::compose(&bucket.name, &compose_request, "test-concatted-file").await?;
        let url = obj3.download_url(100)?;
        let content = reqwest::get(&url).await?.text().await?;
        assert_eq!(content.as_bytes(), &[0, 1, 2, 3]);
        Ok(())
    }

    #[tokio::test]
    async fn copy() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::read_test_bucket().await;
        let original = Object::create(&bucket.name, vec![2, 3], "test-copy", "text/plain").await?;
        original.copy(&bucket.name, "test-copy - copy").await?;
        Ok(())
    }

    #[tokio::test]
    async fn rewrite() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::read_test_bucket().await;
        let obj = Object::create(&bucket.name, vec![0, 1], "test-rewrite", "text/plain").await?;
        let obj = obj.rewrite(&bucket.name, "test-rewritten").await?;
        let url = obj.download_url(100)?;
        let download = crate::CLIENT.head(&url).send().await?;
        assert_eq!(download.status().as_u16(), 200);
        Ok(())
    }

    #[tokio::test]
    async fn test_url_encoding() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::read_test_bucket().await;
        let complicated_names = [
            "asdf",
            "asdf+1",
            "asdf&&+1?=3,,-_()*&^%$#@!`~{}[]\\|:;\"'<>,.?/äöüëß",
            "https://www.google.com",
            "परिक्षण फाईल",
            "测试很重要",
        ];
        for name in &complicated_names {
            let _obj = Object::create(&bucket.name, vec![0, 1], name, "text/plain").await?;
            let obj = Object::read(&bucket.name, &name).await.unwrap();
            let url = obj.download_url(100)?;
            let download = crate::CLIENT.head(&url).send().await?;
            assert_eq!(download.status().as_u16(), 200);
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_download_url_with() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::read_test_bucket().await;
        let client = reqwest::Client::new();
        let obj = Object::create(&bucket.name, vec![0, 1], "test-rewrite", "text/plain").await?;

        let opts1 = crate::DownloadOptions::new().content_disposition("attachment");
        let download_url1 = obj.download_url_with(100, opts1)?;
        let download1 = client.head(&download_url1).send().await?;
        assert_eq!(download1.headers()["content-disposition"], "attachment");
        Ok(())
    }

    #[cfg(feature = "sync")]
    mod sync {
        use super::*;

        #[test]
        fn create() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::read_test_bucket_sync();
            Object::create_sync(&bucket.name, vec![0, 1], "test-create", "text/plain")?;
            Ok(())
        }

        #[test]
        fn create_streamed() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::read_test_bucket_sync();
            let cursor = std::io::Cursor::new([0, 1]);
            Object::create_streamed_sync(
                &bucket.name,
                cursor,
                2,
                "test-create-streamed",
                "text/plain",
            )?;
            Ok(())
        }

        #[test]
        fn list() -> Result<(), Box<dyn std::error::Error>> {
            let test_bucket = crate::read_test_bucket_sync();
            Object::list_sync(&test_bucket.name)?;
            Ok(())
        }

        #[test]
        fn list_prefix() -> Result<(), Box<dyn std::error::Error>> {
            let test_bucket = crate::read_test_bucket_sync();

            let prefix_names = [
                "test-list-prefix/1",
                "test-list-prefix/2",
                "test-list-prefix/sub/1",
                "test-list-prefix/sub/2",
            ];

            for name in &prefix_names {
                Object::create_sync(&test_bucket.name, vec![0, 1], name, "text/plain")?;
            }

            let list = Object::list_prefix_sync(&test_bucket.name, "test-list-prefix/")?;
            assert_eq!(list.len(), 4);
            let list = Object::list_prefix_sync(&test_bucket.name, "test-list-prefix/sub")?;
            assert_eq!(list.len(), 2);
            Ok(())
        }

        #[test]
        fn read() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::read_test_bucket_sync();
            Object::create_sync(&bucket.name, vec![0, 1], "test-read", "text/plain")?;
            Object::read_sync(&bucket.name, "test-read")?;
            Ok(())
        }

        #[test]
        fn download() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::read_test_bucket_sync();
            let content = b"hello world";
            Object::create_sync(
                &bucket.name,
                content.to_vec(),
                "test-download",
                "application/octet-stream",
            )?;

            let data = Object::download_sync(&bucket.name, "test-download")?;
            assert_eq!(data, content);

            Ok(())
        }

        #[test]
        fn update() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::read_test_bucket_sync();
            let mut obj =
                Object::create_sync(&bucket.name, vec![0, 1], "test-update", "text/plain")?;
            obj.content_type = Some("application/xml".to_string());
            obj.update_sync()?;
            Ok(())
        }

        #[test]
        fn delete() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::read_test_bucket_sync();
            Object::create_sync(&bucket.name, vec![0, 1], "test-delete", "text/plain")?;

            Object::delete_sync(&bucket.name, "test-delete")?;

            let list = Object::list_prefix_sync(&bucket.name, "test-delete")?;
            assert!(list.is_empty());

            Ok(())
        }

        #[test]
        fn delete_nonexistent() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::read_test_bucket_sync();

            let nonexistent_object = "test-delete-nonexistent";

            let delete_result = Object::delete_sync(&bucket.name, nonexistent_object);

            if let Err(Error::Google(google_error_response)) = delete_result {
                assert!(google_error_response.to_string().contains(&format!(
                    "No such object: {}/{}",
                    bucket.name, nonexistent_object
                )));
            } else {
                panic!("Expected a Google error, instead got {:?}", delete_result);
            }

            Ok(())
        }

        #[test]
        fn compose() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::read_test_bucket_sync();
            let obj1 =
                Object::create_sync(&bucket.name, vec![0, 1], "test-compose-1", "text/plain")?;
            let obj2 =
                Object::create_sync(&bucket.name, vec![2, 3], "test-compose-2", "text/plain")?;
            let compose_request = ComposeRequest {
                kind: "storage#composeRequest".to_string(),
                source_objects: vec![
                    SourceObject {
                        name: obj1.name.clone(),
                        generation: None,
                        object_preconditions: None,
                    },
                    SourceObject {
                        name: obj2.name.clone(),
                        generation: None,
                        object_preconditions: None,
                    },
                ],
                destination: None,
            };
            let obj3 = Object::compose_sync(&bucket.name, &compose_request, "test-concatted-file")?;
            let url = obj3.download_url(100)?;
            let content = reqwest::blocking::get(&url)?.text()?;
            assert_eq!(content.as_bytes(), &[0, 1, 2, 3]);
            Ok(())
        }

        #[test]
        fn copy() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::read_test_bucket_sync();
            let original =
                Object::create_sync(&bucket.name, vec![2, 3], "test-copy", "text/plain")?;
            original.copy_sync(&bucket.name, "test-copy - copy")?;
            Ok(())
        }

        #[test]
        fn rewrite() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::read_test_bucket_sync();
            let obj = Object::create_sync(&bucket.name, vec![0, 1], "test-rewrite", "text/plain")?;
            let obj = obj.rewrite_sync(&bucket.name, "test-rewritten")?;
            let url = obj.download_url(100)?;
            let client = reqwest::blocking::Client::new();
            let download = client.head(&url).send()?;
            assert_eq!(download.status().as_u16(), 200);
            Ok(())
        }

        #[test]
        fn test_url_encoding() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::read_test_bucket_sync();
            let complicated_names = [
                "asdf",
                "asdf+1",
                "asdf&&+1?=3,,-_()*&^%$#@!`~{}[]\\|:;\"'<>,.?/äöüëß",
                "https://www.google.com",
                "परिक्षण फाईल",
                "测试很重要",
            ];
            for name in &complicated_names {
                let _obj = Object::create_sync(&bucket.name, vec![0, 1], name, "text/plain")?;
                let obj = Object::read_sync(&bucket.name, &name).unwrap();
                let url = obj.download_url(100)?;
                let client = reqwest::blocking::Client::new();
                let download = client.head(&url).send()?;
                assert_eq!(download.status().as_u16(), 200);
            }
            Ok(())
        }
    }
}

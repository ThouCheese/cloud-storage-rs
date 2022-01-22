use futures_util::{stream, Stream, TryStream};
use reqwest::StatusCode;

use crate::{
    error::GoogleResponse,
    object::{percent_encode, ComposeRequest, ObjectList, RewriteResponse, SizedByteStream},
    ListRequest, Object,
};

// Object uploads has its own url for some reason
const BASE_URL: &str = "https://storage.googleapis.com/upload/storage/v1/b";

/// Operations on [`Object`](Object)s.
#[derive(Debug)]
pub struct ObjectClient<'a>(pub(super) &'a super::Client);

impl<'a> ObjectClient<'a> {
    /// Create a new object.
    /// Upload a file as that is loaded in memory to google cloud storage, where it will be
    /// interpreted according to the mime type you specified.
    /// ## Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # fn read_cute_cat(_in: &str) -> Vec<u8> { vec![0, 1] }
    /// use cloud_storage::Client;
    /// use cloud_storage::Object;
    ///
    /// let file: Vec<u8> = read_cute_cat("cat.png");
    /// let client = Client::default();
    /// client.object().create("cat-photos", file, "recently read cat.png", "image/png").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(
        &self,
        bucket: &str,
        file: Vec<u8>,
        filename: &str,
        mime_type: &str,
    ) -> crate::Result<Object> {
        use reqwest::header::{CONTENT_LENGTH, CONTENT_TYPE};

        let url = &format!(
            "{}/{}/o?uploadType=media&name={}",
            BASE_URL,
            percent_encode(bucket),
            percent_encode(filename),
        );
        let mut headers = self.0.get_headers().await?;
        headers.insert(CONTENT_TYPE, mime_type.parse()?);
        headers.insert(CONTENT_LENGTH, file.len().to_string().parse()?);
        let response = self
            .0
            .client
            .post(url)
            .headers(headers)
            .body(file)
            .send()
            .await?;
        if response.status() == 200 {
            Ok(serde_json::from_str(&response.text().await?)?)
        } else {
            Err(crate::Error::new(&response.text().await?))
        }
    }

    /// Create a new object. This works in the same way as `ObjectClient::create`, except it does not need
    /// to load the entire file in ram.
    /// ## Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Client;
    /// use cloud_storage::Object;
    ///
    /// let client = Client::default();
    /// let file = reqwest::Client::new()
    ///     .get("https://my_domain.rs/nice_cat_photo.png")
    ///     .send()
    ///     .await?
    ///     .bytes_stream();
    /// client.object().create_streamed("cat-photos", file, 10, "recently read cat.png", "image/png").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_streamed<S>(
        &self,
        bucket: &str,
        stream: S,
        length: impl Into<Option<u64>>,
        filename: &str,
        mime_type: &str,
    ) -> crate::Result<Object>
    where
        S: TryStream + Send + Sync + 'static,
        S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
        bytes::Bytes: From<S::Ok>,
    {
        use reqwest::header::{CONTENT_LENGTH, CONTENT_TYPE};

        let url = &format!(
            "{}/{}/o?uploadType=media&name={}",
            BASE_URL,
            percent_encode(bucket),
            percent_encode(filename),
        );
        let mut headers = self.0.get_headers().await?;
        headers.insert(CONTENT_TYPE, mime_type.parse()?);
        if let Some(length) = length.into() {
            headers.insert(CONTENT_LENGTH, length.into());
        }

        let body = reqwest::Body::wrap_stream(stream);
        let response = self
            .0
            .client
            .post(url)
            .headers(headers)
            .body(body)
            .send()
            .await?;
        if response.status() == 200 {
            Ok(serde_json::from_str(&response.text().await?)?)
        } else {
            Err(crate::Error::new(&response.text().await?))
        }
    }

    /// Obtain a list of objects within this Bucket.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Client;
    /// use cloud_storage::{Object, ListRequest};
    ///
    /// let client = Client::default();
    /// let all_objects = client.object().list("my_bucket", ListRequest::default()).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(
        &self,
        bucket: &'a str,
        list_request: ListRequest,
    ) -> crate::Result<impl Stream<Item = crate::Result<ObjectList>> + 'a> {
        enum ListState {
            Start(ListRequest),
            HasMore(ListRequest),
            Done,
        }
        use ListState::*;
        impl ListState {
            fn into_has_more(self) -> Option<ListState> {
                match self {
                    Start(req) | HasMore(req) => Some(HasMore(req)),
                    Done => None,
                }
            }

            fn req_mut(&mut self) -> Option<&mut ListRequest> {
                match self {
                    Start(ref mut req) | HasMore(ref mut req) => Some(req),
                    Done => None,
                }
            }
        }

        let client = self.0;

        Ok(stream::unfold(
            ListState::Start(list_request),
            move |mut state| async move {
                let url = format!("{}/b/{}/o", crate::BASE_URL, percent_encode(bucket));
                let headers = match client.get_headers().await {
                    Ok(h) => h,
                    Err(e) => return Some((Err(e), state)),
                };
                let req = state.req_mut()?;
                if req.max_results == Some(0) {
                    return None;
                }

                let response = client
                    .client
                    .get(&url)
                    .query(req)
                    .headers(headers)
                    .send()
                    .await;

                let response = match response {
                    Ok(r) if r.status() == 200 => r,
                    Ok(r) => {
                        let e = match r.json::<crate::error::GoogleErrorResponse>().await {
                            Ok(err_res) => err_res.into(),
                            Err(serde_err) => serde_err.into(),
                        };
                        return Some((Err(e), state));
                    }
                    Err(e) => return Some((Err(e.into()), state)),
                };

                let result: GoogleResponse<ObjectList> = match response.json().await {
                    Ok(json) => json,
                    Err(e) => return Some((Err(e.into()), state)),
                };

                let response_body = match result {
                    GoogleResponse::Success(success) => success,
                    GoogleResponse::Error(e) => return Some((Err(e.into()), state)),
                };

                let next_state = if let Some(ref page_token) = response_body.next_page_token {
                    req.page_token = Some(page_token.clone());
                    req.max_results = req
                        .max_results
                        .map(|rem| rem.saturating_sub(response_body.items.len()));
                    state.into_has_more()?
                } else {
                    Done
                };

                Some((Ok(response_body), next_state))
            },
        ))
    }

    /// Obtains a single object with the specified name in the specified bucket.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Client;
    /// use cloud_storage::Object;
    ///
    /// let client = Client::default();
    /// let object = client.object().read("my_bucket", "path/to/my/file.png").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn read(&self, bucket: &str, file_name: &str) -> crate::Result<Object> {
        let url = format!(
            "{}/b/{}/o/{}",
            crate::BASE_URL,
            percent_encode(bucket),
            percent_encode(file_name),
        );
        let result: GoogleResponse<Object> = self
            .0
            .client
            .get(&url)
            .headers(self.0.get_headers().await?)
            .send()
            .await?
            .json()
            .await?;
        match result {
            GoogleResponse::Success(s) => Ok(s),
            GoogleResponse::Error(e) => Err(e.into()),
        }
    }

    /// Download the content of the object with the specified name in the specified bucket.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Client;
    /// use cloud_storage::Object;
    ///
    /// let client = Client::default();
    /// let bytes = client.object().download("my_bucket", "path/to/my/file.png").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download(&self, bucket: &str, file_name: &str) -> crate::Result<Vec<u8>> {
        let url = format!(
            "{}/b/{}/o/{}?alt=media",
            crate::BASE_URL,
            percent_encode(bucket),
            percent_encode(file_name),
        );
        let resp = self
            .0
            .client
            .get(&url)
            .headers(self.0.get_headers().await?)
            .send()
            .await?;
        if resp.status() == StatusCode::NOT_FOUND {
            Err(crate::Error::Other(resp.text().await?))
        } else {
            Ok(resp.error_for_status()?.bytes().await?.to_vec())
        }
    }

    /// Download the content of the object with the specified name in the specified bucket, without
    /// allocating the whole file into a vector.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Client;
    /// use cloud_storage::Object;
    /// use futures::StreamExt;
    /// use tokio::fs::File;
    /// use tokio::io::{AsyncWriteExt, BufWriter};
    ///
    /// let client = Client::default();
    /// let mut stream = client.object().download_streamed("my_bucket", "path/to/my/file.png").await?;
    /// let mut file = BufWriter::new(File::create("file.png").await.unwrap());
    /// while let Some(byte) = stream.next().await {
    ///     file.write_all(&[byte.unwrap()]).await.unwrap();
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download_streamed(
        &self,
        bucket: &str,
        file_name: &str,
    ) -> crate::Result<impl Stream<Item = crate::Result<u8>> + Unpin> {
        use futures_util::{StreamExt, TryStreamExt};
        let url = format!(
            "{}/b/{}/o/{}?alt=media",
            crate::BASE_URL,
            percent_encode(bucket),
            percent_encode(file_name),
        );
        let response = self
            .0
            .client
            .get(&url)
            .headers(self.0.get_headers().await?)
            .send()
            .await?
            .error_for_status()?;
        let size = response.content_length();
        let bytes = response
            .bytes_stream()
            .map(|chunk| chunk.map(|c| futures_util::stream::iter(c.into_iter().map(Ok))))
            .try_flatten();
        Ok(SizedByteStream::new(bytes, size))
    }

    /// Obtains a single object with the specified name in the specified bucket.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Client;
    /// use cloud_storage::Object;
    ///
    /// let client = Client::default();
    /// let mut object = client.object().read("my_bucket", "path/to/my/file.png").await?;
    /// object.content_type = Some("application/xml".to_string());
    /// client.object().update(&object).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(&self, object: &Object) -> crate::Result<Object> {
        let url = format!(
            "{}/b/{}/o/{}",
            crate::BASE_URL,
            percent_encode(&object.bucket),
            percent_encode(&object.name),
        );
        let result: GoogleResponse<Object> = self
            .0
            .client
            .put(&url)
            .headers(self.0.get_headers().await?)
            .json(&object)
            .send()
            .await?
            .json()
            .await?;
        match result {
            GoogleResponse::Success(s) => Ok(s),
            GoogleResponse::Error(e) => Err(e.into()),
        }
    }

    /// Deletes a single object with the specified name in the specified bucket.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Client;
    /// use cloud_storage::Object;
    ///
    /// let client = Client::default();
    /// client.object().delete("my_bucket", "path/to/my/file.png").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, bucket: &str, file_name: &str) -> crate::Result<()> {
        let url = format!(
            "{}/b/{}/o/{}",
            crate::BASE_URL,
            percent_encode(bucket),
            percent_encode(file_name),
        );
        let response = self
            .0
            .client
            .delete(&url)
            .headers(self.0.get_headers().await?)
            .send()
            .await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(crate::Error::Google(response.json().await?))
        }
    }

    /// Obtains a single object with the specified name in the specified bucket.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Client;
    /// use cloud_storage::object::{Object, ComposeRequest, SourceObject};
    ///
    /// let client = Client::default();
    /// let obj1 = client.object().read("my_bucket", "file1").await?;
    /// let obj2 = client.object().read("my_bucket", "file2").await?;
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
    /// let obj3 = client.object().compose("my_bucket", &compose_request, "test-concatted-file").await?;
    /// // obj3 is now a file with the content of obj1 and obj2 concatted together.
    /// # Ok(())
    /// # }
    /// ```
    pub async fn compose(
        &self,
        bucket: &str,
        req: &ComposeRequest,
        destination_object: &str,
    ) -> crate::Result<Object> {
        let url = format!(
            "{}/b/{}/o/{}/compose",
            crate::BASE_URL,
            percent_encode(bucket),
            percent_encode(destination_object)
        );
        let result: GoogleResponse<Object> = self
            .0
            .client
            .post(&url)
            .headers(self.0.get_headers().await?)
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

    /// Copy this object to the target bucket and path
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Client;
    /// use cloud_storage::object::{Object, ComposeRequest};
    ///
    /// let client = Client::default();
    /// let obj1 = client.object().read("my_bucket", "file1").await?;
    /// let obj2 = client.object().copy(&obj1, "my_other_bucket", "file2").await?;
    /// // obj2 is now a copy of obj1.
    /// # Ok(())
    /// # }
    /// ```
    pub async fn copy(
        &self,
        object: &Object,
        destination_bucket: &str,
        path: &str,
    ) -> crate::Result<Object> {
        use reqwest::header::CONTENT_LENGTH;

        let url = format!(
            "{base}/b/{sBucket}/o/{sObject}/copyTo/b/{dBucket}/o/{dObject}",
            base = crate::BASE_URL,
            sBucket = percent_encode(&object.bucket),
            sObject = percent_encode(&object.name),
            dBucket = percent_encode(destination_bucket),
            dObject = percent_encode(path),
        );
        let mut headers = self.0.get_headers().await?;
        headers.insert(CONTENT_LENGTH, "0".parse()?);
        let result: GoogleResponse<Object> = self
            .0
            .client
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
    /// use cloud_storage::Client;
    /// use cloud_storage::object::Object;
    ///
    /// let client = Client::default();
    /// let obj1 = client.object().read("my_bucket", "file1").await?;
    /// let obj2 = client.object().rewrite(&obj1, "my_other_bucket", "file2").await?;
    /// // obj2 is now a copy of obj1.
    /// # Ok(())
    /// # }
    /// ```
    pub async fn rewrite(
        &self,
        object: &Object,
        destination_bucket: &str,
        path: &str,
    ) -> crate::Result<Object> {
        use reqwest::header::CONTENT_LENGTH;

        let url = format!(
            "{base}/b/{sBucket}/o/{sObject}/rewriteTo/b/{dBucket}/o/{dObject}",
            base = crate::BASE_URL,
            sBucket = percent_encode(&object.bucket),
            sObject = percent_encode(&object.name),
            dBucket = percent_encode(destination_bucket),
            dObject = percent_encode(path),
        );
        let mut headers = self.0.get_headers().await?;
        headers.insert(CONTENT_LENGTH, "0".parse()?);
        let s = self
            .0
            .client
            .post(&url)
            .headers(headers)
            .send()
            .await?
            .text()
            .await?;

        let result: RewriteResponse = serde_json::from_str(dbg!(&s)).unwrap();
        Ok(result.resource)
        // match result {
        // GoogleResponse::Success(s) => Ok(s.resource),
        // GoogleResponse::Error(e) => Err(e.into()),
        // }
    }
}

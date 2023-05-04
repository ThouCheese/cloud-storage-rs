use futures_util::{Stream, stream, TryStream};
use crate::{models::{CreateParameters, ObjectList, ReadParameters, UpdateParameters, DeleteParameters, ComposeRequest, ComposeParameters, CopyParameters, RewriteParameters, Response, rewrite_response::RewriteResponse}, Object, Error, ListRequest, sized_byte_stream::SizedByteStream};

/// Operations on [`Object`](Object)s.
#[derive(Debug)]
pub struct ObjectClient<'a> {
    pub(crate) client: &'a super::CloudStorageClient,
    pub(crate) base_url: String,
    pub(crate) insert_url: String,
}

impl<'a> ObjectClient<'a> {
    /// Create a new object.
    /// Upload a file as that is loaded in memory to google cloud storage, where it will be
    /// interpreted according to the mime type you specified.
    /// ## Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # fn read_cute_cat(_in: &str) -> Vec<u8> { vec![0, 1] }
    /// # use cloud_storage::CloudStorageClient;
    /// # use cloud_storage::Object;
    ///
    /// let file: Vec<u8> = read_cute_cat("cat.png");
    /// let client = CloudStorageClient::default();
    /// client.object("cat-photos").create(file, "recently read cat.png", "image/png", None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(
        &self,
        file: Vec<u8>,
        filename: &str,
        mime_type: &str,
        parameters: Option<CreateParameters>,
    ) -> Result<Object, Error> {
        use reqwest::header::{CONTENT_LENGTH, CONTENT_TYPE};

        let url = &format!("{}?name={}&uploadType=media", self.insert_url, crate::percent_encode(filename));
        let mut headers = self.client.get_headers().await?;
        headers.insert(CONTENT_TYPE, mime_type.parse()?);
        headers.insert(CONTENT_LENGTH, file.len().to_string().parse()?);
        let response = self.client.reqwest
            .post(url)
            .query(&parameters)
            .headers(headers)
            .body(file)
            .send()
            .await?;

        let mut object = response.json::<Response<Object>>().await??;
        object.private_key = Some(self.client.service_account.private_key.clone());
        object.client_email = Some(self.client.service_account.client_email.clone());
        Ok(object)
    }

    /// Create a new object. This works in the same way as `ObjectClient::create` but allows setting of metadata for this object.
    /// Upload a file as that is loaded in memory to google cloud storage, where it will be
    /// interpreted according to the mime type you specified. The metadata will be set at the time of creation.
    /// ## Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # fn read_cute_cat(_in: &str) -> Vec<u8> { vec![0, 1] }
    /// # use cloud_storage::CloudStorageClient;
    /// # use cloud_storage::Object;
    ///
    /// let file: Vec<u8> = read_cute_cat("cat.png");
    /// let client = CloudStorageClient::default();
    /// let metadata = serde_json::json!({
    ///     "metadata": {
    ///         "custom_id": "1234"
    ///     }
    /// });
    /// client.object("cat-photos").create_with(file, "recently read cat.png", "image/png", &metadata).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_with(
        &self,
        file: Vec<u8>,
        filename: &str,
        mime_type: &str,
        metadata: &serde_json::Value,
    ) -> Result<Object, Error> {
        let url = &format!("{}?name={}&uploadType=multipart", self.insert_url, crate::percent_encode(filename));

        // single-request upload that includes metadata require a mutlipart request where
        // part 1 is metadata, and part2 is the file to upload
        let metadata_part =
            reqwest::multipart::Part::text(metadata.to_string()).mime_str("application/json")?;
        let file_part = reqwest::multipart::Part::bytes(file).mime_str(mime_type)?;
        let form = reqwest::multipart::Form::new()
            .part("metadata", metadata_part)
            .part("file", file_part);
        let headers = self.client.get_headers().await?;
        let response = self.client.reqwest
            .post(url)
            .headers(headers)
            .multipart(form)
            .send()
            .await?;
        let mut object = response.json::<Response<Object>>().await??;
        object.private_key = Some(self.client.service_account.private_key.clone());
        object.client_email = Some(self.client.service_account.client_email.clone());
        Ok(object)
    }

    /// Create a new object. This works in the same way as `ObjectClient::create`, except it does not need
    /// to load the entire file in ram.
    /// ## Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::CloudStorageClient;
    /// # use cloud_storage::Object;
    ///
    /// let client = CloudStorageClient::default();
    /// let file = reqwest::Client::new()
    ///     .get("https://my_domain.rs/nice_cat_photo.png")
    ///     .send()
    ///     .await?
    ///     .bytes_stream();
    /// let metadata = serde_json::json!({
    ///     "metadata": {
    ///         "custom_id": "1234"
    ///     }
    /// });
    /// client.object("cat-photos").create_streamed_with(file, "recently read cat.png", "image/png", &metadata).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_streamed_with<S>(
        &self,
        stream: S,
        filename: &str,
        mime_type: &str,
        metadata: &serde_json::Value,
    ) -> Result<Object, Error>
    where
        S: TryStream + Send + Sync + 'static,
        S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
        bytes::Bytes: From<S::Ok>,
    {
        let url = &format!("{}?name={}&uploadType=multipart", self.insert_url, crate::percent_encode(filename));
        let headers = self.client.get_headers().await?;

        // single-request upload that includes metadata require a mutlipart request where
        // part 1 is metadata, and part2 is the file to upload
        let body = reqwest::Body::wrap_stream(stream);
        let metadata_part =
            reqwest::multipart::Part::text(metadata.to_string()).mime_str("application/json")?;
        let file_part = reqwest::multipart::Part::stream(body).mime_str(mime_type)?;
        let form = reqwest::multipart::Form::new()
            .part("metadata", metadata_part)
            .part("file", file_part);

        let response = self.client.reqwest
            .post(url)
            .headers(headers)
            .multipart(form)
            .send()
            .await?;
        let mut object = response.json::<Response<Object>>().await??;
        object.private_key = Some(self.client.service_account.private_key.clone());
        object.client_email = Some(self.client.service_account.client_email.clone());
        Ok(object)
    }

    /// Create a new object. This works in the same way as `ObjectClient::create`, except it does not need
    /// to load the entire file in ram.
    /// ## Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::CloudStorageClient;
    /// # use cloud_storage::Object;
    ///
    /// let client = CloudStorageClient::default();
    /// let file = reqwest::Client::new()
    ///     .get("https://my_domain.rs/nice_cat_photo.png")
    ///     .send()
    ///     .await?
    ///     .bytes_stream();
    /// client.object("cat-photos").create_streamed(file, 10, "recently read cat.png", "image/png", None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_streamed<S>(
        &self,
        stream: S,
        length: impl Into<Option<u64>>,
        filename: &str,
        mime_type: &str,
        parameters: Option<CreateParameters>,
    ) -> Result<Object, Error>
    where
        S: TryStream + Send + Sync + 'static,
        S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
        bytes::Bytes: From<S::Ok>,
    {
        use reqwest::header::{CONTENT_LENGTH, CONTENT_TYPE};

        let url = &format!("{}?name={}&uploadType=media", self.insert_url, crate::percent_encode(filename));
        let mut headers = self.client.get_headers().await?;
        headers.insert(CONTENT_TYPE, mime_type.parse()?);
        if let Some(length) = length.into() {
            headers.insert(CONTENT_LENGTH, length.into());
        }

        let body = reqwest::Body::wrap_stream(stream);
        let response = self.client.reqwest
            .post(url)
            .query(&parameters)
            .headers(headers)
            .body(body)
            .send()
            .await?;
        let mut object = response.json::<Response<Object>>().await??;
        object.private_key = Some(self.client.service_account.private_key.clone());
        object.client_email = Some(self.client.service_account.client_email.clone());
        Ok(object)
    }

    /// Obtain a list of objects within this Bucket.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::CloudStorageClient;
    /// # use cloud_storage::{Object, ListRequest};
    ///
    /// let client = CloudStorageClient::default();
    /// let all_objects = client.object("my_bucket").list(ListRequest::default()).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(
        &self,
        list_request: ListRequest,
    ) -> Result<impl Stream<Item = Result<ObjectList, Error>>, Error> {
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

        let reqwest = self.client.reqwest.clone();
        let headers = self.client.get_headers().await?;
        let url = self.base_url.to_string();

        Ok(stream::unfold(ListState::Start(list_request), move |mut state| {
                let reqwest = reqwest.clone();
                let url = url.clone();
                let headers = headers.clone();
                
                async move {
                    let req = state.req_mut()?;
                    if req.max_results == Some(0) {
                        return None;
                    }

                    let response = reqwest
                        .get(&url)
                        .query(req)
                        .headers(headers.clone())
                        .send()
                        .await;

                    let response = match response {
                        Ok(r) if r.status() == 200 => r,
                        Ok(r) => {
                            let e = match r.json::<crate::models::ErrorResponse>().await {
                                Ok(err_res) => err_res.into(),
                                Err(serde_err) => serde_err.into(),
                            };
                            return Some((Err(e), state));
                        }
                        Err(e) => return Some((Err(e.into()), state)),
                    };

                    let result: crate::models::Response<ObjectList> = match response.json().await {
                        Ok(json) => json,
                        Err(e) => return Some((Err(e.into()), state)),
                    };

                    let response_body = match result {
                        crate::models::Response::Success(success) => success,
                        crate::models::Response::Error(e) => return Some((Err(e.into()), state)),
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
                }
            }
        ))
    }

    /// Obtains a single object with the specified name in the specified bucket.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::CloudStorageClient;
    /// # use cloud_storage::Object;
    ///
    /// let client = CloudStorageClient::default();
    /// let object = client.object("my_bucket").read("path/to/my/file.png", None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn read(
        &self,
        file_name: &str,
        parameters: Option<ReadParameters>,
    ) -> Result<Object, Error> {
        //let paramters = qs::
        let url = format!(
            "{}/{}",
            self.base_url,
            crate::percent_encode(file_name),
        );
        let response = self.client.reqwest
            .get(&url)
            .query(&parameters)
            .headers(self.client.get_headers().await?)
            .send()
            .await?;

        let mut object = response.json::<Response<Object>>().await??;
        object.private_key = Some(self.client.service_account.private_key.clone());
        object.client_email = Some(self.client.service_account.client_email.clone());
        Ok(object)
    }

    /// Download the content of the object with the specified name in the specified bucket.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::CloudStorageClient;
    /// # use cloud_storage::Object;
    ///
    /// let client = CloudStorageClient::default();
    /// let bytes = client.object("my_bucket").download("path/to/my/file.png", None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download(
        &self,
        file_name: &str,
        parameters: Option<ReadParameters>,
    ) -> Result<Vec<u8>, Error> {
        let url = format!(
            "{}/{}?alt=media",
            self.base_url,
            crate::percent_encode(file_name),
        );
        let response = self.client.reqwest
            .get(&url)
            .query(&parameters)
            .headers(self.client.get_headers().await?)
            .send()
            .await?;

            if response.status() == reqwest::StatusCode::NOT_FOUND {
                Err(crate::Error::Other(response.text().await?))
            } else {
                Ok(response.error_for_status()?.bytes().await?.to_vec())
            }
    }

    /// Download the content of the object with the specified name in the specified bucket, without
    /// allocating the whole file into a vector.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::CloudStorageClient;
    /// # use cloud_storage::Object;
    /// # use futures_util::stream::StreamExt;
    /// # use tokio::fs::File;
    /// # use tokio::io::{AsyncWriteExt, BufWriter};
    /// # use bytes::Buf;
    /// let cloud_storage_client = CloudStorageClient::default();
    /// let client = cloud_storage_client.object("my_bucket");
    /// let mut stream = client.download_streamed("path/to/my/file.png", None).await?;
    /// let mut file = BufWriter::new(File::create("file.png").await.unwrap());
    /// while let Some(byte) = stream.next().await {
    ///     file.write_all(byte.unwrap().chunk()).await.unwrap();
    /// }
    /// file.flush().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download_streamed(
        &self,
        file_name: &str,
        parameters: Option<ReadParameters>,
    ) -> Result<impl Stream<Item = Result<bytes::Bytes, Error>> + Unpin, Error> {
        use futures_util::TryStreamExt;
        let url = format!(
            "{}/{}?alt=media",
            self.base_url,
            crate::percent_encode(file_name),
        );
        let response = self.client.reqwest
            .get(&url)
            .query(&parameters)
            .headers(self.client.get_headers().await?)
            .send()
            .await?
            .error_for_status()?;
        let size = response.content_length();
        let bytes = response.bytes_stream().map_err(Error::from);
        Ok(SizedByteStream::new(bytes, size))
    }

    /// Updates a single object with the specified name in the specified bucket with the new
    /// information in `object`.
    ///
    /// Note that if the `name` or `bucket` fields are changed, the object will not be found.
    /// See [`rewrite`](Self::rewrite()) or [`copy`](Self::copy()) for similar operations.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::CloudStorageClient;
    /// # use cloud_storage::Object;
    ///
    /// let cloud_storage_client = CloudStorageClient::default();
    /// let client = cloud_storage_client.object("my_bucket");
    /// let mut object = client.read("path/to/my/file.png", None).await?;
    /// object.content_type = Some("application/xml".to_string());
    /// client.update(&object, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(
        &self,
        object: &Object,
        parameters: Option<UpdateParameters>,
    ) -> Result<Object, Error> {
        let url = format!(
            "{}/{}",
            self.base_url,
            crate::percent_encode(&object.name),
        );
        let response = self.client.reqwest
            .put(&url)
            .query(&parameters)
            .headers(self.client.get_headers().await?)
            .json(&object)
            .send()
            .await?;

        let mut object = response.json::<Response<Object>>().await??;
        object.private_key = Some(self.client.service_account.private_key.clone());
        object.client_email = Some(self.client.service_account.client_email.clone());
        Ok(object)
    }

    /// Deletes a single object with the specified name in the specified bucket.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::CloudStorageClient;
    /// # use cloud_storage::Object;
    ///
    /// let client = CloudStorageClient::default();
    /// client.object("my_bucket").delete("path/to/my/file.png", None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(
        &self,
        file_name: &str,
        parameters: Option<DeleteParameters>,
    ) -> Result<(), Error> {
        let url = format!(
            "{}/{}",
            self.base_url,
            crate::percent_encode(file_name),
        );
        let response = self.client.reqwest
            .delete(&url)
            .query(&parameters)
            .headers(self.client.get_headers().await?)
            .send()
            .await?;

       if response.status().is_success() {
            Ok(())
        } else {
            Err(crate::Error::Google(response.json().await?))
        }
    }

    /// Concatenates the contents of multiple objects into one.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::CloudStorageClient;
    /// # use cloud_storage::models::{Object, ComposeRequest, SourceObject};
    ///
    /// let client = CloudStorageClient::default();
    /// let obj1 = client.object("my_bucket").read("file1", None).await?;
    /// let obj2 = client.object("my_bucket").read("file2", None).await?;
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
    /// let obj3 = client.object("my_bucket").compose(&compose_request, "test-concatted-file", None).await?;
    /// // obj3 is now a file with the content of obj1 and obj2 concatted together.
    /// # Ok(())
    /// # }
    /// ```
    pub async fn compose(
        &self,
        req: &ComposeRequest,
        destination_object: &str,
        parameters: Option<ComposeParameters>,
    ) -> Result<Object, Error> {
        let url = format!(
            "{}/{}/compose",
            self.base_url,
            crate::percent_encode(destination_object)
        );
        let response = self.client.reqwest
            .post(&url)
            .query(&parameters)
            .headers(self.client.get_headers().await?)
            .json(req)
            .send()
            .await?;

        let mut object = response.json::<Response<Object>>().await??;
        object.private_key = Some(self.client.service_account.private_key.clone());
        object.client_email = Some(self.client.service_account.client_email.clone());
        Ok(object)
    }

    /// Copy this object to the target bucket and path.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::CloudStorageClient;
    /// # use cloud_storage::models::{Object, ComposeRequest};
    ///
    /// let client = CloudStorageClient::default();
    /// let obj1 = client.object("my_bucket").read("file1", None).await?;
    /// let obj2 = client.object("my_bucket").copy(&obj1, "my_other_bucket", "file2", None).await?;
    /// // obj2 is now a copy of obj1.
    /// # Ok(())
    /// # }
    /// ```
    pub async fn copy(
        &self,
        object: &Object,
        destination_bucket: &str,
        path: &str,
        parameters: Option<CopyParameters>,
    ) -> Result<Object, Error> {
        use reqwest::header::CONTENT_LENGTH;

        let url = format!(
            "{base}/{sObject}/copyTo/b/{dBucket}/o/{dObject}",
            base = self.base_url,
            sObject = crate::percent_encode(&object.name),
            dBucket = crate::percent_encode(destination_bucket),
            dObject = crate::percent_encode(path),
        );
        let mut headers = self.client.get_headers().await?;
        headers.insert(CONTENT_LENGTH, "0".parse()?);
        let response = self.client.reqwest
            .post(&url)
            .query(&parameters)
            .headers(headers)
            .send()
            .await?;

        let mut object = response.json::<Response<Object>>().await??;
        object.private_key = Some(self.client.service_account.private_key.clone());
        object.client_email = Some(self.client.service_account.client_email.clone());
        Ok(object)
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
    /// # use cloud_storage::CloudStorageClient;
    /// # use cloud_storage::models::Object;
    ///
    /// let client = CloudStorageClient::default();
    /// let obj1 = client.object("my_bucket").read("file1", None).await?;
    /// let obj2 = client.object("my_bucket").rewrite(&obj1, "my_other_bucket", "file2", None).await?;
    /// // obj2 is now a copy of obj1.
    /// # Ok(())
    /// # }
    /// ```
    pub async fn rewrite(
        &self,
        object: &Object,
        destination_bucket: &str,
        path: &str,
        parameters: Option<RewriteParameters>,
    ) -> Result<Object, Error> {
        use reqwest::header::CONTENT_LENGTH;

        let url = format!(
            "{base}/{sObject}/rewriteTo/b/{dBucket}/o/{dObject}",
            base = self.base_url,
            sObject = crate::percent_encode(&object.name),
            dBucket = crate::percent_encode(destination_bucket),
            dObject = crate::percent_encode(path),
        );
        let mut headers = self.client.get_headers().await?;
        headers.insert(CONTENT_LENGTH, "0".parse()?);
        let response = self.client.reqwest
            .post(&url)
            .query(&parameters)
            .headers(headers)
            .send()
            .await?;

        let mut object = response.json::<RewriteResponse>().await?.resource;
        object.private_key = Some(self.client.service_account.private_key.clone());
        object.client_email = Some(self.client.service_account.client_email.clone());
        Ok(object)
    }
}

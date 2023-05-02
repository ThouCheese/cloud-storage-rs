use futures_util::{TryStream, Stream};

use crate::{Object, models::{CreateParameters, ObjectList, ReadParameters, UpdateParameters, DeleteParameters, ComposeRequest, ComposeParameters, CopyParameters, RewriteParameters}, Error, ListRequest};

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
    /// Object::create("cat-photos", file, "recently read cat.png", "image/png", None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(
        bucket: &str,
        file: Vec<u8>,
        filename: &str,
        mime_type: &str,
        parameters: Option<CreateParameters>,
    ) -> Result<Self, Error> {
        crate::CLOUD_CLIENT
            .object()
            .create(bucket, file, filename, mime_type, parameters)
            .await
    }

    /// The synchronous equivalent of `Object::create`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn create_sync(
        bucket: &str,
        file: Vec<u8>,
        filename: &str,
        mime_type: &str,
        parameters: Option<CreateParameters>,
    ) -> Result<Self, Error> {
        crate::runtime()?.block_on(Self::create(bucket, file, filename, mime_type, parameters))
    }

    /// Create a new object with metadata.
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
    /// let metadata = serde_json::json!({
    ///     "metadata": {
    ///         "custom_id": "1234"
    ///     }
    /// });
    /// Object::create("cat-photos", file, "recently read cat.png", "image/png", &metadata).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_with(
        bucket: &str,
        file: Vec<u8>,
        filename: &str,
        mime_type: &str,
        metadata: &serde_json::Value,
    ) -> Result<Self, Error> {
        crate::CLOUD_CLIENT
            .object()
            .create_with(bucket, file, filename, mime_type, metadata)
            .await
    }

    /// Synchronous equivalent of `Object::create_with`
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn create_with_sync(
        bucket: &str,
        file: Vec<u8>,
        filename: &str,
        mime_type: &str,
        metadata: &serde_json::Value,
    ) -> Result<Self, Error> {
        crate::runtime()?.block_on(Self::create_with(
            bucket, file, filename, mime_type, metadata,
        ))
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
    /// Object::create_streamed("cat-photos", file, 10, "recently read cat.png", "image/png", None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_streamed<S>(
        bucket: &str,
        stream: S,
        length: impl Into<Option<u64>>,
        filename: &str,
        mime_type: &str,
        parameters: Option<CreateParameters>,
    ) -> Result<Self, Error>
    where
        S: TryStream + Send + Sync + 'static,
        S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
        bytes::Bytes: From<S::Ok>,
    {
        crate::CLOUD_CLIENT
            .object()
            .create_streamed(bucket, stream, length, filename, mime_type, parameters)
            .await
    }

    /// The synchronous equivalent of `Object::create_streamed`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn create_streamed_sync<R: std::io::Read + Send + 'static>(
        bucket: &str,
        mut file: R,
        length: impl Into<Option<u64>>,
        filename: &str,
        mime_type: &str,
        parameters: Option<CreateParameters>,
    ) -> Result<Self, Error> {
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| crate::Error::Other(e.to_string()))?;

        let stream = futures_util::stream::once(async { Ok::<_, crate::Error>(buffer) });

        crate::runtime()?.block_on(Self::create_streamed(
            bucket, stream, length, filename, mime_type, parameters,
        ))
    }

    /// Obtain a list of objects within this Bucket. This function will repeatedly query Google and
    /// merge the responses into one. Google responds with 1000 Objects at a time, so if you want to
    /// make sure only one http call is performed, make sure to set `list_request.max_results` to
    /// 1000.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::{Object, ListRequest};
    ///
    /// let all_objects = Object::list("my_bucket", ListRequest::default()).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list<'a>(
        bucket: &'a str,
        list_request: ListRequest,
    ) -> Result<impl Stream<Item = Result<ObjectList, Error>> + '_, Error> {
        let object_client : crate::client::ObjectClient<'a> = crate::CLOUD_CLIENT.object();
        object_client.list(bucket.clone(), list_request).await
    }

    /// The synchronous equivalent of `Object::list`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn list_sync(bucket: &str, list_request: ListRequest) -> Result<Vec<ObjectList>, Error> {
        use futures_util::TryStreamExt;

        let rt = crate::runtime()?;
        let listed = rt.block_on(Self::list(bucket, list_request))?;
        rt.block_on(listed.try_collect())
    }

    /// Obtains a single object with the specified name in the specified bucket.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Object;
    ///
    /// let object = Object::read("my_bucket", "path/to/my/file.png", None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn read(
        bucket: &str,
        file_name: &str,
        parameters: Option<ReadParameters>,
    ) -> Result<Self, Error> {
        crate::CLOUD_CLIENT
            .object()
            .read(bucket, file_name, parameters)
            .await
    }

    /// The synchronous equivalent of `Object::read`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn read_sync(
        bucket: &str,
        file_name: &str,
        parameters: Option<ReadParameters>,
    ) -> Result<Self, Error> {
        crate::runtime()?.block_on(Self::read(bucket, file_name, parameters))
    }

    /// Download the content of the object with the specified name in the specified bucket.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Object;
    ///
    /// let bytes = Object::download("my_bucket", "path/to/my/file.png", None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download(
        bucket: &str,
        file_name: &str,
        parameters: Option<ReadParameters>,
    ) -> Result<Vec<u8>, Error> {
        crate::CLOUD_CLIENT
            .object()
            .download(bucket, file_name, parameters)
            .await
    }

    /// The synchronous equivalent of `Object::download`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn download_sync(
        bucket: &str,
        file_name: &str,
        parameters: Option<ReadParameters>,
    ) -> Result<Vec<u8>, Error> {
        crate::runtime()?.block_on(Self::download(bucket, file_name, parameters))
    }

    /// Download the content of the object with the specified name in the specified bucket, without
    /// allocating the whole file into a vector.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Object;
    /// use futures_util::stream::StreamExt;
    /// use std::fs::File;
    /// use std::io::{BufWriter, Write};
    ///
    /// let mut stream = Object::download_streamed("my_bucket", "path/to/my/file.png", None).await?;
    /// let mut file = BufWriter::new(File::create("file.png").unwrap());
    /// while let Some(byte) = stream.next().await {
    ///     file.write_all(&[byte.unwrap()]).unwrap();
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download_streamed<'a>(
        bucket: &str,
        file_name: &str,
        parameters: Option<ReadParameters>,
    ) -> Result<impl Stream<Item = Result<bytes::Bytes, Error>> + Unpin, Error> {
        crate::CLOUD_CLIENT
            .object()
            .download_streamed(bucket, file_name, parameters)
            .await
    }

    /// Obtains a single object with the specified name in the specified bucket.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Object;
    ///
    /// let mut object = Object::read("my_bucket", "path/to/my/file.png", None).await?;
    /// object.content_type = Some("application/xml".to_string());
    /// object.update(None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(&self, parameters: Option<UpdateParameters>) -> Result<Self, Error> {
        crate::CLOUD_CLIENT.object().update(self, parameters).await
    }

    /// The synchronous equivalent of `Object::download`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn update_sync(&self, parameters: Option<UpdateParameters>) -> Result<Self, Error> {
        crate::runtime()?.block_on(self.update(parameters))
    }

    /// Deletes a single object with the specified name in the specified bucket.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Object;
    ///
    /// Object::delete("my_bucket", "path/to/my/file.png", None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(
        bucket: &str,
        file_name: &str,
        parameters: Option<DeleteParameters>,
    ) -> Result<(), Error> {
        crate::CLOUD_CLIENT
            .object()
            .delete(bucket, file_name, parameters)
            .await
    }

    /// The synchronous equivalent of `Object::delete`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn delete_sync(
        bucket: &str,
        file_name: &str,
        parameters: Option<DeleteParameters>,
    ) -> Result<(), Error> {
        crate::runtime()?.block_on(Self::delete(bucket, file_name, parameters))
    }

    /// Obtains a single object with the specified name in the specified bucket.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::object::{Object, ComposeRequest, SourceObject};
    ///
    /// let obj1 = Object::read("my_bucket", "file1", None).await?;
    /// let obj2 = Object::read("my_bucket", "file2", None).await?;
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
    /// let obj3 = Object::compose("my_bucket", &compose_request, "test-concatted-file", None).await?;
    /// // obj3 is now a file with the content of obj1 and obj2 concatted together.
    /// # Ok(())
    /// # }
    /// ```
    pub async fn compose(
        bucket: &str,
        req: &ComposeRequest,
        destination_object: &str,
        parameters: Option<ComposeParameters>,
    ) -> Result<Self, Error> {
        crate::CLOUD_CLIENT
            .object()
            .compose(bucket, req, destination_object, parameters)
            .await
    }

    /// The synchronous equivalent of `Object::compose`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn compose_sync(
        bucket: &str,
        req: &ComposeRequest,
        destination_object: &str,
        parameters: Option<ComposeParameters>,
    ) -> Result<Self, Error> {

        crate::runtime()?.block_on(Self::compose(bucket, req, destination_object, parameters))
    }

    /// Copy this object to the target bucket and path
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::object::{Object, ComposeRequest};
    ///
    /// let obj1 = Object::read("my_bucket", "file1", None).await?;
    /// let obj2 = obj1.copy("my_other_bucket", "file2", None).await?;
    /// // obj2 is now a copy of obj1.
    /// # Ok(())
    /// # }
    /// ```
    pub async fn copy(
        &self,
        destination_bucket: &str,
        path: &str,
        parameters: Option<CopyParameters>,
    ) -> Result<Self, Error> {
        crate::CLOUD_CLIENT
            .object()
            .copy(self, destination_bucket, path, parameters)
            .await
    }

    /// The synchronous equivalent of `Object::copy`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn copy_sync(
        &self,
        destination_bucket: &str,
        path: &str,
        parameters: Option<CopyParameters>,
    ) -> Result<Self, Error> {
        crate::runtime()?.block_on(self.copy(destination_bucket, path, parameters))
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
    /// let obj1 = Object::read("my_bucket", "file1", None).await?;
    /// let obj2 = obj1.rewrite("my_other_bucket", "file2", None).await?;
    /// // obj2 is now a copy of obj1.
    /// # Ok(())
    /// # }
    /// ```
    pub async fn rewrite(
        &self,
        destination_bucket: &str,
        path: &str,
        parameters: Option<RewriteParameters>,
    ) -> Result<Self, Error> {
        crate::CLOUD_CLIENT
            .object()
            .rewrite(self, destination_bucket, path, parameters)
            .await
    }

    /// The synchronous equivalent of `Object::rewrite`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn rewrite_sync(
        &self,
        destination_bucket: &str,
        path: &str,
        parameters: Option<RewriteParameters>,
    ) -> Result<Self, Error> {
        crate::runtime()?.block_on(self.rewrite(destination_bucket, path, parameters))
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, io::Write};

    use super::*;
    use crate::{Error, models::{ComposeRequest, SourceObject}};
    use bytes::Buf;
    use futures_util::{stream, StreamExt, TryStreamExt};

    #[tokio::test]
    async fn create() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::global_client::read_test_bucket().await;
        Object::create(&bucket.name, vec![0, 1], "test-create", "text/plain", None).await?;
        Ok(())
    }

    #[tokio::test]
    async fn create_with() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::global_client::read_test_bucket().await;
        let metadata = serde_json::json!({
            "metadata": {
                "object_id": "1234"
            }
        });
        let obj = Object::create_with(
            &bucket.name,
            vec![0, 1],
            "test-create-meta",
            "text/plain",
            &metadata,
        )
        .await?;
        assert_eq!(
            obj.metadata.unwrap().get("object_id"),
            Some(&String::from("1234"))
        );
        Ok(())
    }

    #[tokio::test]
    async fn create_streamed() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::global_client::read_test_bucket().await;
        let stream = stream::iter([0u8, 1].iter())
            .map(Ok::<_, Box<dyn std::error::Error + Send + Sync>>)
            .map_ok(|&b| bytes::BytesMut::from(&[b][..]));
        Object::create_streamed(
            &bucket.name,
            stream,
            2,
            "test-create-streamed",
            "text/plain",
            None,
        )
        .await?;
        Ok(())
    }

    #[tokio::test]
    async fn list() -> Result<(), Box<dyn std::error::Error>> {
        let test_bucket = crate::global_client::read_test_bucket().await;
        let _v: Vec<ObjectList> = Object::list(&test_bucket.name, ListRequest::default())
            .await?
            .try_collect()
            .await?;
        Ok(())
    }

    async fn flattened_list_prefix_stream(
        bucket: &str,
        prefix: &str,
    ) -> Result<Vec<Object>, Box<dyn std::error::Error>> {
        let request = ListRequest {
            prefix: Some(prefix.into()),
            ..Default::default()
        };

        Ok(Object::list(bucket, request)
            .await?
            .map_ok(|object_list| object_list.items)
            .try_concat()
            .await?)
    }

    #[tokio::test]
    async fn list_prefix() -> Result<(), Box<dyn std::error::Error>> {
        let test_bucket = crate::global_client::read_test_bucket().await;

        let prefix_names = [
            "test-list-prefix/1",
            "test-list-prefix/2",
            "test-list-prefix/sub/1",
            "test-list-prefix/sub/2",
        ];

        for name in &prefix_names {
            Object::create(&test_bucket.name, vec![0, 1], name, "text/plain", None).await?;
        }

        let list = flattened_list_prefix_stream(&test_bucket.name, "test-list-prefix/").await?;
        assert_eq!(list.len(), 4);
        let list = flattened_list_prefix_stream(&test_bucket.name, "test-list-prefix/sub").await?;
        assert_eq!(list.len(), 2);
        Ok(())
    }

    #[tokio::test]
    async fn read() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::global_client::read_test_bucket().await;
        Object::create(&bucket.name, vec![0, 1], "test-read", "text/plain", None).await?;
        Object::read(&bucket.name, "test-read", None).await?;
        Ok(())
    }

    #[tokio::test]
    async fn download() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::global_client::read_test_bucket().await;
        let content = b"hello world";
        Object::create(
            &bucket.name,
            content.to_vec(),
            "test-download",
            "application/octet-stream",
            None,
        )
        .await?;

        let data = Object::download(&bucket.name, "test-download", None).await?;
        assert_eq!(data, content);

        Ok(())
    }

    #[tokio::test]
    async fn download_streamed() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::global_client::read_test_bucket().await;
        let content = b"hello world";
        Object::create(
            &bucket.name,
            content.to_vec(),
            "test-download",
            "application/octet-stream",
            None,
        )
        .await?;

        let mut result = Object::download_streamed(&bucket.name, "test-download", None).await?;
        let mut data: Vec<u8> = Vec::new();
        while let Some(part) = result.next().await {
            data.write_all(part?.chunk());
        }
        assert_eq!(data, content);

        Ok(())
    }

    #[tokio::test]
    async fn download_streamed_large() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::global_client::read_test_bucket().await;
        let content = vec![5u8; 1_000_000];
        Object::create(
            &bucket.name,
            content.to_vec(),
            "test-download-large",
            "application/octet-stream",
            None,
        )
        .await?;

        let mut result =
            Object::download_streamed(&bucket.name, "test-download-large", None).await?;
        let mut data: Vec<u8> = Vec::new();
        while let Some(part) = result.next().await {
            data.write_all(part?.chunk());
        }
        assert_eq!(data, content);

        Ok(())
    }

    #[tokio::test]
    async fn update() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::global_client::read_test_bucket().await;
        let mut obj =
            Object::create(&bucket.name, vec![0, 1], "test-update", "text/plain", None).await?;
        obj.content_type = Some("application/xml".to_string());
        obj.update(None).await?;
        Ok(())
    }

    #[tokio::test]
    async fn delete() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::global_client::read_test_bucket().await;
        Object::create(&bucket.name, vec![0, 1], "test-delete", "text/plain", None).await?;

        Object::delete(&bucket.name, "test-delete", None).await?;

        let list: Vec<_> = flattened_list_prefix_stream(&bucket.name, "test-delete").await?;
        assert!(list.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn delete_nonexistent() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::global_client::read_test_bucket().await;

        let nonexistent_object = "test-delete-nonexistent";

        let delete_result = Object::delete(&bucket.name, nonexistent_object, None).await;

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
        let bucket = crate::global_client::read_test_bucket().await;
        let obj1 = Object::create(
            &bucket.name,
            vec![0, 1],
            "test-compose-1",
            "text/plain",
            None,
        )
        .await?;
        let obj2 = Object::create(
            &bucket.name,
            vec![2, 3],
            "test-compose-2",
            "text/plain",
            None,
        )
        .await?;
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
        let obj3 =
            Object::compose(&bucket.name, &compose_request, "test-concatted-file", None).await?;
        let url = obj3.download_url(100)?;
        let content = reqwest::get(&url).await?.text().await?;
        assert_eq!(content.as_bytes(), &[0, 1, 2, 3]);
        Ok(())
    }

    #[tokio::test]
    async fn copy() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::global_client::read_test_bucket().await;
        let original =
            Object::create(&bucket.name, vec![2, 3], "test-copy", "text/plain", None).await?;
        original
            .copy(&bucket.name, "test-copy - copy", None)
            .await?;
        Ok(())
    }

    #[tokio::test]
    async fn rewrite() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::global_client::read_test_bucket().await;
        let obj =
            Object::create(&bucket.name, vec![0, 1], "test-rewrite", "text/plain", None).await?;
        let obj = obj.rewrite(&bucket.name, "test-rewritten", None).await?;
        let url = obj.download_url(100)?;
        let client = reqwest::Client::default();
        let download = client.head(&url).send().await?;
        assert_eq!(download.status().as_u16(), 200);
        Ok(())
    }

    #[tokio::test]
    async fn test_url_encoding() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::global_client::read_test_bucket().await;
        let complicated_names = [
            "asdf",
            "asdf+1",
            "asdf&&+1?=3,,-_()*&^%$#@!`~{}[]\\|:;\"'<>,.?/äöüëß",
            "https://www.google.com",
            "परिक्षण फाईल",
            "测试很重要",
        ];
        for name in &complicated_names {
            let _obj = Object::create(&bucket.name, vec![0, 1], name, "text/plain", None).await?;
            let obj = Object::read(&bucket.name, &name, None).await.unwrap();
            let url = obj.download_url(100)?;
            let client = reqwest::Client::default();
            let download = client.head(&url).send().await?;
            assert_eq!(download.status().as_u16(), 200);
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_download_url_with() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::global_client::read_test_bucket().await;
        let client = reqwest::Client::new();
        let obj =
            Object::create(&bucket.name, vec![0, 1], "test-rewrite", "text/plain", None).await?;

        let opts1 = crate::DownloadOptions::new().content_disposition("attachment");
        let download_url1 = obj.download_url_with(100, opts1)?;
        let download1 = client.head(&download_url1).send().await?;
        assert_eq!(download1.headers()["content-disposition"], "attachment");
        Ok(())
    }

    #[tokio::test]
    async fn test_upload_url() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::global_client::read_test_bucket().await;
        let client = reqwest::Client::new();
        let blob_name = "test-upload-url";
        let obj = Object::create(&bucket.name, vec![0, 1], blob_name, "text/plain", None).await?;

        let url = obj.upload_url(100).unwrap();
        let updated_content = vec![2, 3];
        let response = client
            .put(&url)
            .body(updated_content.clone())
            .send()
            .await?;
        assert!(response.status().is_success());
        let data = Object::download(&bucket.name, blob_name, None).await?;
        assert_eq!(data, updated_content);
        Ok(())
    }

    #[tokio::test]
    async fn test_upload_url_with() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::global_client::read_test_bucket().await;
        let client = reqwest::Client::new();
        let blob_name = "test-upload-url";
        let obj = Object::create(&bucket.name, vec![0, 1], blob_name, "text/plain", None).await?;
        let mut custom_metadata = HashMap::new();
        custom_metadata.insert(String::from("field"), String::from("value"));

        let (url, headers) = obj.upload_url_with(100, custom_metadata).unwrap();
        let updated_content = vec![2, 3];
        let mut request = client.put(&url).body(updated_content);
        for (metadata_field, metadata_value) in headers.iter() {
            request = request.header(metadata_field, metadata_value);
        }
        let response = request.send().await?;
        assert!(response.status().is_success());
        let updated_obj = Object::read(&bucket.name, blob_name, None).await?;
        let obj_metadata = updated_obj.metadata.unwrap();
        assert_eq!(obj_metadata.get("field").unwrap(), "value");
        Ok(())
    }

    #[cfg(feature = "sync")]
    mod sync {
        use super::*;

        #[test]
        fn create() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::global_client::read_test_bucket_sync();
            Object::create_sync(&bucket.name, vec![0, 1], "test-create", "text/plain", None)?;
            Ok(())
        }

        #[test]
        fn create_with() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::global_client::read_test_bucket_sync();
            let metadata = serde_json::json!({
                "metadata": {
                    "object_id": "1234"
                }
            });
            let obj = Object::create_with_sync(
                &bucket.name,
                vec![0, 1],
                "test-create-meta",
                "text/plain",
                &metadata,
            )?;
            assert_eq!(
                obj.metadata.unwrap().get("object_id"),
                Some(&String::from("1234"))
            );
            Ok(())
        }

        #[test]
        fn create_streamed() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::global_client::read_test_bucket_sync();
            let cursor = std::io::Cursor::new([0, 1]);
            Object::create_streamed_sync(
                &bucket.name,
                cursor,
                2,
                "test-create-streamed",
                "text/plain",
                None,
            )?;
            Ok(())
        }

        #[test]
        fn list() -> Result<(), Box<dyn std::error::Error>> {
            let test_bucket = crate::global_client::read_test_bucket_sync();
            Object::list_sync(&test_bucket.name, ListRequest::default())?;
            Ok(())
        }

        #[test]
        fn list_prefix() -> Result<(), Box<dyn std::error::Error>> {
            let test_bucket = crate::global_client::read_test_bucket_sync();

            let prefix_names = [
                "test-list-prefix/1",
                "test-list-prefix/2",
                "test-list-prefix/sub/1",
                "test-list-prefix/sub/2",
            ];

            for name in &prefix_names {
                Object::create_sync(&test_bucket.name, vec![0, 1], name, "text/plain", None)?;
            }

            let request = ListRequest {
                prefix: Some("test-list-prefix/".into()),
                ..Default::default()
            };
            let list = Object::list_sync(&test_bucket.name, request)?;
            assert_eq!(list[0].items.len(), 4);

            let request = ListRequest {
                prefix: Some("test-list-prefix/sub".into()),
                ..Default::default()
            };
            let list = Object::list_sync(&test_bucket.name, request)?;
            assert_eq!(list[0].items.len(), 2);
            Ok(())
        }

        #[test]
        fn list_prefix_delimiter() -> Result<(), Box<dyn std::error::Error>> {
            let test_bucket = crate::global_client::read_test_bucket_sync();

            let prefix_names = [
                "test-list-prefix/1",
                "test-list-prefix/2",
                "test-list-prefix/sub/1",
                "test-list-prefix/sub/2",
            ];

            for name in &prefix_names {
                Object::create_sync(&test_bucket.name, vec![0, 1], name, "text/plain", None)?;
            }

            let request = ListRequest {
                prefix: Some("test-list-prefix/".into()),
                delimiter: Some("/".into()),
                ..Default::default()
            };
            let list = Object::list_sync(&test_bucket.name, request)?;
            assert_eq!(list[0].items.len(), 2);
            assert_eq!(list[0].prefixes.len(), 1);
            Ok(())
        }

        #[test]
        fn read() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::global_client::read_test_bucket_sync();
            Object::create_sync(&bucket.name, vec![0, 1], "test-read", "text/plain", None)?;
            Object::read_sync(&bucket.name, "test-read", None)?;
            Ok(())
        }

        #[test]
        fn download() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::global_client::read_test_bucket_sync();
            let content = b"hello world";
            Object::create_sync(
                &bucket.name,
                content.to_vec(),
                "test-download",
                "application/octet-stream",
                None,
            )?;

            let data = Object::download_sync(&bucket.name, "test-download", None)?;
            assert_eq!(data, content);

            Ok(())
        }

        #[test]
        fn update() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::global_client::read_test_bucket_sync();
            let mut obj =
                Object::create_sync(&bucket.name, vec![0, 1], "test-update", "text/plain", None)?;
            obj.content_type = Some("application/xml".to_string());
            obj.update_sync(None)?;
            Ok(())
        }

        #[test]
        fn delete() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::global_client::read_test_bucket_sync();
            Object::create_sync(&bucket.name, vec![0, 1], "test-delete", "text/plain", None)?;

            Object::delete_sync(&bucket.name, "test-delete", None)?;

            let request = ListRequest {
                prefix: Some("test-delete".into()),
                ..Default::default()
            };

            let list = Object::list_sync(&bucket.name, request)?;
            assert!(list[0].items.is_empty());

            Ok(())
        }

        #[test]
        fn delete_nonexistent() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::global_client::read_test_bucket_sync();

            let nonexistent_object = "test-delete-nonexistent";

            let delete_result = Object::delete_sync(&bucket.name, nonexistent_object, None);

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
            let bucket = crate::global_client::read_test_bucket_sync();
            let obj1 = Object::create_sync(
                &bucket.name,
                vec![0, 1],
                "test-compose-1",
                "text/plain",
                None,
            )?;
            let obj2 = Object::create_sync(
                &bucket.name,
                vec![2, 3],
                "test-compose-2",
                "text/plain",
                None,
            )?;
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
            let obj3 =
                Object::compose_sync(&bucket.name, &compose_request, "test-concatted-file", None)?;
            let url = obj3.download_url(100)?;
            let content = reqwest::blocking::get(&url)?.text()?;
            assert_eq!(content.as_bytes(), &[0, 1, 2, 3]);
            Ok(())
        }

        #[test]
        fn copy() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::global_client::read_test_bucket_sync();
            let original =
                Object::create_sync(&bucket.name, vec![2, 3], "test-copy", "text/plain", None)?;
            original.copy_sync(&bucket.name, "test-copy - copy", None)?;
            Ok(())
        }

        #[test]
        fn rewrite() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::global_client::read_test_bucket_sync();
            let obj =
                Object::create_sync(&bucket.name, vec![0, 1], "test-rewrite", "text/plain", None)?;
            let obj = obj.rewrite_sync(&bucket.name, "test-rewritten", None)?;
            let url = obj.download_url(100)?;
            let client = reqwest::blocking::Client::new();
            let download = client.head(&url).send()?;
            assert_eq!(download.status().as_u16(), 200);
            Ok(())
        }

        #[test]
        fn test_url_encoding() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::global_client::read_test_bucket_sync();
            let complicated_names = [
                "asdf",
                "asdf+1",
                "asdf&&+1?=3,,-_()*&^%$#@!`~{}[]\\|:;\"'<>,.?/äöüëß",
                "https://www.google.com",
                "परिक्षण फाईल",
                "测试很重要",
            ];
            for name in &complicated_names {
                let _obj = Object::create_sync(&bucket.name, vec![0, 1], name, "text/plain", None)?;
                let obj = Object::read_sync(&bucket.name, &name, None).unwrap();
                let url = obj.download_url(100)?;
                let client = reqwest::blocking::Client::new();
                let download = client.head(&url).send()?;
                assert_eq!(download.status().as_u16(), 200);
            }
            Ok(())
        }
    }
}
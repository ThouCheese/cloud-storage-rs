use bytes::Buf;
use futures_util::{io::AllowStdIo, StreamExt, TryStreamExt};
use tokio::io::AsyncWriteExt;
use tokio_util::compat::FuturesAsyncWriteCompatExt;

use crate::{models::{CreateParameters, ObjectList, ReadParameters, UpdateParameters, DeleteParameters, ComposeRequest, ComposeParameters, CopyParameters, RewriteParameters}, Object, Error, ListRequest};

/// Operations on [`Object`](Object)s.
#[derive(Debug)]
pub struct ObjectClient<'a> {
    pub(crate) client: crate::client::ObjectClient<'a>,
    pub(crate) runtime: &'a tokio::runtime::Handle,
}

impl<'a> ObjectClient<'a> {
    /// Create a new object.
    /// Upload a file as that is loaded in memory to google cloud storage, where it will be
    /// interpreted according to the mime type you specified.
    /// ## Example
    /// ```rust,no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # fn read_cute_cat(_in: &str) -> Vec<u8> { vec![0, 1] }
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::Object;
    ///
    /// let file: Vec<u8> = read_cute_cat("cat.png");
    /// let client = Client::new()?;
    /// client.object("cat-photos").create(file, "recently read cat.png", "image/png", None)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create(
        &self,
        file: Vec<u8>,
        filename: &str,
        mime_type: &str,
        parameters: Option<CreateParameters>,
    ) -> Result<Object, Error> {
        self.runtime.block_on(
            self.client
                .create(file, filename, mime_type, parameters),
        )
    }

    /// Create a new object.
    /// Upload a file as that is loaded in memory to google cloud storage, where it will be
    /// interpreted according to the mime type you specified.
    /// ## Example
    /// ```rust,no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # fn read_cute_cat(_in: &str) -> Vec<u8> { vec![0, 1] }
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::Object;
    ///
    /// let file: Vec<u8> = read_cute_cat("cat.png");
    /// let client = Client::new()?;
    /// let metadata = serde_json::json!({
    ///     "metadata": {
    ///         "custom_id": "1234"
    ///     }
    /// });
    /// client.object("cat-photos").create_with(file, "recently read cat.png", "image/png", &metadata)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_with(
        &self,
        file: Vec<u8>,
        filename: &str,
        mime_type: &str,
        metadata: &serde_json::Value,
    ) -> Result<Object, Error> {
        self.runtime.block_on(
            self.client
                .create_with(file, filename, mime_type, metadata),
        )
    }

    /// Create a new object. This works in the same way as `ObjectClient::create`, except it does not need
    /// to load the entire file in ram.
    pub fn create_streamed<R>(
        &self,
        file: R,
        length: impl Into<Option<u64>>,
        filename: &str,
        mime_type: &str,
        parameters: Option<CreateParameters>,
    ) -> Result<Object, Error>
    where
        R: std::io::Read + Send + Sync + Unpin + 'static,
    {
        let stream = super::helpers::ReaderStream::new(file);

        self.runtime.block_on(
            self.client
                .create_streamed(stream, length, filename, mime_type, parameters),
        )
    }

    /// Create a new object with metadata. This works in the same way as `ObjectClient::create`, except it does not need
    /// to load the entire file in ram.
    pub fn create_streamed_with<R>(
        &self,
        file: R,
        filename: &str,
        mime_type: &str,
        metadata: &serde_json::Value,
    ) -> Result<Object, Error>
    where
        R: std::io::Read + Send + Sync + Unpin + 'static,
    {
        let stream = super::helpers::ReaderStream::new(file);

        self.runtime.block_on(
            self.client
                .create_streamed_with(stream, filename, mime_type, metadata),
        )
    }

    /// Obtain a list of objects within this Bucket.
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::{Object, ListRequest};
    ///
    /// let client = Client::new()?;
    /// let all_objects = client.object("my_bucket").list(ListRequest::default())?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(
        &self,
        list_request: ListRequest,
    ) -> Result<Vec<ObjectList>, Error> {
        let rt = &self.runtime;
        let listed = rt.block_on(self.client.list(list_request))?;
        rt.block_on(listed.try_collect())
    }

    /// Obtains a single object with the specified name in the specified bucket.
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::Object;
    ///
    /// let client = Client::new()?;
    /// let object = client.object("my_bucket").read("path/to/my/file.png", None)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read(
        &self,
        file_name: &str,
        parameters: Option<ReadParameters>,
    ) -> Result<Object, Error> {
        self.runtime
            .block_on(self.client.read(file_name, parameters))
    }

    /// Download the content of the object with the specified name in the specified bucket.
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::Object;
    ///
    /// let client = Client::new()?;
    /// let bytes = client.object("my_bucket").download("path/to/my/file.png", None)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn download(
        &self,
        file_name: &str,
        parameters: Option<ReadParameters>,
    ) -> Result<Vec<u8>, Error> {
        self.runtime.block_on(
            self.client
                .download(file_name, parameters),
        )
    }

    /// Download the content of the object with the specified name in the specified bucket.
    /// This works in the same way as `ObjectClient::download_streamed`, except it does not
    /// need to load the entire result in ram.
    ///
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::Object;
    ///
    /// let client = Client::new()?;
    /// let file = File::create("somefile")?;
    /// let bytes = client.object("my_bucket").download("path/to/my/file.png", file)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn download_streamed<W>(&self, file_name: &str, file: W) -> Result<(), Error>
    where
        W: std::io::Write, // + Send + Sync + Unpin + 'static,
    {
        self.runtime.block_on(async {
            let mut stream = self.client
                .download_streamed(file_name, None)
                .await?;

            let mut writer = tokio::io::BufWriter::new(AllowStdIo::new(file).compat_write());
            while let Some(byte) = stream.next().await {
                writer.write_all(byte?.chunk()).await?;
            }
            writer.flush().await?;
            Ok(())
        })
    }

    /// Obtains a single object with the specified name in the specified bucket.
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::Object;
    ///
    /// let client = Client::new()?;
    /// let mut object = client.object("my_bucket").read("path/to/my/file.png", None)?;
    /// object.content_type = Some("application/xml".to_string());
    /// client.object().update(&object, None)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn update(
        &self,
        object: &Object,
        parameters: Option<UpdateParameters>,
    ) -> Result<Object, Error> {
        self.runtime
            .block_on(self.client.update(object, parameters))
    }

    /// Deletes a single object with the specified name in the specified bucket.
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::Object;
    ///
    /// let client = Client::new()?;
    /// client.object("my_bucket").delete("path/to/my/file.png", None)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete(
        &self,
        file_name: &str,
        parameters: Option<DeleteParameters>,
    ) -> Result<(), Error> {
        self.runtime
            .block_on(self.client.delete(file_name, parameters))
    }

    /// Obtains a single object with the specified name in the specified bucket.
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::object::{Object, ComposeRequest, SourceObject};
    ///
    /// let client = Client::new()?;
    /// let obj1 = client.object("my_bucket").read("file1", None)?;
    /// let obj2 = client.object("my_bucket").read("file2", None)?;
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
    /// let obj3 = client.object("my_bucket").compose(&compose_request, "test-concatted-file", None)?;
    /// // obj3 is now a file with the content of obj1 and obj2 concatted together.
    /// # Ok(())
    /// # }
    /// ```
    pub fn compose(
        &self,
        req: &ComposeRequest,
        destination_object: &str,
        parameters: Option<ComposeParameters>,
    ) -> Result<Object, Error> {
        self.runtime.block_on(self.client.compose(
            req,
            destination_object,
            parameters,
        ))
    }

    /// Copy this object to the target bucket and path
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::object::{Object, ComposeRequest};
    ///
    /// let client = Client::new()?;
    /// let obj1 = client.object("my_bucket").read("file1", None)?;
    /// let obj2 = client.object().copy(&obj1, "my_other_bucket", "file2", None)?;
    /// // obj2 is now a copy of obj1.
    /// # Ok(())
    /// # }
    /// ```
    pub fn copy(
        &self,
        object: &Object,
        destination_bucket: &str,
        path: &str,
        parameters: Option<CopyParameters>,
    ) -> Result<Object, Error> {
        self.runtime.block_on(self.client.copy(
            object,
            destination_bucket,
            path,
            parameters,
        ))
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
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::object::Object;
    ///
    /// let client = Client::new()?;
    /// let obj1 = client.object("my_bucket").read("file1", None)?;
    /// let obj2 = client.object().rewrite(&obj1, "my_other_bucket", "file2", None)?;
    /// // obj2 is now a copy of obj1.
    /// # Ok(())
    /// # }
    /// ```
    pub fn rewrite(
        &self,
        object: &Object,
        destination_bucket: &str,
        path: &str,
        parameters: Option<RewriteParameters>,
    ) -> Result<Object, Error> {
        self.runtime.block_on(self.client.rewrite(
            object,
            destination_bucket,
            path,
            parameters,
        ))
    }
}

use crate::{
    object::{ComposeRequest, ObjectList},
    ListRequest, Object,
};
use futures_util::TryStreamExt;

/// Operations on [`Object`](Object)s.
#[derive(Debug)]
pub struct ObjectClient<'a>(pub(super) &'a super::Client);

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
    /// client.object().create("cat-photos", file, "recently read cat.png", "image/png")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create(
        &self,
        bucket: &str,
        file: Vec<u8>,
        filename: &str,
        mime_type: &str,
    ) -> crate::Result<Object> {
        self.0.runtime.block_on(
            self.0
                .client
                .object()
                .create(bucket, file, filename, mime_type),
        )
    }

    /// Create a new object. This works in the same way as `ObjectClient::create`, except it does not need
    /// to load the entire file in ram.
    pub fn create_streamed<R>(
        &self,
        bucket: &str,
        file: R,
        length: impl Into<Option<u64>>,
        filename: &str,
        mime_type: &str,
    ) -> crate::Result<Object>
    where
        R: std::io::Read + Send + Sync + Unpin + 'static,
    {
        let stream = super::helpers::ReaderStream::new(file);

        self.0.runtime.block_on(
            self.0
                .client
                .object()
                .create_streamed(bucket, stream, length, filename, mime_type),
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
    /// let all_objects = client.object().list("my_bucket", ListRequest::default())?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(
        &self,
        bucket: &'a str,
        list_request: ListRequest,
    ) -> crate::Result<Vec<ObjectList>> {
        let rt = &self.0.runtime;
        let listed = rt.block_on(self.0.client.object().list(bucket, list_request))?;
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
    /// let object = client.object().read("my_bucket", "path/to/my/file.png")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read(&self, bucket: &str, file_name: &str) -> crate::Result<Object> {
        self.0
            .runtime
            .block_on(self.0.client.object().read(bucket, file_name))
    }

    /// Download the content of the object with the specified name in the specified bucket.
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::Object;
    ///
    /// let client = Client::new()?;
    /// let bytes = client.object().download("my_bucket", "path/to/my/file.png")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn download(&self, bucket: &str, file_name: &str) -> crate::Result<Vec<u8>> {
        self.0
            .runtime
            .block_on(self.0.client.object().download(bucket, file_name))
    }

    /// Obtains a single object with the specified name in the specified bucket.
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::Object;
    ///
    /// let client = Client::new()?;
    /// let mut object = client.object().read("my_bucket", "path/to/my/file.png")?;
    /// object.content_type = Some("application/xml".to_string());
    /// client.object().update(&object)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn update(&self, object: &Object) -> crate::Result<Object> {
        self.0
            .runtime
            .block_on(self.0.client.object().update(object))
    }

    /// Deletes a single object with the specified name in the specified bucket.
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::Object;
    ///
    /// let client = Client::new()?;
    /// client.object().delete("my_bucket", "path/to/my/file.png")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete(&self, bucket: &str, file_name: &str) -> crate::Result<()> {
        self.0
            .runtime
            .block_on(self.0.client.object().delete(bucket, file_name))
    }

    /// Obtains a single object with the specified name in the specified bucket.
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::object::{Object, ComposeRequest, SourceObject};
    ///
    /// let client = Client::new()?;
    /// let obj1 = client.object().read("my_bucket", "file1")?;
    /// let obj2 = client.object().read("my_bucket", "file2")?;
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
    /// let obj3 = client.object().compose("my_bucket", &compose_request, "test-concatted-file")?;
    /// // obj3 is now a file with the content of obj1 and obj2 concatted together.
    /// # Ok(())
    /// # }
    /// ```
    pub fn compose(
        &self,
        bucket: &str,
        req: &ComposeRequest,
        destination_object: &str,
    ) -> crate::Result<Object> {
        self.0.runtime.block_on(
            self.0
                .client
                .object()
                .compose(bucket, req, destination_object),
        )
    }

    /// Copy this object to the target bucket and path
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::object::{Object, ComposeRequest};
    ///
    /// let client = Client::new()?;
    /// let obj1 = client.object().read("my_bucket", "file1")?;
    /// let obj2 = client.object().copy(&obj1, "my_other_bucket", "file2")?;
    /// // obj2 is now a copy of obj1.
    /// # Ok(())
    /// # }
    /// ```
    pub fn copy(
        &self,
        object: &Object,
        destination_bucket: &str,
        path: &str,
    ) -> crate::Result<Object> {
        self.0.runtime.block_on(
            self.0
                .client
                .object()
                .copy(object, destination_bucket, path),
        )
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
    /// let obj1 = client.object().read("my_bucket", "file1")?;
    /// let obj2 = client.object().rewrite(&obj1, "my_other_bucket", "file2")?;
    /// // obj2 is now a copy of obj1.
    /// # Ok(())
    /// # }
    /// ```
    pub fn rewrite(
        &self,
        object: &Object,
        destination_bucket: &str,
        path: &str,
    ) -> crate::Result<Object> {
        self.0.runtime.block_on(
            self.0
                .client
                .object()
                .rewrite(object, destination_bucket, path),
        )
    }
}

use crate::{models::{create, IamPolicy, TestIamPermission}, Bucket, Error};

/// Operations on [`Bucket`]()s.
#[derive(Debug)]
pub struct BucketClient<'a> {
    pub(crate) client: crate::client::BucketClient<'a>,
    pub(crate) runtime: &'a tokio::runtime::Handle,
}

impl<'a> BucketClient<'a> {
    /// Creates a new `Bucket`. There are many options that you can provide for creating a new
    /// bucket, so the `create::Bucket` resource contains all of them. Note that `create::Bucket` implements
    /// `Default`, so you don't have to specify the fields you're not using. And error is returned
    /// if that bucket name is already taken.
    /// ### Example
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::sync::CloudStorageClient;
    /// # use cloud_storage::models::{Bucket, create};
    /// # use cloud_storage::models::{Location, MultiRegion};
    ///
    /// let client = CloudStorageClient::new()?;
    /// let new_bucket = create::Bucket {
    ///    name: "cloud-storage-rs-doc-1".to_string(), // this is the only mandatory field
    ///    location: Location::Multi(MultiRegion::Eu),
    ///    ..Default::default()
    /// };
    /// let bucket = client.bucket().create(&new_bucket)?;
    /// # client.bucket().delete(bucket)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create(&self, new_bucket: &create::Bucket) -> Result<Bucket, Error> {
        self.runtime
            .block_on(self.client.create(new_bucket))
    }

    /// Returns all `Bucket`s within this project.
    ///
    /// ### Note
    /// When using incorrect permissions, this function fails silently and returns an empty list.
    ///
    /// ### Example
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::sync::CloudStorageClient;
    /// # use cloud_storage::Bucket;
    ///
    /// let client = CloudStorageClient::new()?;
    /// let buckets = client.bucket().list()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> Result<Vec<Bucket>, Error> {
        self.runtime.block_on(self.client.list())
    }

    /// Returns a single `Bucket` by its name. If the Bucket does not exist, an error is returned.
    /// ### Example
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::sync::CloudStorageClient;
    /// # use cloud_storage::Bucket;
    ///
    /// let client = CloudStorageClient::new()?;
    /// # use cloud_storage::models::create;
    /// # let new_bucket = create::Bucket {
    /// #   name: "cloud-storage-rs-doc-2".to_string(),
    /// #    ..Default::default()
    /// # };
    /// # let _ = client.bucket().create(&new_bucket)?;
    ///
    /// let bucket = client.bucket().read("cloud-storage-rs-doc-2")?;
    /// # client.bucket().delete(bucket)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read(&self, name: &str) -> Result<Bucket, Error> {
        self.runtime.block_on(self.client.read(name))
    }

    /// Update an existing `Bucket`. If you declare you bucket as mutable, you can edit its fields.
    /// You can then flush your changes to Google Cloud Storage using this method.
    /// ### Example
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::sync::CloudStorageClient;
    /// # use cloud_storage::models::{Bucket, RetentionPolicy};
    ///
    /// let client = CloudStorageClient::new()?;
    /// # use cloud_storage::models::create;
    /// # let new_bucket = create::Bucket {
    /// #   name: "cloud-storage-rs-doc-3".to_string(),
    /// #    ..Default::default()
    /// # };
    /// # let _ = client.bucket().create(&new_bucket)?;
    ///
    /// let mut bucket = client.bucket().read("cloud-storage-rs-doc-3")?;
    /// bucket.retention_policy = Some(RetentionPolicy {
    ///     retention_period: 50,
    ///     effective_time: time::OffsetDateTime::now_utc() + std::time::Duration::from_secs(50),
    ///     is_locked: Some(false),
    /// });
    /// client.bucket().update(&bucket)?;
    /// # client.bucket().delete(bucket)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn update(&self, bucket: &Bucket) -> Result<Bucket, Error> {
        self.runtime
            .block_on(self.client.update(bucket))
    }

    /// Delete an existing `Bucket`. This permanently removes a bucket from Google Cloud Storage.
    /// An error is returned when you don't have sufficient permissions, or when the
    /// `retention_policy` prevents you from deleting your Bucket.
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::sync::CloudStorageClient;
    /// # use cloud_storage::Bucket;
    /// #
    /// let client = CloudStorageClient::new()?;
    /// # use cloud_storage::models::create;
    /// # let new_bucket = create::Bucket {
    /// #   name: "unnecessary-bucket".to_string(),
    /// #    ..Default::default()
    /// # };
    /// # let _ = client.bucket().create(&new_bucket)?;
    ///
    /// let bucket = client.bucket().read("unnecessary-bucket")?;
    /// client.bucket().delete(bucket)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete(&self, bucket: Bucket) -> Result<(), Error> {
        self.runtime
            .block_on(self.client.delete(bucket))
    }

    /// Returns the [IAM Policy](https://cloud.google.com/iam/docs/) for this bucket.
    /// ### Example
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::sync::CloudStorageClient;
    /// # use cloud_storage::Bucket;
    /// # use cloud_storage::models::create;
    /// #
    /// let cloud_storage_client = CloudStorageClient::new()?;
    /// let client = cloud_storage_client.bucket();
    /// # let new_bucket = create::Bucket {
    /// #   name: "cloud-storage-rs-doc-4".to_string(),
    /// #    ..Default::default()
    /// # };
    /// # let _ = client.create(&new_bucket)?;
    ///
    /// let bucket = client.read("cloud-storage-rs-doc-4")?;
    /// let policy = client.get_iam_policy(&bucket)?;
    /// # client.delete(bucket)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_iam_policy(&self, bucket: &Bucket) -> Result<IamPolicy, Error> {
        self.runtime
            .block_on(self.client.get_iam_policy(bucket))
    }

    /// Updates the [IAM Policy](https://cloud.google.com/iam/docs/) for this bucket.
    /// ### Example
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::sync::CloudStorageClient;
    /// # use cloud_storage::Bucket;
    /// # use cloud_storage::models::{IamPolicy, Binding, IamRole, StandardIamRole, Entity};
    ///
    /// let client = CloudStorageClient::new()?;
    /// # use cloud_storage::models::create;
    /// # let new_bucket = create::Bucket {
    /// #   name: "cloud-storage-rs-doc-5".to_string(),
    /// #    ..Default::default()
    /// # };
    /// # let _ = client.bucket().create(&new_bucket)?;
    ///
    /// let bucket = client.bucket().read("cloud-storage-rs-doc-5")?;
    /// let iam_policy = IamPolicy {
    ///     version: 1,
    ///     bindings: vec![
    ///         Binding {
    ///             role: IamRole::Standard(StandardIamRole::ObjectViewer),
    ///             members: vec!["allUsers".to_string()],
    ///             condition: None,
    ///         }
    ///     ],
    ///     ..Default::default()
    /// };
    /// let policy = client.bucket().set_iam_policy(&bucket, &iam_policy)?;
    /// # client.bucket().delete(bucket)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_iam_policy(&self, bucket: &Bucket, iam: &IamPolicy) -> Result<IamPolicy, Error> {
        self.runtime
            .block_on(self.client.set_iam_policy(bucket, iam))
    }

    /// Checks whether the user provided in the service account has this permission.
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::sync::CloudStorageClient;
    /// # use cloud_storage::Bucket;
    ///
    /// let client = CloudStorageClient::new()?;
    /// let bucket = client.bucket().read("my_bucket")?;
    /// client.bucket().test_iam_permission(&bucket, "storage.buckets.get")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn test_iam_permission(
        &self,
        bucket: &Bucket,
        permission: &str,
    ) -> Result<TestIamPermission, Error> {
        self.runtime.block_on(self.client.test_iam_permission(bucket, permission))
    }
}

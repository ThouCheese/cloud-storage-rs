use crate::{
    bucket::{IamPolicy, TestIamPermission},
    Bucket, NewBucket,
};

/// Operations on [`Bucket`]()s.
#[derive(Debug)]
pub struct BucketClient<'a>(pub(super) &'a super::Client);

impl<'a> BucketClient<'a> {
    /// Creates a new `Bucket`. There are many options that you can provide for creating a new
    /// bucket, so the `NewBucket` resource contains all of them. Note that `NewBucket` implements
    /// `Default`, so you don't have to specify the fields you're not using. And error is returned
    /// if that bucket name is already taken.
    /// ### Example
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::bucket::{Bucket, NewBucket};
    /// use cloud_storage::bucket::{Location, MultiRegion};
    ///
    /// let client = Client::new()?;
    /// let new_bucket = NewBucket {
    ///    name: "cloud-storage-rs-doc-1".to_string(), // this is the only mandatory field
    ///    location: Location::Multi(MultiRegion::Eu),
    ///    ..Default::default()
    /// };
    /// let bucket = client.bucket().create(&new_bucket)?;
    /// # client.bucket().delete(bucket)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create(&self, new_bucket: &NewBucket) -> crate::Result<Bucket> {
        self.0
            .runtime
            .block_on(self.0.client.bucket().create(new_bucket))
    }

    /// Returns all `Bucket`s within this project.
    ///
    /// ### Note
    /// When using incorrect permissions, this function fails silently and returns an empty list.
    ///
    /// ### Example
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::Bucket;
    ///
    /// let client = Client::new()?;
    /// let buckets = client.bucket().list()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> crate::Result<Vec<Bucket>> {
        self.0.runtime.block_on(self.0.client.bucket().list())
    }

    /// Returns a single `Bucket` by its name. If the Bucket does not exist, an error is returned.
    /// ### Example
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::Bucket;
    ///
    /// let client = Client::new()?;
    /// # use cloud_storage::bucket::NewBucket;
    /// # let new_bucket = NewBucket {
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
    pub fn read(&self, name: &str) -> crate::Result<Bucket> {
        self.0.runtime.block_on(self.0.client.bucket().read(name))
    }

    /// Update an existing `Bucket`. If you declare you bucket as mutable, you can edit its fields.
    /// You can then flush your changes to Google Cloud Storage using this method.
    /// ### Example
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::bucket::{Bucket, RetentionPolicy};
    ///
    /// let client = Client::new()?;
    /// # use cloud_storage::bucket::NewBucket;
    /// # let new_bucket = NewBucket {
    /// #   name: "cloud-storage-rs-doc-3".to_string(),
    /// #    ..Default::default()
    /// # };
    /// # let _ = client.bucket().create(&new_bucket)?;
    ///
    /// let mut bucket = client.bucket().read("cloud-storage-rs-doc-3")?;
    /// bucket.retention_policy = Some(RetentionPolicy {
    ///     retention_period: 50,
    ///     effective_time: chrono::Utc::now() + chrono::Duration::seconds(50),
    ///     is_locked: Some(false),
    /// });
    /// client.bucket().update(&bucket)?;
    /// # client.bucket().delete(bucket)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn update(&self, bucket: &Bucket) -> crate::Result<Bucket> {
        self.0
            .runtime
            .block_on(self.0.client.bucket().update(bucket))
    }

    /// Delete an existing `Bucket`. This permanently removes a bucket from Google Cloud Storage.
    /// An error is returned when you don't have sufficient permissions, or when the
    /// `retention_policy` prevents you from deleting your Bucket.
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::Bucket;
    ///
    /// let client = Client::new()?;
    /// # use cloud_storage::bucket::NewBucket;
    /// # let new_bucket = NewBucket {
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
    pub fn delete(&self, bucket: Bucket) -> crate::Result<()> {
        self.0
            .runtime
            .block_on(self.0.client.bucket().delete(bucket))
    }

    /// Returns the [IAM Policy](https://cloud.google.com/iam/docs/) for this bucket.
    /// ### Example
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::Bucket;
    ///
    /// let client = Client::new()?;
    /// # use cloud_storage::bucket::NewBucket;
    /// # let new_bucket = NewBucket {
    /// #   name: "cloud-storage-rs-doc-4".to_string(),
    /// #    ..Default::default()
    /// # };
    /// # let _ = client.bucket().create(&new_bucket)?;
    ///
    /// let bucket = client.bucket().read("cloud-storage-rs-doc-4")?;
    /// let policy = client.bucket().get_iam_policy(&bucket)?;
    /// # client.bucket().delete(bucket)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_iam_policy(&self, bucket: &Bucket) -> crate::Result<IamPolicy> {
        self.0
            .runtime
            .block_on(self.0.client.bucket().get_iam_policy(bucket))
    }

    /// Updates the [IAM Policy](https://cloud.google.com/iam/docs/) for this bucket.
    /// ### Example
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::Bucket;
    /// use cloud_storage::bucket::{IamPolicy, Binding, IamRole, StandardIamRole, Entity};
    ///
    /// let client = Client::new()?;
    /// # use cloud_storage::bucket::NewBucket;
    /// # let new_bucket = NewBucket {
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
    pub fn set_iam_policy(&self, bucket: &Bucket, iam: &IamPolicy) -> crate::Result<IamPolicy> {
        self.0
            .runtime
            .block_on(self.0.client.bucket().set_iam_policy(bucket, iam))
    }

    /// Checks whether the user provided in the service account has this permission.
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::Bucket;
    ///
    /// let client = Client::new()?;
    /// let bucket = client.bucket().read("my-bucket")?;
    /// client.bucket().test_iam_permission(&bucket, "storage.buckets.get")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn test_iam_permission(
        &self,
        bucket: &Bucket,
        permission: &str,
    ) -> crate::Result<TestIamPermission> {
        self.0.runtime.block_on(
            self.0
                .client
                .bucket()
                .test_iam_permission(bucket, permission),
        )
    }
}

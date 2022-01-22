use crate::bucket_access_control::{BucketAccessControl, Entity, NewBucketAccessControl};

/// Operations on [`BucketAccessControl`](BucketAccessControl)s.
#[derive(Debug)]
pub struct BucketAccessControlClient<'a>(pub(super) &'a super::Client);

impl<'a> BucketAccessControlClient<'a> {
    /// Create a new `BucketAccessControl` using the provided `NewBucketAccessControl`, related to
    /// the `Bucket` provided by the `bucket_name` argument.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```rust,no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::bucket_access_control::{BucketAccessControl, NewBucketAccessControl};
    /// use cloud_storage::bucket_access_control::{Role, Entity};
    ///
    /// let client = Client::new()?;
    /// let new_bucket_access_control = NewBucketAccessControl {
    ///     entity: Entity::AllUsers,
    ///     role: Role::Reader,
    /// };
    /// client.bucket_access_control().create("mybucket", &new_bucket_access_control)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create(
        &self,
        bucket: &str,
        new_bucket_access_control: &NewBucketAccessControl,
    ) -> crate::Result<BucketAccessControl> {
        self.0.runtime.block_on(
            self.0
                .client
                .bucket_access_control()
                .create(bucket, new_bucket_access_control),
        )
    }

    /// Returns all `BucketAccessControl`s related to this bucket.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```rust,no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::bucket_access_control::BucketAccessControl;
    ///
    /// let client = Client::new()?;
    /// let acls = client.bucket_access_control().list("mybucket")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self, bucket: &str) -> crate::Result<Vec<BucketAccessControl>> {
        self.0
            .runtime
            .block_on(self.0.client.bucket_access_control().list(bucket))
    }

    /// Returns the ACL entry for the specified entity on the specified bucket.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```rust,no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::bucket_access_control::{BucketAccessControl, Entity};
    ///
    /// let client = Client::new()?;
    /// let controls = client.bucket_access_control().read("mybucket", &Entity::AllUsers)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read(&self, bucket: &str, entity: &Entity) -> crate::Result<BucketAccessControl> {
        self.0
            .runtime
            .block_on(self.0.client.bucket_access_control().read(bucket, entity))
    }

    /// Update this `BucketAccessControl`.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```rust,no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::bucket_access_control::{BucketAccessControl, Entity};
    ///
    /// let client = Client::new()?;
    /// let mut acl = client.bucket_access_control().read("mybucket", &Entity::AllUsers)?;
    /// acl.entity = Entity::AllAuthenticatedUsers;
    /// client.bucket_access_control().update(&acl)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn update(
        &self,
        bucket_access_control: &BucketAccessControl,
    ) -> crate::Result<BucketAccessControl> {
        self.0.runtime.block_on(
            self.0
                .client
                .bucket_access_control()
                .update(bucket_access_control),
        )
    }

    /// Permanently deletes the ACL entry for the specified entity on the specified bucket.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```rust,no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::bucket_access_control::{BucketAccessControl, Entity};
    ///
    /// let client = Client::new()?;
    /// let controls = client.bucket_access_control().read("mybucket", &Entity::AllUsers)?;
    /// client.bucket_access_control().delete(controls)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete(&self, bucket_access_control: BucketAccessControl) -> crate::Result<()> {
        self.0.runtime.block_on(
            self.0
                .client
                .bucket_access_control()
                .delete(bucket_access_control),
        )
    }
}

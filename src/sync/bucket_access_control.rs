use crate::{models::{create, BucketAccessControl, Entity}, Error};


/// Operations on [`BucketAccessControl`](BucketAccessControl)s.
#[derive(Debug)]
pub struct BucketAccessControlClient<'a> {
    pub(crate) client: crate::client::BucketAccessControlClient<'a>,
    pub(crate) runtime: &'a tokio::runtime::Handle,
}

impl<'a> BucketAccessControlClient<'a> {
    /// Create a new `BucketAccessControl` using the provided `create::BucketAccessControl`, related to
    /// the `Bucket` provided by the `bucket_name` argument.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```rust,no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::sync::CloudStorageClient;
    /// # use cloud_storage::models::{BucketAccessControl, create};
    /// # use cloud_storage::models::{Role, Entity};
    ///
    /// let client = CloudStorageClient::new()?;
    /// let new_bucket_access_control = create::BucketAccessControl {
    ///     entity: Entity::AllUsers,
    ///     role: Role::Reader,
    /// };
    /// client.bucket_access_control("my_bucket").create(&new_bucket_access_control)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create(
        &self,
        new_bucket_access_control: &create::BucketAccessControl,
    ) -> Result<BucketAccessControl, Error> {
        self.runtime.block_on(self.client.create_using(new_bucket_access_control))
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
    /// # use cloud_storage::sync::CloudStorageClient;
    /// # use cloud_storage::models::BucketAccessControl;
    ///
    /// let client = CloudStorageClient::new()?;
    /// let acls = client.bucket_access_control("my_bucket").list()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> Result<Vec<BucketAccessControl>, Error> {
        self.runtime.block_on(self.client.list())
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
    /// # use cloud_storage::sync::CloudStorageClient;
    /// # use cloud_storage::models::{BucketAccessControl, Entity};
    ///
    /// let client = CloudStorageClient::new()?;
    /// let controls = client.bucket_access_control("my_bucket").read(&Entity::AllUsers)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read(&self, entity: &Entity) -> Result<BucketAccessControl, Error> {
        self.runtime.block_on(self.client.read(entity))
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
    /// # use cloud_storage::sync::CloudStorageClient;
    /// # use cloud_storage::models::{BucketAccessControl, Entity};
    ///
    /// let client = CloudStorageClient::new()?;
    /// let my_bucket = client.bucket_access_control("my_bucket");
    /// let mut acl = my_bucket.read(&Entity::AllUsers)?;
    /// acl.entity = Entity::AllAuthenticatedUsers;
    /// my_bucket.update(&acl)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn update(
        &self,
        bucket_access_control: &BucketAccessControl,
    ) -> Result<BucketAccessControl, Error> {
        self.runtime.block_on(self.client.update(bucket_access_control))
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
    /// # use cloud_storage::sync::CloudStorageClient;
    /// # use cloud_storage::models::{BucketAccessControl, Entity};
    ///
    /// let client = CloudStorageClient::new()?;
    /// let my_bucket = client.bucket_access_control("my_bucket");
    /// let controls = my_bucket.read(&Entity::AllUsers)?;
    /// my_bucket.delete(controls)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete(&self, bucket_access_control: BucketAccessControl) -> Result<(), Error> {
        self.runtime.block_on(
            self
                .client
                .delete(bucket_access_control),
        )
    }
}

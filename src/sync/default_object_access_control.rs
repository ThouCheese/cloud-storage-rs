use crate::{models::{create, DefaultObjectAccessControl, Entity}, Error};

/// Operations on [`DefaultObjectAccessControl`](DefaultObjectAccessControl)s.
#[derive(Debug)]
pub struct DefaultObjectAccessControlClient<'a> {
    pub(crate) client: &'a crate::client::DefaultObjectAccessControlClient<'a>,
    pub(crate) runtime: &'a tokio::runtime::Handle,
}

impl<'a> DefaultObjectAccessControlClient<'a> {
    /// Create a new `DefaultObjectAccessControl` entry on the specified bucket.
    /// ### Important
    /// Important: This method fails with a `400 Bad Request` response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::default_object_access_control::{
    ///     DefaultObjectAccessControl, create::DefaultObjectAccessControl, Role, Entity,
    /// };
    ///
    /// let client = Client::new()?;
    /// let new_acl = create::DefaultObjectAccessControl {
    ///     entity: Entity::AllAuthenticatedUsers,
    ///     role: Role::Reader,
    /// };
    /// let default_acl = client.default_object_access_control("my_bucket").create(&new_acl)?;
    /// # client.default_object_access_control().delete(default_acl)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create(
        &self,
        new_acl: &create::DefaultObjectAccessControl,
    ) -> Result<DefaultObjectAccessControl, Error> {
        self.runtime.block_on(
            self.client
                .create(new_acl),
        )
    }

    /// Retrieves default object ACL entries on the specified bucket.
    /// ### Important
    /// Important: This method fails with a `400 Bad Request` response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::default_object_access_control::DefaultObjectAccessControl;
    ///
    /// let client = Client::new()?;
    /// let default_acls = client.default_object_access_control().list("my_bucket")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self, bucket: &str) -> Result<Vec<DefaultObjectAccessControl>, Error> {
        self.runtime
            .block_on(self.client.list())
    }

    /// Read a single `DefaultObjectAccessControl`.
    /// The `bucket` argument is the name of the bucket whose `DefaultObjectAccessControl` is to be
    /// read, and the `entity` argument is the entity holding the permission. Options are
    /// Can be "user-`userId`", "user-`email_address`", "group-`group_id`", "group-`email_address`",
    /// "allUsers", or "allAuthenticatedUsers".
    /// ### Important
    /// Important: This method fails with a `400 Bad Request` response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::default_object_access_control::{DefaultObjectAccessControl, Entity};
    ///
    /// let client = Client::new()?;
    /// let default_acl = client.default_object_access_control("my_bucket").read(&Entity::AllUsers)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read(&self, bucket: &str, entity: &Entity) -> Result<DefaultObjectAccessControl, Error> {
        self.runtime.block_on(
            self.client.read(entity),
        )
    }

    /// Update the current `DefaultObjectAccessControl`.
    /// ### Important
    /// Important: This method fails with a `400 Bad Request` response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::default_object_access_control::{DefaultObjectAccessControl, Entity};
    ///
    /// let client = Client::new()?;
    /// let mut default_acl = client.default_object_access_control("my_bucket").read(&Entity::AllUsers)?;
    /// default_acl.entity = Entity::AllAuthenticatedUsers;
    /// client.default_object_access_control().update(&default_acl)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn update(
        &self,
        default_object_access_control: &DefaultObjectAccessControl,
    ) -> Result<DefaultObjectAccessControl, Error> {
        self.runtime.block_on(
            self.client
                .update(default_object_access_control),
        )
    }

    /// Delete this 'DefaultObjectAccessControl`.
    /// ### Important
    /// Important: This method fails with a `400 Bad Request` response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::sync::Client;
    /// use cloud_storage::default_object_access_control::{DefaultObjectAccessControl, Entity};
    ///
    /// let client = Client::new()?;
    /// let mut default_acl = client.default_object_access_control("my_bucket").read(&Entity::AllUsers)?;
    /// client.default_object_access_control().delete(default_acl)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete(
        &self,
        default_object_access_control: DefaultObjectAccessControl,
    ) -> Result<(), crate::Error> {
        self.runtime.block_on(
            self.client
                .delete(default_object_access_control),
        )
    }
}

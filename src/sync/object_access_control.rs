use crate::{models::{create, ObjectAccessControl, Entity}, Error};


/// Operations on [`ObjectAccessControl`](ObjectAccessControl)s.
#[derive(Debug)]
pub struct ObjectAccessControlClient<'a> {
    pub(crate) client: crate::client::ObjectAccessControlClient<'a>,
    pub(crate) runtime: &'a tokio::runtime::Handle,
}

impl<'a> ObjectAccessControlClient<'a> {
    /// Creates a new ACL entry on the specified `object`.
    ///
    /// ### Important
    /// This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    pub fn create(
        &self,
        new_object_access_control: &create::ObjectAccessControl,
    ) -> Result<ObjectAccessControl, Error> {
        self.runtime
            .block_on(self.client.create(new_object_access_control))
    }

    /// Retrieves `ACL` entries on the specified object.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    pub fn list(&self) -> Result<Vec<ObjectAccessControl>, Error> {
        self.runtime
            .block_on(self.client.list())
    }

    /// Returns the `ACL` entry for the specified entity on the specified bucket.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    pub fn read(
        &self,
        entity: &Entity,
    ) -> Result<ObjectAccessControl, Error> {
        self.runtime.block_on(
            self.client.read(entity),
        )
    }

    /// Updates an ACL entry on the specified object.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    pub fn update(
        &self,
        object_access_control: &ObjectAccessControl,
    ) -> Result<ObjectAccessControl, Error> {
        self.runtime.block_on(
            self.client.update(object_access_control),
        )
    }

    /// Permanently deletes the ACL entry for the specified entity on the specified object.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    pub fn delete(&self, object_access_control: ObjectAccessControl) -> Result<(), Error> {
        self.runtime.block_on(
            self.client
                .delete(object_access_control),
        )
    }
}

use crate::{
    bucket_access_control::Entity,
    object_access_control::{NewObjectAccessControl, ObjectAccessControl},
};

/// Operations on [`ObjectAccessControl`](ObjectAccessControl)s.
#[derive(Debug)]
pub struct ObjectAccessControlClient<'a>(pub(super) &'a super::Client);

impl<'a> ObjectAccessControlClient<'a> {
    /// Creates a new ACL entry on the specified `object`.
    ///
    /// ### Important
    /// This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    pub fn create(
        &self,
        bucket: &str,
        object: &str,
        new_object_access_control: &NewObjectAccessControl,
    ) -> crate::Result<ObjectAccessControl> {
        self.0
            .runtime
            .block_on(self.0.client.object_access_control().create(
                bucket,
                object,
                new_object_access_control,
            ))
    }

    /// Retrieves `ACL` entries on the specified object.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    pub fn list(&self, bucket: &str, object: &str) -> crate::Result<Vec<ObjectAccessControl>> {
        self.0
            .runtime
            .block_on(self.0.client.object_access_control().list(bucket, object))
    }

    /// Returns the `ACL` entry for the specified entity on the specified bucket.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    pub fn read(
        &self,
        bucket: &str,
        object: &str,
        entity: &Entity,
    ) -> crate::Result<ObjectAccessControl> {
        self.0.runtime.block_on(
            self.0
                .client
                .object_access_control()
                .read(bucket, object, entity),
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
    ) -> crate::Result<ObjectAccessControl> {
        self.0.runtime.block_on(
            self.0
                .client
                .object_access_control()
                .update(object_access_control),
        )
    }

    /// Permanently deletes the ACL entry for the specified entity on the specified object.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    pub fn delete(&self, object_access_control: ObjectAccessControl) -> crate::Result<()> {
        self.0.runtime.block_on(
            self.0
                .client
                .object_access_control()
                .delete(object_access_control),
        )
    }
}

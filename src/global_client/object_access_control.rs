use crate::{models::{create, ObjectAccessControl, Entity}, Error};

impl ObjectAccessControl {
    /// Creates a new ACL entry on the specified `object`.
    ///
    /// ### Important
    /// This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    pub async fn create(
        bucket: &str,
        object: &str,
        new_object_access_control: &create::ObjectAccessControl,
    ) -> Result<Self, Error> {
        crate::CLOUD_CLIENT
            .object_access_control(bucket, object)
            .create(new_object_access_control)
            .await
    }

    /// The synchronous equivalent of `ObjectAccessControl::create`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn create_sync(
        bucket: &str,
        object: &str,
        new_object_access_control: &create::ObjectAccessControl,
    ) -> Result<Self, Error> {
        crate::runtime()?.block_on(Self::create(bucket, object, new_object_access_control))
    }

    /// Retrieves `ACL` entries on the specified object.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    pub async fn list(bucket: &str, object: &str) -> Result<Vec<Self>, Error> {
        crate::CLOUD_CLIENT
            .object_access_control(bucket, object)
            .list()
            .await
    }

    /// The synchronous equivalent of `ObjectAccessControl::list`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn list_sync(bucket: &str, object: &str) -> Result<Vec<Self>, Error> {
        crate::runtime()?.block_on(Self::list(bucket, object))
    }

    /// Returns the `ACL` entry for the specified entity on the specified bucket.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    pub async fn read(bucket: &str, object: &str, entity: &Entity) -> Result<Self, Error> {
        crate::CLOUD_CLIENT
            .object_access_control(bucket, object)
            .read(entity)
            .await
    }

    /// The synchronous equivalent of `ObjectAccessControl::read`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn read_sync(bucket: &str, object: &str, entity: &Entity) -> Result<Self, Error> {
        crate::runtime()?.block_on(Self::read(bucket, object, entity))
    }

    /// Updates an ACL entry on the specified object.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    pub async fn update(&self) -> Result<Self, Error> {
        crate::CLOUD_CLIENT
            .object_access_control(&self.bucket, &self.object)
            .update(self)
            .await
    }

    /// The synchronous equivalent of `ObjectAccessControl::update`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn update_sync(&self) -> Result<Self, Error> {
        crate::runtime()?.block_on(self.update())
    }

    /// Permanently deletes the ACL entry for the specified entity on the specified object.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    pub async fn delete(self) -> Result<(), Error> {
        crate::CLOUD_CLIENT
            .object_access_control(&self.bucket, &self.object)
            .delete(self)
            .await
    }

    /// The synchronous equivalent of `ObjectAccessControl::delete`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn delete_sync(self) -> Result<(), Error> {
        crate::runtime()?.block_on(self.delete())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Object, models::Role};

    #[tokio::test]
    async fn create() {
        let bucket = crate::global_client::read_test_bucket().await;
        Object::create(
            &bucket.name,
            vec![0, 1],
            "test-object-access-controls-create",
            "text/plain",
            None
        )
        .await
        .unwrap();
        let new_bucket_access_control = create::ObjectAccessControl {
            entity: Entity::AllUsers,
            role: Role::Reader,
        };
        ObjectAccessControl::create(
            &bucket.name,
            "test-object-access-controls-create",
            &new_bucket_access_control,
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn list() {
        let bucket = crate::global_client::read_test_bucket().await;
        Object::create(
            &bucket.name,
            vec![0, 1],
            "test-object-access-controls-list",
            "text/plain",
            None
        )
        .await
        .unwrap();
        ObjectAccessControl::list(&bucket.name, "test-object-access-controls-list")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn read() {
        let bucket = crate::global_client::read_test_bucket().await;
        Object::create(
            &bucket.name,
            vec![0, 1],
            "test-object-access-controls-read",
            "text/plain",
            None
        )
        .await
        .unwrap();
        let new_bucket_access_control = create::ObjectAccessControl {
            entity: Entity::AllUsers,
            role: Role::Reader,
        };
        ObjectAccessControl::create(
            &bucket.name,
            "test-object-access-controls-read",
            &new_bucket_access_control,
        )
        .await
        .unwrap();
        ObjectAccessControl::read(
            &bucket.name,
            "test-object-access-controls-read",
            &Entity::AllUsers,
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn update() {
        // use a seperate bucket to prevent synchronization issues
        let bucket = crate::global_client::create_test_bucket("test-object-access-controls-update").await;
        let new_bucket_access_control = create::ObjectAccessControl {
            entity: Entity::AllUsers,
            role: Role::Reader,
        };
        Object::create(&bucket.name, vec![0, 1], "test-update", "text/plain", None)
            .await
            .unwrap();
        ObjectAccessControl::create(&bucket.name, "test-update", &new_bucket_access_control)
            .await
            .unwrap();
        let mut acl = ObjectAccessControl::read(&bucket.name, "test-update", &Entity::AllUsers)
            .await
            .unwrap();
        acl.entity = Entity::AllAuthenticatedUsers;
        acl.update().await.unwrap();
        Object::delete(&bucket.name, "test-update", None).await.unwrap();
        bucket.delete().await.unwrap();
    }

    #[tokio::test]
    async fn delete() {
        // use a seperate bucket to prevent synchronization issues
        let bucket = crate::global_client::create_test_bucket("test-object-access-controls-delete").await;
        let new_bucket_access_control = create::ObjectAccessControl {
            entity: Entity::AllUsers,
            role: Role::Reader,
        };
        Object::create(&bucket.name, vec![0, 1], "test-delete", "text/plain", None)
            .await
            .unwrap();
        ObjectAccessControl::create(&bucket.name, "test-delete", &new_bucket_access_control)
            .await
            .unwrap();
        let acl = ObjectAccessControl::read(&bucket.name, "test-delete", &Entity::AllUsers)
            .await
            .unwrap();
        acl.delete().await.unwrap();
        Object::delete(&bucket.name, "test-delete", None).await.unwrap();
        bucket.delete().await.unwrap();
    }

    #[cfg(feature = "sync")]
    mod sync {
        use super::*;

        #[test]
        fn create() {
            let bucket = crate::global_client::read_test_bucket_sync();
            Object::create_sync(
                &bucket.name,
                vec![0, 1],
                "test-object-access-controls-create",
                "text/plain",
                None
            )
            .unwrap();
            let new_bucket_access_control = create::ObjectAccessControl {
                entity: Entity::AllUsers,
                role: Role::Reader,
            };
            ObjectAccessControl::create_sync(
                &bucket.name,
                "test-object-access-controls-create",
                &new_bucket_access_control,
            )
            .unwrap();
        }

        #[test]
        fn list() {
            let bucket = crate::global_client::read_test_bucket_sync();
            Object::create_sync(
                &bucket.name,
                vec![0, 1],
                "test-object-access-controls-list",
                "text/plain",
                None
            )
            .unwrap();
            ObjectAccessControl::list_sync(&bucket.name, "test-object-access-controls-list")
                .unwrap();
        }

        #[test]
        fn read() {
            let bucket = crate::global_client::read_test_bucket_sync();
            Object::create_sync(
                &bucket.name,
                vec![0, 1],
                "test-object-access-controls-read",
                "text/plain",
                None
            )
            .unwrap();
            let new_bucket_access_control = create::ObjectAccessControl {
                entity: Entity::AllUsers,
                role: Role::Reader,
            };
            ObjectAccessControl::create_sync(
                &bucket.name,
                "test-object-access-controls-read",
                &new_bucket_access_control,
            )
            .unwrap();
            ObjectAccessControl::read_sync(
                &bucket.name,
                "test-object-access-controls-read",
                &Entity::AllUsers,
            )
            .unwrap();
        }

        #[test]
        fn update() {
            // use a seperate bucket to prevent synchronization issues
            let bucket = crate::global_client::create_test_bucket_sync("test-object-access-controls-update");
            let new_bucket_access_control = create::ObjectAccessControl {
                entity: Entity::AllUsers,
                role: Role::Reader,
            };
            Object::create_sync(&bucket.name, vec![0, 1], "test-update", "text/plain", None).unwrap();
            ObjectAccessControl::create_sync(
                &bucket.name,
                "test-update",
                &new_bucket_access_control,
            )
            .unwrap();
            let mut acl =
                ObjectAccessControl::read_sync(&bucket.name, "test-update", &Entity::AllUsers)
                    .unwrap();
            acl.entity = Entity::AllAuthenticatedUsers;
            acl.update_sync().unwrap();
            Object::delete_sync(&bucket.name, "test-update", None).unwrap();
            bucket.delete_sync().unwrap();
        }

        #[test]
        fn delete() {
            // use a seperate bucket to prevent synchronization issues
            let bucket = crate::global_client::create_test_bucket_sync("test-object-access-controls-delete");
            let new_bucket_access_control = create::ObjectAccessControl {
                entity: Entity::AllUsers,
                role: Role::Reader,
            };
            Object::create_sync(&bucket.name, vec![0, 1], "test-delete", "text/plain", None).unwrap();
            ObjectAccessControl::create_sync(
                &bucket.name,
                "test-delete",
                &new_bucket_access_control,
            )
            .unwrap();
            let acl =
                ObjectAccessControl::read_sync(&bucket.name, "test-delete", &Entity::AllUsers)
                    .unwrap();
            acl.delete_sync().unwrap();
            Object::delete_sync(&bucket.name, "test-delete", None).unwrap();
            bucket.delete_sync().unwrap();
        }
    }
}

use crate::{models::{DefaultObjectAccessControl, create, Entity}, Error};

impl DefaultObjectAccessControl {
    /// Create a new `DefaultObjectAccessControl` entry on the specified bucket.
    /// ### Important
    /// Important: This method fails with a `400 Bad Request` response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::default_object_access_control::{
    ///     DefaultObjectAccessControl, create::DefaultObjectAccessControl, Role, Entity,
    /// };
    ///
    /// let new_acl = create::DefaultObjectAccessControl {
    ///     entity: Entity::AllAuthenticatedUsers,
    ///     role: Role::Reader,
    /// };
    /// let default_acl = DefaultObjectAccessControl::create("my_bucket", &new_acl).await?;
    /// # default_acl.delete().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(
        bucket: &str,
        new_acl: &create::DefaultObjectAccessControl,
    ) -> Result<Self, Error> {
        crate::CLOUD_CLIENT.default_object_access_control(bucket).create(new_acl).await
    }

    /// The synchronous equivalent of `DefautObjectAccessControl::create`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn create_sync(
        bucket: &str,
        new_acl: &create::DefaultObjectAccessControl,
    ) -> Result<Self, Error> {
        crate::runtime()?.block_on(Self::create(bucket, new_acl))
    }

    /// Retrieves default object ACL entries on the specified bucket.
    /// ### Important
    /// Important: This method fails with a `400 Bad Request` response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::default_object_access_control::DefaultObjectAccessControl;
    ///
    /// let default_acls = DefaultObjectAccessControl::list("my_bucket").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(bucket: &str) -> Result<Vec<Self>, Error> {
        crate::CLOUD_CLIENT.default_object_access_control(bucket).list().await
    }

    /// The synchronous equivalent of `DefautObjectAccessControl::list`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn list_sync(bucket: &str) -> Result<Vec<Self>, Error> {
        crate::runtime()?.block_on(Self::list(bucket))
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
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::default_object_access_control::{DefaultObjectAccessControl, Entity};
    ///
    /// let default_acl = DefaultObjectAccessControl::read("my_bucket", &Entity::AllUsers).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn read(bucket: &str, entity: &Entity) -> Result<Self, Error> {
        crate::CLOUD_CLIENT.default_object_access_control(bucket).read(entity)
            .await
    }

    /// The synchronous equivalent of `DefautObjectAccessControl::read`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn read_sync(bucket: &str, entity: &Entity) -> Result<Self, Error> {
        crate::runtime()?.block_on(Self::read(bucket, entity))
    }

    /// Update the current `DefaultObjectAccessControl`.
    /// ### Important
    /// Important: This method fails with a `400 Bad Request` response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::default_object_access_control::{DefaultObjectAccessControl, Entity};
    ///
    /// let mut default_acl = DefaultObjectAccessControl::read("my_bucket", &Entity::AllUsers).await?;
    /// default_acl.entity = Entity::AllAuthenticatedUsers;
    /// default_acl.update().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(&self) -> Result<Self, Error> {
        crate::CLOUD_CLIENT.default_object_access_control(&self.bucket).update(self).await
    }

    /// The synchronous equivalent of `DefautObjectAccessControl::update`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn update_sync(&self) -> Result<Self, Error> {
        crate::runtime()?.block_on(self.update())
    }

    /// Delete this 'DefaultObjectAccessControl`.
    /// ### Important
    /// Important: This method fails with a `400 Bad Request` response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::default_object_access_control::{DefaultObjectAccessControl, Entity};
    ///
    /// let mut default_acl = DefaultObjectAccessControl::read("my_bucket", &Entity::AllUsers).await?;
    /// default_acl.delete().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(self) -> Result<(), crate::Error> {
        crate::CLOUD_CLIENT.default_object_access_control(&self.bucket).delete(self).await
    }

    /// The synchronous equivalent of `DefautObjectAccessControl::delete`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn delete_sync(self) -> Result<(), crate::Error> {
        crate::runtime()?.block_on(self.delete())
    }
}

#[cfg(test)]
mod tests {
    use crate::models::Role;

    use super::*;

    #[tokio::test]
    async fn create() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::global_client::read_test_bucket().await;
        let new_acl = create::DefaultObjectAccessControl {
            entity: Entity::AllUsers,
            role: Role::Reader,
        };
        DefaultObjectAccessControl::create(&bucket.name, &new_acl).await?;
        Ok(())
    }

    #[tokio::test]
    async fn read() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::global_client::read_test_bucket().await;
        DefaultObjectAccessControl::read(&bucket.name, &Entity::AllUsers).await?;
        Ok(())
    }

    #[tokio::test]
    async fn list() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::global_client::read_test_bucket().await;
        DefaultObjectAccessControl::list(&bucket.name).await?;
        Ok(())
    }

    #[tokio::test]
    async fn update() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::global_client::read_test_bucket().await;
        let new_acl = create::DefaultObjectAccessControl {
            entity: Entity::AllUsers,
            role: Role::Reader,
        };
        let mut default_acl = DefaultObjectAccessControl::create(&bucket.name, &new_acl).await?;
        default_acl.entity = Entity::AllAuthenticatedUsers;
        default_acl.update().await?;
        Ok(())
    }

    #[tokio::test]
    async fn delete() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::global_client::read_test_bucket().await;
        let default_acl =
            DefaultObjectAccessControl::read(&bucket.name, &Entity::AllAuthenticatedUsers).await?;
        default_acl.delete().await?;
        Ok(())
    }

    #[cfg(feature = "sync")]
    mod sync {
        use super::*;

        #[test]
        fn create() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::global_client::read_test_bucket_sync();
            let new_acl = create::DefaultObjectAccessControl {
                entity: Entity::AllUsers,
                role: Role::Reader,
            };
            DefaultObjectAccessControl::create_sync(&bucket.name, &new_acl)?;
            Ok(())
        }

        #[test]
        fn read() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::global_client::read_test_bucket_sync();
            let new_acl = create::DefaultObjectAccessControl {
                entity: Entity::AllUsers,
                role: Role::Reader,
            };
            DefaultObjectAccessControl::create_sync(&bucket.name, &new_acl)?;
            DefaultObjectAccessControl::read_sync(&bucket.name, &Entity::AllUsers)?;
            Ok(())
        }

        #[test]
        fn list() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::global_client::read_test_bucket_sync();
            DefaultObjectAccessControl::list_sync(&bucket.name)?;
            Ok(())
        }

        #[test]
        fn update() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::global_client::read_test_bucket_sync();
            let new_acl = create::DefaultObjectAccessControl {
                entity: Entity::AllUsers,
                role: Role::Reader,
            };
            let mut default_acl = DefaultObjectAccessControl::create_sync(&bucket.name, &new_acl)?;
            default_acl.entity = Entity::AllAuthenticatedUsers;
            default_acl.update_sync()?;
            Ok(())
        }

        #[test]
        fn delete() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::global_client::read_test_bucket_sync();
            let new_acl = create::DefaultObjectAccessControl {
                entity: Entity::AllUsers,
                role: Role::Reader,
            };
            let acl = DefaultObjectAccessControl::create_sync(&bucket.name, &new_acl)?;
            acl.delete_sync()?;
            Ok(())
        }
    }
}

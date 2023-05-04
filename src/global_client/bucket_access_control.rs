use crate::{models::{BucketAccessControl, create, Entity}, Error};

impl BucketAccessControl {
    /// Create a new `BucketAccessControl` using the provided `create::BucketAccessControl`, related to
    /// the `Bucket` provided by the `bucket_name` argument.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::models::{BucketAccessControl, create};
    /// # use cloud_storage::models::{Role, Entity};
    ///
    /// let new_bucket_access_control = create::BucketAccessControl {
    ///     entity: Entity::AllUsers,
    ///     role: Role::Reader,
    /// };
    /// BucketAccessControl::create_using("my_bucket", &new_bucket_access_control).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_using(
        bucket: &str,
        new_bucket_access_control: &create::BucketAccessControl,
    ) -> Result<Self, Error> {
        crate::CLOUD_CLIENT
            .bucket_access_control(bucket)
            .create_using(new_bucket_access_control)
            .await
    }

    /// The synchronous equivalent of `BucketAccessControl::create`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn create_sync(
        bucket: &str,
        new_bucket_access_control: &create::BucketAccessControl,
    ) -> Result<Self, Error> {
        crate::runtime()?.block_on(Self::create_using(bucket, new_bucket_access_control))
    }

    /// Returns all `BucketAccessControl`s related to this bucket.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::models::BucketAccessControl;
    ///
    /// let acls = BucketAccessControl::list("my_bucket").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(bucket: &str) -> Result<Vec<Self>, Error> {
        crate::CLOUD_CLIENT
            .bucket_access_control(bucket)
            .list()
            .await
    }

    /// The synchronous equivalent of `BucketAccessControl::list`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn list_sync(bucket: &str) -> Result<Vec<Self>, Error> {
        crate::runtime()?.block_on(Self::list(bucket))
    }

    /// Returns the ACL entry for the specified entity on the specified bucket.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::models::{BucketAccessControl, Entity};
    ///
    /// let controls = BucketAccessControl::read("my_bucket", &Entity::AllUsers).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn read(bucket: &str, entity: &Entity) -> Result<Self, Error> {
        crate::CLOUD_CLIENT
            .bucket_access_control(bucket)
            .read(entity)
            .await
    }

    /// The synchronous equivalent of `BucketAccessControl::read`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn read_sync(bucket: &str, entity: &Entity) -> Result<Self, Error> {
        crate::runtime()?.block_on(Self::read(bucket, entity))
    }

    /// Update this `BucketAccessControl`.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::models::{BucketAccessControl, Entity};
    ///
    /// let mut acl = BucketAccessControl::read("my_bucket", &Entity::AllUsers).await?;
    /// acl.entity = Entity::AllAuthenticatedUsers;
    /// acl.update().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(&self) -> Result<Self, Error> {
        crate::CLOUD_CLIENT
            .bucket_access_control(&self.bucket)
            .update(self)
            .await
    }

    /// The synchronous equivalent of `BucketAccessControl::update`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn update_sync(&self) -> Result<Self, Error> {
        crate::runtime()?.block_on(self.update())
    }

    /// Permanently deletes the ACL entry for the specified entity on the specified bucket.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::models::{BucketAccessControl, Entity};
    ///
    /// let controls = BucketAccessControl::read("my_bucket", &Entity::AllUsers).await?;
    /// controls.delete().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(self) -> Result<(), Error> {
        crate::CLOUD_CLIENT
            .bucket_access_control(&self.bucket)
            .delete(self)
            .await
    }

    /// The synchronous equivalent of `BucketAccessControl::delete`.
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
    use crate::models::{create, Entity, Role, BucketAccessControl};


    #[tokio::test]
    async fn create() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::global_client::read_test_bucket().await;
        let new_bucket_access_control = create::BucketAccessControl {
            entity: Entity::AllUsers,
            role: Role::Reader,
        };
        BucketAccessControl::create_using(&bucket.name, &new_bucket_access_control)
            .await
            .unwrap();
        Ok(())
    }

    #[tokio::test]
    async fn list() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::global_client::read_test_bucket().await;
        BucketAccessControl::list(&bucket.name).await?;
        Ok(())
    }

    #[tokio::test]
    async fn read() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::global_client::read_test_bucket().await;
        BucketAccessControl::read(&bucket.name, &Entity::AllUsers).await?;
        Ok(())
    }

    #[tokio::test]
    async fn update() -> Result<(), Box<dyn std::error::Error>> {
        // use a seperate bucket to prevent synchronization issues
        let bucket = crate::global_client::create_test_bucket("test-update-bucket-access-controls").await;
        let new_bucket_access_control = create::BucketAccessControl {
            entity: Entity::AllUsers,
            role: Role::Reader,
        };
        BucketAccessControl::create_using(&bucket.name, &new_bucket_access_control).await?;
        let mut acl = BucketAccessControl::read(&bucket.name, &Entity::AllUsers).await?;
        acl.entity = Entity::AllAuthenticatedUsers;
        acl.update().await?;
        bucket.delete().await?;
        Ok(())
    }

    #[tokio::test]
    async fn delete() -> Result<(), Box<dyn std::error::Error>> {
        // use a seperate bucket to prevent synchronization issues
        let bucket = crate::global_client::create_test_bucket("test-delete-bucket-access-controls").await;
        let new_bucket_access_control = create::BucketAccessControl {
            entity: Entity::AllUsers,
            role: Role::Reader,
        };
        BucketAccessControl::create_using(&bucket.name, &new_bucket_access_control).await?;
        let acl = BucketAccessControl::read(&bucket.name, &Entity::AllUsers).await?;
        acl.delete().await?;
        bucket.delete().await?;
        Ok(())
    }

    #[cfg(feature = "sync")]
    mod sync {
        use super::*;

        #[test]
        fn create() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::global_client::read_test_bucket_sync();
            let new_bucket_access_control = create::BucketAccessControl {
                entity: Entity::AllUsers,
                role: Role::Reader,
            };
            BucketAccessControl::create_sync(&bucket.name, &new_bucket_access_control)?;
            Ok(())
        }

        #[test]
        fn list() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::global_client::read_test_bucket_sync();
            BucketAccessControl::list_sync(&bucket.name)?;
            Ok(())
        }

        #[test]
        fn read() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::global_client::read_test_bucket_sync();
            BucketAccessControl::read_sync(&bucket.name, &Entity::AllUsers)?;
            Ok(())
        }

        #[test]
        fn update() -> Result<(), Box<dyn std::error::Error>> {
            // use a seperate bucket to prevent synchronization issues
            let bucket = crate::global_client::create_test_bucket_sync("test-update-bucket-access-controls");
            let new_bucket_access_control = create::BucketAccessControl {
                entity: Entity::AllUsers,
                role: Role::Reader,
            };
            BucketAccessControl::create_sync(&bucket.name, &new_bucket_access_control)?;
            let mut acl = BucketAccessControl::read_sync(&bucket.name, &Entity::AllUsers)?;
            acl.entity = Entity::AllAuthenticatedUsers;
            acl.update_sync()?;
            bucket.delete_sync()?;
            Ok(())
        }

        #[test]
        fn delete() -> Result<(), Box<dyn std::error::Error>> {
            // use a seperate bucket to prevent synchronization issues
            let bucket = crate::global_client::create_test_bucket_sync("test-delete-bucket-access-controls");
            let new_bucket_access_control = create::BucketAccessControl {
                entity: Entity::AllUsers,
                role: Role::Reader,
            };
            BucketAccessControl::create_sync(&bucket.name, &new_bucket_access_control)?;
            let acl = BucketAccessControl::read_sync(&bucket.name, &Entity::AllUsers)?;
            acl.delete_sync()?;
            bucket.delete_sync()?;
            Ok(())
        }
    }
}

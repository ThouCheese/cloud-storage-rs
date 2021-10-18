pub use crate::resources::common::{Entity, ProjectTeam, Role};

/// The BucketAccessControl resource represents the Access Control Lists (ACLs) for buckets within
/// Google Cloud Storage. ACLs let you specify who has access to your data and to what extent.
///
/// ```text,ignore
/// Important: This method fails with a 400 Bad Request response for buckets with uniform
/// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
/// control access instead.
/// ```
///
/// There are three roles that can be assigned to an entity:
///
/// * READERs can get the bucket, though no acl property will be returned, and list the bucket's
/// objects.
/// * WRITERs are READERs, and they can insert objects into the bucket and delete the bucket's
/// objects.
/// * OWNERs are WRITERs, and they can get the acl property of a bucket, update a bucket, and call
/// all BucketAccessControl methods on the bucket.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BucketAccessControl {
    /// The kind of item this is. For bucket access control entries, this is always
    /// `storage#bucketAccessControl`.
    pub kind: String,
    /// The ID of the access-control entry.
    pub id: String,
    /// The link to this access-control entry.
    pub self_link: String,
    /// The name of the bucket.
    pub bucket: String,
    /// The entity holding the permission, in one of the following forms:
    ///
    /// * `user-userId`
    /// * `user-email`
    /// * `group-groupId`
    /// * `group-email`
    /// * `domain-domain`
    /// * `project-team-projectId`
    /// * `allUsers`
    /// * `allAuthenticatedUsers`
    ///
    /// Examples:
    ///
    /// * The user liz@example.com would be user-liz@example.com.
    /// * The group example@googlegroups.com would be group-example@googlegroups.com.
    /// * To refer to all members of the G Suite for Business domain example.com, the entity would
    /// be domain-example.com.
    pub entity: Entity,
    /// The access permission for the entity.
    pub role: Role,
    /// The email address associated with the entity, if any.
    pub email: Option<String>,
    /// The ID for the entity, if any.
    pub entity_id: Option<String>,
    /// The domain associated with the entity, if any.
    pub domain: Option<String>,
    /// The project team associated with the entity, if any.
    pub project_team: Option<ProjectTeam>,
    /// HTTP 1.1 Entity tag for the access-control entry.
    pub etag: String,
}

/// Model that can be used to create a new BucketAccessControl object.
#[derive(Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewBucketAccessControl {
    /// The entity holding the permission, in one of the following forms:
    ///
    /// * `user-userId`
    /// * `user-email`
    /// * `group-groupId`
    /// * `group-email`
    /// * `domain-domain`
    /// * `project-team-projectId`
    /// * `allUsers`
    /// * `allAuthenticatedUsers`
    ///
    /// Examples:
    ///
    /// * The user liz@example.com would be user-liz@example.com.
    /// * The group example@googlegroups.com would be group-example@googlegroups.com.
    /// * To refer to all members of the G Suite for Business domain example.com, the entity would
    /// be domain-example.com.
    pub entity: Entity,
    /// The access permission for the entity.
    pub role: Role,
}

impl BucketAccessControl {
    /// Create a new `BucketAccessControl` using the provided `NewBucketAccessControl`, related to
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
    /// use cloud_storage::bucket_access_control::{BucketAccessControl, NewBucketAccessControl};
    /// use cloud_storage::bucket_access_control::{Role, Entity};
    ///
    /// let new_bucket_access_control = NewBucketAccessControl {
    ///     entity: Entity::AllUsers,
    ///     role: Role::Reader,
    /// };
    /// BucketAccessControl::create("mybucket", &new_bucket_access_control).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "global-client")]
    pub async fn create(
        bucket: &str,
        new_bucket_access_control: &NewBucketAccessControl,
    ) -> crate::Result<Self> {
        crate::CLOUD_CLIENT
            .bucket_access_control()
            .create(bucket, new_bucket_access_control)
            .await
    }

    /// The synchronous equivalent of `BucketAccessControl::create`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(all(feature = "global-client", feature = "sync"))]
    pub fn create_sync(
        bucket: &str,
        new_bucket_access_control: &NewBucketAccessControl,
    ) -> crate::Result<Self> {
        crate::runtime()?.block_on(Self::create(bucket, new_bucket_access_control))
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
    /// use cloud_storage::bucket_access_control::BucketAccessControl;
    ///
    /// let acls = BucketAccessControl::list("mybucket").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "global-client")]
    pub async fn list(bucket: &str) -> crate::Result<Vec<Self>> {
        crate::CLOUD_CLIENT
            .bucket_access_control()
            .list(bucket)
            .await
    }

    /// The synchronous equivalent of `BucketAccessControl::list`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(all(feature = "global-client", feature = "sync"))]
    pub fn list_sync(bucket: &str) -> crate::Result<Vec<Self>> {
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
    /// use cloud_storage::bucket_access_control::{BucketAccessControl, Entity};
    ///
    /// let controls = BucketAccessControl::read("mybucket", &Entity::AllUsers).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "global-client")]
    pub async fn read(bucket: &str, entity: &Entity) -> crate::Result<Self> {
        crate::CLOUD_CLIENT
            .bucket_access_control()
            .read(bucket, entity)
            .await
    }

    /// The synchronous equivalent of `BucketAccessControl::read`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(all(feature = "global-client", feature = "sync"))]
    pub fn read_sync(bucket: &str, entity: &Entity) -> crate::Result<Self> {
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
    /// use cloud_storage::bucket_access_control::{BucketAccessControl, Entity};
    ///
    /// let mut acl = BucketAccessControl::read("mybucket", &Entity::AllUsers).await?;
    /// acl.entity = Entity::AllAuthenticatedUsers;
    /// acl.update().await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "global-client")]
    pub async fn update(&self) -> crate::Result<Self> {
        crate::CLOUD_CLIENT
            .bucket_access_control()
            .update(self)
            .await
    }

    /// The synchronous equivalent of `BucketAccessControl::update`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(all(feature = "global-client", feature = "sync"))]
    pub fn update_sync(&self) -> crate::Result<Self> {
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
    /// use cloud_storage::bucket_access_control::{BucketAccessControl, Entity};
    ///
    /// let controls = BucketAccessControl::read("mybucket", &Entity::AllUsers).await?;
    /// controls.delete().await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "global-client")]
    pub async fn delete(self) -> crate::Result<()> {
        crate::CLOUD_CLIENT
            .bucket_access_control()
            .delete(self)
            .await
    }

    /// The synchronous equivalent of `BucketAccessControl::delete`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(all(feature = "global-client", feature = "sync"))]
    pub fn delete_sync(self) -> crate::Result<()> {
        crate::runtime()?.block_on(self.delete())
    }
}

#[cfg(all(test, feature = "global-client"))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::read_test_bucket().await;
        let new_bucket_access_control = NewBucketAccessControl {
            entity: Entity::AllUsers,
            role: Role::Reader,
        };
        BucketAccessControl::create(&bucket.name, &new_bucket_access_control)
            .await
            .unwrap();
        Ok(())
    }

    #[tokio::test]
    async fn list() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::read_test_bucket().await;
        BucketAccessControl::list(&bucket.name).await?;
        Ok(())
    }

    #[tokio::test]
    async fn read() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::read_test_bucket().await;
        BucketAccessControl::read(&bucket.name, &Entity::AllUsers).await?;
        Ok(())
    }

    #[tokio::test]
    async fn update() -> Result<(), Box<dyn std::error::Error>> {
        // use a seperate bucket to prevent synchronization issues
        let bucket = crate::create_test_bucket("test-update-bucket-access-controls").await;
        let new_bucket_access_control = NewBucketAccessControl {
            entity: Entity::AllUsers,
            role: Role::Reader,
        };
        BucketAccessControl::create(&bucket.name, &new_bucket_access_control).await?;
        let mut acl = BucketAccessControl::read(&bucket.name, &Entity::AllUsers).await?;
        acl.entity = Entity::AllAuthenticatedUsers;
        acl.update().await?;
        bucket.delete().await?;
        Ok(())
    }

    #[tokio::test]
    async fn delete() -> Result<(), Box<dyn std::error::Error>> {
        // use a seperate bucket to prevent synchronization issues
        let bucket = crate::create_test_bucket("test-delete-bucket-access-controls").await;
        let new_bucket_access_control = NewBucketAccessControl {
            entity: Entity::AllUsers,
            role: Role::Reader,
        };
        BucketAccessControl::create(&bucket.name, &new_bucket_access_control).await?;
        let acl = BucketAccessControl::read(&bucket.name, &Entity::AllUsers).await?;
        acl.delete().await?;
        bucket.delete().await?;
        Ok(())
    }

    #[cfg(all(feature = "global-client", feature = "sync"))]
    mod sync {
        use super::*;

        #[test]
        fn create() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::read_test_bucket_sync();
            let new_bucket_access_control = NewBucketAccessControl {
                entity: Entity::AllUsers,
                role: Role::Reader,
            };
            BucketAccessControl::create_sync(&bucket.name, &new_bucket_access_control)?;
            Ok(())
        }

        #[test]
        fn list() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::read_test_bucket_sync();
            BucketAccessControl::list_sync(&bucket.name)?;
            Ok(())
        }

        #[test]
        fn read() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::read_test_bucket_sync();
            BucketAccessControl::read_sync(&bucket.name, &Entity::AllUsers)?;
            Ok(())
        }

        #[test]
        fn update() -> Result<(), Box<dyn std::error::Error>> {
            // use a seperate bucket to prevent synchronization issues
            let bucket = crate::create_test_bucket_sync("test-update-bucket-access-controls");
            let new_bucket_access_control = NewBucketAccessControl {
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
            let bucket = crate::create_test_bucket_sync("test-delete-bucket-access-controls");
            let new_bucket_access_control = NewBucketAccessControl {
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

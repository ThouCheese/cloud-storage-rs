#![allow(unused_imports)]

pub use crate::resources::common::{Entity, ProjectTeam, Role};
use crate::{error::GoogleResponse, resources::common::ListResponse};

/// The DefaultObjectAccessControls resources represent the Access Control Lists (ACLs) applied to a
/// new object within Google Cloud Storage when no ACL was provided for that object. ACLs let you
/// specify who has access to your data and to what extent.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultObjectAccessControl {
    /// The kind of item this is. For object access control entries, this is always
    /// storage#objectAccessControl.
    pub kind: String,
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
    /// The bucket this resource belongs to.
    #[serde(default)]
    pub bucket: String, // this field is not returned by Google, but we populate it manually for the
                        // convenience of the end user.
}

/// Model that can be used to create a new DefaultObjectAccessControl object.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewDefaultObjectAccessControl {
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
    ///     DefaultObjectAccessControl, NewDefaultObjectAccessControl, Role, Entity,
    /// };
    ///
    /// let new_acl = NewDefaultObjectAccessControl {
    ///     entity: Entity::AllAuthenticatedUsers,
    ///     role: Role::Reader,
    /// };
    /// let default_acl = DefaultObjectAccessControl::create("mybucket", &new_acl).await?;
    /// # default_acl.delete().await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "global-client")]
    pub async fn create(
        bucket: &str,
        new_acl: &NewDefaultObjectAccessControl,
    ) -> crate::Result<Self> {
        crate::CLOUD_CLIENT
            .default_object_access_control()
            .create(bucket, new_acl)
            .await
    }

    /// The synchronous equivalent of `DefautObjectAccessControl::create`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(all(feature = "global-client", feature = "sync"))]
    pub fn create_sync(
        bucket: &str,
        new_acl: &NewDefaultObjectAccessControl,
    ) -> crate::Result<Self> {
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
    /// let default_acls = DefaultObjectAccessControl::list("mybucket").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "global-client")]
    pub async fn list(bucket: &str) -> crate::Result<Vec<Self>> {
        crate::CLOUD_CLIENT
            .default_object_access_control()
            .list(bucket)
            .await
    }

    /// The synchronous equivalent of `DefautObjectAccessControl::list`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(all(feature = "global-client", feature = "sync"))]
    pub fn list_sync(bucket: &str) -> crate::Result<Vec<Self>> {
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
    /// let default_acl = DefaultObjectAccessControl::read("mybucket", &Entity::AllUsers).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "global-client")]
    pub async fn read(bucket: &str, entity: &Entity) -> crate::Result<Self> {
        crate::CLOUD_CLIENT
            .default_object_access_control()
            .read(bucket, entity)
            .await
    }

    /// The synchronous equivalent of `DefautObjectAccessControl::read`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(all(feature = "global-client", feature = "sync"))]
    pub fn read_sync(bucket: &str, entity: &Entity) -> crate::Result<Self> {
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
    #[cfg(feature = "global-client")]
    pub async fn update(&self) -> crate::Result<Self> {
        crate::CLOUD_CLIENT
            .default_object_access_control()
            .update(self)
            .await
    }

    /// The synchronous equivalent of `DefautObjectAccessControl::update`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(all(feature = "global-client", feature = "sync"))]
    pub fn update_sync(&self) -> crate::Result<Self> {
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
    #[cfg(feature = "global-client")]
    pub async fn delete(self) -> Result<(), crate::Error> {
        crate::CLOUD_CLIENT
            .default_object_access_control()
            .delete(self)
            .await
    }

    /// The synchronous equivalent of `DefautObjectAccessControl::delete`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(all(feature = "global-client", feature = "sync"))]
    pub fn delete_sync(self) -> Result<(), crate::Error> {
        crate::runtime()?.block_on(self.delete())
    }
}

#[cfg(all(test, feature = "global-client"))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::read_test_bucket().await;
        let new_acl = NewDefaultObjectAccessControl {
            entity: Entity::AllUsers,
            role: Role::Reader,
        };
        DefaultObjectAccessControl::create(&bucket.name, &new_acl).await?;
        Ok(())
    }

    #[tokio::test]
    async fn read() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::read_test_bucket().await;
        NewDefaultObjectAccessControl {
            entity: Entity::AllUsers,
            role: Role::Reader,
        };
        DefaultObjectAccessControl::read(&bucket.name, &Entity::AllUsers).await?;
        Ok(())
    }

    #[tokio::test]
    async fn list() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::read_test_bucket().await;
        DefaultObjectAccessControl::list(&bucket.name).await?;
        Ok(())
    }

    #[tokio::test]
    async fn update() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::read_test_bucket().await;
        let new_acl = NewDefaultObjectAccessControl {
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
        let bucket = crate::read_test_bucket().await;
        let default_acl =
            DefaultObjectAccessControl::read(&bucket.name, &Entity::AllAuthenticatedUsers).await?;
        default_acl.delete().await?;
        Ok(())
    }

    #[cfg(all(feature = "global-client", feature = "sync"))]
    mod sync {
        use super::*;

        #[test]
        fn create() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::read_test_bucket_sync();
            let new_acl = NewDefaultObjectAccessControl {
                entity: Entity::AllUsers,
                role: Role::Reader,
            };
            DefaultObjectAccessControl::create_sync(&bucket.name, &new_acl)?;
            Ok(())
        }

        #[test]
        fn read() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::read_test_bucket_sync();
            let new_acl = NewDefaultObjectAccessControl {
                entity: Entity::AllUsers,
                role: Role::Reader,
            };
            DefaultObjectAccessControl::create_sync(&bucket.name, &new_acl)?;
            DefaultObjectAccessControl::read_sync(&bucket.name, &Entity::AllUsers)?;
            Ok(())
        }

        #[test]
        fn list() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::read_test_bucket_sync();
            DefaultObjectAccessControl::list_sync(&bucket.name)?;
            Ok(())
        }

        #[test]
        fn update() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::read_test_bucket_sync();
            let new_acl = NewDefaultObjectAccessControl {
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
            let bucket = crate::read_test_bucket_sync();
            let new_acl = NewDefaultObjectAccessControl {
                entity: Entity::AllUsers,
                role: Role::Reader,
            };
            let acl = DefaultObjectAccessControl::create_sync(&bucket.name, &new_acl)?;
            acl.delete_sync()?;
            Ok(())
        }
    }
}

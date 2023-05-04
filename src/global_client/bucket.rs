use crate::{Bucket, models::{create, IamPolicy, TestIamPermission}, Error};

impl Bucket {
    /// Creates a new `Bucket`. There are many options that you can provide for creating a new
    /// bucket, so the `NewBucket` resource contains all of them. Note that `NewBucket` implements
    /// `Default`, so you don't have to specify the fields you're not using. And error is returned
    /// if that bucket name is already taken.
    /// ### Example
    /// ```
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::bucket::{Bucket, create::Bucket};
    /// use cloud_storage::bucket::{Location, MultiRegion};
    ///
    /// let new_bucket = create::Bucket {
    ///    name: "cloud-storage-rs-doc-1".to_string(), // this is the only mandatory field
    ///    location: Location::Multi(MultiRegion::Eu),
    ///    ..Default::default()
    /// };
    /// let bucket = Bucket::create(&new_bucket).await?;
    /// # bucket.delete().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(new_bucket: &create::Bucket) -> Result<Self, Error> {
        crate::CLOUD_CLIENT.bucket().create(new_bucket).await
    }

    /// The synchronous equivalent of `Bucket::create`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn create_sync(new_bucket: &create::Bucket) -> Result<Self, Error> {
        crate::runtime()?.block_on(Self::create(new_bucket))
    }

    /// Returns all `Bucket`s within this project.
    ///
    /// ### Note
    /// When using incorrect permissions, this function fails silently and returns an empty list.
    ///
    /// ### Example
    /// ```
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Bucket;
    ///
    /// let buckets = Bucket::list().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list() -> Result<Vec<Self>, Error> {
        crate::CLOUD_CLIENT.bucket().list().await
    }

    /// The synchronous equivalent of `Bucket::list`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn list_sync() -> Result<Vec<Self>, Error> {
        crate::runtime()?.block_on(Self::list())
    }

    /// Returns a single `Bucket` by its name. If the Bucket does not exist, an error is returned.
    /// ### Example
    /// ```
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Bucket;
    /// # use cloud_storage::bucket::NewBucket;
    /// # let new_bucket = create::Bucket {
    /// #   name: "cloud-storage-rs-doc-2".to_string(),
    /// #    ..Default::default()
    /// # };
    /// # let _ = Bucket::create(&new_bucket).await?;
    ///
    /// let bucket = Bucket::read("cloud-storage-rs-doc-2").await?;
    /// # bucket.delete().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn read(name: &str) -> Result<Self, Error> {
        crate::CLOUD_CLIENT.bucket().read(name).await
    }

    /// The synchronous equivalent of `Bucket::read`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn read_sync(name: &str) -> Result<Self, Error> {
        crate::runtime()?.block_on(Self::read(name))
    }

    /// Update an existing `Bucket`. If you declare you bucket as mutable, you can edit its fields.
    /// You can then flush your changes to Google Cloud Storage using this method.
    /// ### Example
    /// ```
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::bucket::{Bucket, RetentionPolicy};
    /// # use cloud_storage::bucket::NewBucket;
    /// # let new_bucket = create::Bucket {
    /// #   name: "cloud-storage-rs-doc-3".to_string(),
    /// #    ..Default::default()
    /// # };
    /// # let _ = Bucket::create(&new_bucket).await?;
    ///
    /// let mut bucket = Bucket::read("cloud-storage-rs-doc-3").await?;
    /// bucket.retention_policy = Some(RetentionPolicy {
    ///     retention_period: 50,
    ///     effective_time: time::OffsetDateTime::now_utc() + std::time::Duration::from_secs(50),
    ///     is_locked: Some(false),
    /// });
    /// bucket.update().await?;
    /// # bucket.delete().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(&self) -> Result<Self, Error> {
        crate::CLOUD_CLIENT.bucket().update(self).await
    }

    /// The synchronous equivalent of `Bucket::update`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn update_sync(&self) -> Result<Self, Error> {
        crate::runtime()?.block_on(self.update())
    }

    /// Delete an existing `Bucket`. This permanently removes a bucket from Google Cloud Storage.
    /// An error is returned when you don't have sufficient permissions, or when the
    /// `retention_policy` prevents you from deleting your Bucket.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Bucket;
    /// # use cloud_storage::bucket::NewBucket;
    /// # let new_bucket = create::Bucket {
    /// #   name: "unnecessary-bucket".to_string(),
    /// #    ..Default::default()
    /// # };
    /// # let _ = Bucket::create(&new_bucket).await?;
    ///
    /// let bucket = Bucket::read("unnecessary-bucket").await?;
    /// bucket.delete().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(self) -> Result<(), Error> {
        crate::CLOUD_CLIENT.bucket().delete(self).await
    }

    /// The synchronous equivalent of `Bucket::delete`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn delete_sync(self) -> Result<(), Error> {
        crate::runtime()?.block_on(self.delete())
    }

    /// Returns the [IAM Policy](https://cloud.google.com/iam/docs/) for this bucket.
    /// ### Example
    /// ```
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Bucket;
    /// # use cloud_storage::bucket::NewBucket;
    /// # let new_bucket = create::Bucket {
    /// #   name: "cloud-storage-rs-doc-4".to_string(),
    /// #    ..Default::default()
    /// # };
    /// # let _ = Bucket::create(&new_bucket).await?;
    ///
    /// let bucket = Bucket::read("cloud-storage-rs-doc-4").await?;
    /// let policy = bucket.get_iam_policy().await?;
    /// # bucket.delete().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_iam_policy(&self) -> Result<IamPolicy, Error> {
        crate::CLOUD_CLIENT.bucket().get_iam_policy(self).await
    }

    /// The synchronous equivalent of `Bucket::get_iam_policy`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn get_iam_policy_sync(&self) -> Result<IamPolicy, Error> {
        crate::runtime()?.block_on(self.get_iam_policy())
    }

    /// Updates the [IAM Policy](https://cloud.google.com/iam/docs/) for this bucket.
    /// ### Example
    /// ```
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Bucket;
    /// use cloud_storage::bucket::{IamPolicy, Binding, IamRole, StandardIamRole, Entity};
    /// # use cloud_storage::bucket::NewBucket;
    /// # let new_bucket = create::Bucket {
    /// #   name: "cloud-storage-rs-doc-5".to_string(),
    /// #    ..Default::default()
    /// # };
    /// # let _ = Bucket::create(&new_bucket).await?;
    ///
    /// let bucket = Bucket::read("cloud-storage-rs-doc-5").await?;
    /// let iam_policy = IamPolicy {
    ///     version: 1,
    ///     bindings: vec![
    ///         Binding {
    ///             role: IamRole::Standard(StandardIamRole::ObjectViewer),
    ///             members: vec!["allUsers".to_string()],
    ///             condition: None,
    ///         }
    ///     ],
    ///     ..Default::default()
    /// };
    /// let policy = bucket.set_iam_policy(&iam_policy).await?;
    /// # bucket.delete().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_iam_policy(&self, iam: &IamPolicy) -> Result<IamPolicy, Error> {
        crate::CLOUD_CLIENT.bucket().set_iam_policy(self, iam).await
    }

    /// The synchronous equivalent of `Bucket::set_iam_policy`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn set_iam_policy_sync(&self, iam: &IamPolicy) -> Result<IamPolicy, Error> {
        crate::runtime()?.block_on(self.set_iam_policy(iam))
    }

    /// Checks whether the user provided in the service account has this permission.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Bucket;
    ///
    /// let bucket = Bucket::read("my_bucket").await?;
    /// bucket.test_iam_permission("storage.buckets.get").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn test_iam_permission(&self, permission: &str) -> Result<TestIamPermission, Error> {
        crate::CLOUD_CLIENT
            .bucket()
            .test_iam_permission(self, permission)
            .await
    }

    /// The synchronous equivalent of `Bucket::test_iam_policy`.
    ///
    /// ### Features
    /// This function requires that the feature flag `sync` is enabled in `Cargo.toml`.
    #[cfg(feature = "sync")]
    pub fn test_iam_permission_sync(&self, permission: &str) -> Result<TestIamPermission, Error> {
        crate::runtime()?.block_on(self.test_iam_permission(permission))
    }
}


#[cfg(test)]
mod tests {
    use crate::{models::{create, Entity, Role, IamConfiguration, UniformBucketLevelAccess, RetentionPolicy, StandardIamRole, IamPolicy, Binding, IamRole}, Bucket};

    #[tokio::test]
    async fn create() -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "dotenv")]
        dotenv::dotenv().unwrap();
        let base_name = std::env::var("TEST_BUCKET")?;
        // use a more complex bucket in this test.
        let new_bucket = create::Bucket {
            name: format!("{}-test-create", base_name),
            default_event_based_hold: Some(true),
            acl: Some(vec![create::BucketAccessControl {
                entity: Entity::AllUsers,
                role: Role::Reader,
            }]),
            default_object_acl: Some(vec![create::DefaultObjectAccessControl {
                entity: Entity::AllUsers,
                role: Role::Reader,
            }]),
            iam_configuration: Some(IamConfiguration {
                uniform_bucket_level_access: UniformBucketLevelAccess {
                    enabled: false,
                    locked_time: None,
                },
            }),
            ..Default::default()
        };
        let bucket = Bucket::create(&new_bucket).await?;
        bucket.delete().await?;
        Ok(())
    }

    #[tokio::test]
    async fn list() -> Result<(), Box<dyn std::error::Error>> {
        Bucket::list().await?;
        Ok(())
    }

    #[tokio::test]
    async fn update() -> Result<(), Box<dyn std::error::Error>> {
        let mut bucket = crate::global_client::create_test_bucket("test-update").await;
        bucket.retention_policy = Some(RetentionPolicy {
            retention_period: 50,
            effective_time: time::OffsetDateTime::now_utc() + std::time::Duration::from_secs(50),
            is_locked: Some(false),
        });
        bucket.update().await?;
        let updated = Bucket::read(&bucket.name).await?;
        assert_eq!(updated.retention_policy.unwrap().retention_period, 50);
        bucket.delete().await?;
        Ok(())
    }

    // used a lot throughout the other tests, but included for completeness
    #[tokio::test]
    async fn delete() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::global_client::create_test_bucket("test-delete").await;
        bucket.delete().await?;
        Ok(())
    }

    #[tokio::test]
    async fn get_iam_policy() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::global_client::create_test_bucket("test-get-iam-policy").await;
        bucket.get_iam_policy().await?;
        bucket.delete().await?;
        Ok(())
    }

    #[tokio::test]
    async fn set_iam_policy() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::global_client::create_test_bucket("test-set-iam-policy").await;
        let iam_policy = IamPolicy {
            bindings: vec![Binding {
                role: IamRole::Standard(StandardIamRole::ObjectViewer),
                members: vec!["allUsers".to_string()],
                condition: None,
            }],
            ..Default::default()
        };
        bucket.set_iam_policy(&iam_policy).await?;
        assert_eq!(bucket.get_iam_policy().await?.bindings, iam_policy.bindings);
        bucket.delete().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_iam_permission() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::global_client::create_test_bucket("test-test-ia-permission").await;
        bucket.test_iam_permission("storage.buckets.get").await?;
        bucket.delete().await?;
        Ok(())
    }

    #[cfg(feature = "sync")]
    mod sync {
        use super::*;

        #[test]
        fn create() -> Result<(), Box<dyn std::error::Error>> {
            #[cfg(feature = "dotenv")]
            dotenv::dotenv().unwrap();
            let base_name = std::env::var("TEST_BUCKET")?;
            // use a more complex bucket in this test.
            let new_bucket = create::Bucket {
                name: format!("{}-test-create", base_name),
                default_event_based_hold: Some(true),
                acl: Some(vec![create::BucketAccessControl {
                    entity: Entity::AllUsers,
                    role: Role::Reader,
                }]),
                default_object_acl: Some(vec![create::DefaultObjectAccessControl {
                    entity: Entity::AllUsers,
                    role: Role::Reader,
                }]),
                iam_configuration: Some(IamConfiguration {
                    uniform_bucket_level_access: UniformBucketLevelAccess {
                        enabled: false,
                        locked_time: None,
                    },
                }),
                ..Default::default()
            };
            let bucket = Bucket::create_sync(&new_bucket)?;
            bucket.delete_sync()?;
            Ok(())
        }

        #[test]
        fn list() -> Result<(), Box<dyn std::error::Error>> {
            Bucket::list_sync()?;
            Ok(())
        }

        #[test]
        fn read() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::global_client::create_test_bucket_sync("test-read");
            let also_bucket = Bucket::read_sync(&bucket.name)?;
            assert_eq!(bucket, also_bucket);
            bucket.delete_sync()?;
            assert!(also_bucket.delete_sync().is_err());
            Ok(())
        }

        #[test]
        fn update() -> Result<(), Box<dyn std::error::Error>> {
            let mut bucket = crate::global_client::create_test_bucket_sync("test-update");
            bucket.retention_policy = Some(RetentionPolicy {
                retention_period: 50,
                effective_time: time::OffsetDateTime::now_utc() + std::time::Duration::from_secs(50),
                is_locked: Some(false),
            });
            bucket.update_sync()?;
            let updated = Bucket::read_sync(&bucket.name)?;
            assert_eq!(updated.retention_policy.unwrap().retention_period, 50);
            bucket.delete_sync()?;
            Ok(())
        }

        // used a lot throughout the other tests, but included for completeness
        #[test]
        fn delete() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::global_client::create_test_bucket_sync("test-delete");
            bucket.delete_sync()?;
            Ok(())
        }

        #[test]
        fn get_iam_policy() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::global_client::create_test_bucket_sync("test-get-iam-policy");
            bucket.get_iam_policy_sync()?;
            bucket.delete_sync()?;
            Ok(())
        }

        #[test]
        fn set_iam_policy() -> Result<(), Box<dyn std::error::Error>> {
            // use crate::resources::iam_policy::{Binding, IamRole, StandardIamRole};

            let bucket = crate::global_client::create_test_bucket_sync("test-set-iam-policy");
            let iam_policy = IamPolicy {
                bindings: vec![Binding {
                    role: IamRole::Standard(StandardIamRole::ObjectViewer),
                    members: vec!["allUsers".to_string()],
                    condition: None,
                }],
                ..Default::default()
            };
            bucket.set_iam_policy_sync(&iam_policy)?;
            assert_eq!(bucket.get_iam_policy_sync()?.bindings, iam_policy.bindings);
            bucket.delete_sync()?;
            Ok(())
        }

        #[test]
        fn test_iam_permission() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::global_client::create_test_bucket_sync("test-test-ia-permission");
            bucket.test_iam_permission_sync("storage.buckets.get")?;
            bucket.delete_sync()?;
            Ok(())
        }
    }
}
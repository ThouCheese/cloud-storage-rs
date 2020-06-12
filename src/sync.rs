use crate::error::{Error, GoogleResponse};
use crate::resources::bucket::{Bucket, NewBucket, IamPolicy, TestIamPermission};
use crate::resources::common::ListResponse;

/// TODO
#[derive(Debug, Clone)]
pub struct Client {
    reqwest: reqwest::blocking::Client,
}

impl Client {
    /// TODO
    pub fn new() -> Self {
        Self {
            reqwest: reqwest::blocking::Client::new(),
        }
    }

    /// Creates a new `Bucket`. There are many options that you can provide for creating a new
    /// bucket, so the `NewBucket` resource contains all of them. Note that `NewBucket` implements
    /// `Default`, so you don't have to specify the fields you're not using. And error is returned
    /// if that bucket name is already taken.
    /// ### Example
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::bucket::{Bucket, NewBucket};
    /// use cloud_storage::bucket::{Location, MultiRegion};
    ///
    /// let new_bucket = NewBucket {
    ///    name: "cloud-storage-rs-doc-1".to_string(), // this is the only mandatory field
    ///    location: Location::Multi(MultiRegion::Eu),
    ///    ..Default::default()
    /// };
    /// let client = cloud_storage::sync::Client::new();
    /// let bucket = client.create_bucket(&new_bucket)?;
    /// # client.delete_bucket(bucket);
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_bucket(&self, new_bucket: &NewBucket) -> Result<Bucket, Error> {
        let url = format!("{}/b/", crate::BASE_URL);
        let project = crate::SERVICE_ACCOUNT.project_id.clone();
        let query = [("project", project)];
        let result: GoogleResponse<Bucket> = self.reqwest
            .post(&url)
            .headers(crate::get_headers()?)
            .query(&query)
            .json(new_bucket)
            .send()?
            .json()?;
        match result {
            GoogleResponse::Success(s) => Ok(s),
            GoogleResponse::Error(e) => Err(e.into()),
        }
    }

    /// Returns a single `Bucket` by its name. If the Bucket does not exist, an error is returned.
    /// ### Example
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = cloud_storage::sync::Client::new();
    /// # use cloud_storage::bucket::NewBucket;
    /// # let new_bucket = NewBucket {
    /// #   name: "cloud-storage-rs-doc-2".to_string(),
    /// #    ..Default::default()
    /// # };
    /// # let _ = client.create_bucket(&new_bucket)?;
    ///
    /// let bucket = client.read_bucket("cloud-storage-rs-doc-2")?;
    /// # client.delete_bucket(bucket)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read_bucket(&self, name: &str) -> Result<Bucket, Error> {
        let url = format!("{}/b/{}", crate::BASE_URL, name);
        let result: GoogleResponse<Bucket> = self.reqwest
            .get(&url)
            .headers(crate::get_headers()?)
            .send()?
            .json()?;
        match result {
            GoogleResponse::Success(s) => Ok(s),
            GoogleResponse::Error(e) => Err(e.into()),
        }
    }

    /// Returns all `Bucket`s within this project.
    /// ### Example
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = cloud_storage::sync::Client::new();
    /// let buckets = client.list_buckets()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_buckets(&self) -> Result<Vec<Bucket>, Error> {
        let url = format!("{}/b/", crate::BASE_URL);
        let project = crate::SERVICE_ACCOUNT.project_id.clone();
        let query = [("project", project)];
        let result: GoogleResponse<ListResponse<Bucket>> = self.reqwest
            .get(&url)
            .headers(crate::get_headers()?)
            .query(&query)
            .send()?
            .json()?;
        match result {
            GoogleResponse::Success(s) => Ok(s.items),
            GoogleResponse::Error(e) => Err(e.into()),
        }
    }


    /// Update an existing `Bucket`. If you declare you bucket as mutable, you can edit its fields.
    /// You can then flush your changes to Google Cloud Storage using this method.
    /// ### Example
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = cloud_storage::sync::Client::new();
    /// use cloud_storage::bucket::RetentionPolicy;
    /// # use cloud_storage::bucket::NewBucket;
    /// # let new_bucket = NewBucket {
    /// #   name: "cloud-storage-rs-doc-3".to_string(),
    /// #    ..Default::default()
    /// # };
    /// # let _ = client.create_bucket(&new_bucket)?;
    ///
    /// let mut bucket = client.read_bucket("cloud-storage-rs-doc-3")?;
    /// bucket.retention_policy = Some(RetentionPolicy {
    ///     retention_period: 50,
    ///     effective_time: chrono::Utc::now() + chrono::Duration::seconds(50),
    ///     is_locked: Some(false),
    /// });
    /// client.update_bucket(&bucket)?;
    /// # client.delete_bucket(bucket)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn update_bucket(&self, bucket: &Bucket) -> Result<Bucket, Error> {
        let url = format!("{}/b/{}", crate::BASE_URL, bucket.name);
        let result: GoogleResponse<Bucket> = self.reqwest
            .put(&url)
            .headers(crate::get_headers()?)
            .json(bucket)
            .send()?
            .json()?;
        match result {
            GoogleResponse::Success(s) => Ok(s),
            GoogleResponse::Error(e) => Err(e.into()),
        }
    }

    /// Delete an existing `Bucket`. This permanently removes a bucket from Google Cloud Storage.
    /// An error is returned when you don't have sufficient permissions, or when the
    /// `retention_policy` prevents you from deleting your Bucket.
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = cloud_storage::sync::Client::new();
    /// # use cloud_storage::bucket::NewBucket;
    /// # let new_bucket = NewBucket {
    /// #   name: "unnecessary-bucket".to_string(),
    /// #    ..Default::default()
    /// # };
    /// # let _ = client.create_bucket(&new_bucket)?;
    ///
    /// let bucket = client.read_bucket("unnecessary-bucket")?;
    /// client.delete_bucket(bucket)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete_bucket(&self, bucket: Bucket) -> Result<(), Error> {
        let url = format!("{}/b/{}", crate::BASE_URL, bucket.name);
        let response = self.reqwest.delete(&url).headers(crate::get_headers()?).send()?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::Google(response.json()?))
        }
    }

    /// Returns the [IAM Policy](https://cloud.google.com/iam/docs/) for this bucket.
    /// ### Example
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = cloud_storage::sync::Client::new();
    /// # use cloud_storage::bucket::NewBucket;
    /// # let new_bucket = NewBucket {
    /// #   name: "cloud-storage-rs-doc-4".to_string(),
    /// #    ..Default::default()
    /// # };
    /// # let _ = client.create_bucket(&new_bucket)?;
    ///
    /// let bucket = client.read_bucket("cloud-storage-rs-doc-4")?;
    /// let policy = client.get_bucket_iam_policy(&bucket)?;
    /// # client.delete_bucket(bucket)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_bucket_iam_policy(&self, bucket: &Bucket) -> Result<IamPolicy, Error> {
        let url = format!("{}/b/{}/iam", crate::BASE_URL, bucket.name);
        let result: GoogleResponse<IamPolicy> = self.reqwest
            .get(&url)
            .headers(crate::get_headers()?)
            .send()?
            .json()?;
        match result {
            GoogleResponse::Success(s) => Ok(s),
            GoogleResponse::Error(e) => Err(e.into()),
        }
    }

    /// Updates the [IAM Policy](https://cloud.google.com/iam/docs/) for this bucket.
    /// ### Example
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::bucket::{IamPolicy, Binding, IamRole, StandardIamRole, Entity};
    /// let client = cloud_storage::sync::Client::new();
    /// # use cloud_storage::bucket::NewBucket;
    /// # let new_bucket = NewBucket {
    /// #   name: "cloud-storage-rs-doc-5".to_string(),
    /// #    ..Default::default()
    /// # };
    /// # let _ = client.create_bucket(&new_bucket)?;
    ///
    /// let bucket = client.read_bucket("cloud-storage-rs-doc-5")?;
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
    /// let policy = client.set_bucket_iam_policy(&bucket, &iam_policy)?;
    /// # client.delete_bucket(bucket)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_bucket_iam_policy(&self, bucket: &Bucket, iam: &IamPolicy) -> Result<IamPolicy, Error> {
        let url = format!("{}/b/{}/iam", crate::BASE_URL, bucket.name);
        let result: GoogleResponse<IamPolicy> = self.reqwest
            .put(&url)
            .headers(crate::get_headers()?)
            .json(iam)
            .send()?
            .json()?;
        match result {
            GoogleResponse::Success(s) => Ok(s),
            GoogleResponse::Error(e) => Err(e.into()),
        }
    }

    /// Checks whether the user provided in the service account has this permission.
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = cloud_storage::sync::Client::new();
    /// let bucket = client.read_bucket("my-bucket")?;
    /// client.test_bucket_iam_permission(&bucket, "storage.buckets.get")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn test_bucket_iam_permission(&self, bucket: &Bucket, permission: &str) -> Result<TestIamPermission, Error> {
        if permission == "storage.buckets.list" || permission == "storage.buckets.create" {
            return Err(Error::new(
                "tested permission must not be `storage.buckets.list` or `storage.buckets.create`",
            ));
        }
        let url = format!("{}/b/{}/iam/testPermissions", crate::BASE_URL, bucket.name);
        let result: GoogleResponse<TestIamPermission> = self.reqwest
            .get(&url)
            .headers(crate::get_headers()?)
            .query(&[("permissions", permission)])
            .send()?
            .json()?;
        match result {
            GoogleResponse::Success(s) => Ok(s),
            GoogleResponse::Error(e) => Err(e.into()),
        }
    }

    fn _lock_bucket_retention_policy() {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::common::{Role, Entity};
    use crate::resources::bucket::{IamRole, StandardIamRole, IamConfiguration, UniformBucketLevelAccess, RetentionPolicy, IamPolicy, Binding};
    use crate::resources::bucket_access_control::NewBucketAccessControl;
    use crate::resources::default_object_access_control::NewDefaultObjectAccessControl;

    #[test]
    fn create() -> Result<(), Box<dyn std::error::Error>> {
        dotenv::dotenv().ok();
        let base_name = std::env::var("TEST_BUCKET")?;
        // use a more complex bucket in this test.
        let new_bucket = NewBucket {
            name: format!("{}-test-create", base_name),
            default_event_based_hold: Some(true),
            acl: Some(vec![NewBucketAccessControl {
                entity: Entity::AllUsers,
                role: Role::Reader,
            }]),
            default_object_acl: Some(vec![NewDefaultObjectAccessControl {
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
        let client = Client::new();
        let bucket = client.create_bucket(&new_bucket)?;
        client.delete_bucket(bucket)?;
        Ok(())
    }

    #[test]
    fn list() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();
        client.list_buckets()?;
        Ok(())
    }

    #[test]
    fn read() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();
        let bucket = crate::create_test_bucket("test-read");
        let also_bucket = client.read_bucket(&bucket.name)?;
        assert_eq!(bucket, also_bucket);
        client.delete_bucket(bucket)?;
        assert!(client.delete_bucket(also_bucket).is_err());
        Ok(())
    }

    #[test]
    fn update() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();
        let mut bucket = crate::create_test_bucket("test-update");
        bucket.retention_policy = Some(RetentionPolicy {
            retention_period: 50,
            effective_time: chrono::Utc::now() + chrono::Duration::seconds(50),
            is_locked: Some(false),
        });
        client.update_bucket(&bucket)?;
        let updated = client.read_bucket(&bucket.name)?;
        assert_eq!(updated.retention_policy.unwrap().retention_period, 50);
        client.delete_bucket(bucket)?;
        Ok(())
    }

    // used a lot throughout the other tests, but included for completeness
    #[test]
    fn delete() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();
        let bucket = crate::create_test_bucket("test-delete");
        client.delete_bucket(bucket)?;
        Ok(())
    }

    #[test]
    fn get_iam_policy() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();
        let bucket = crate::create_test_bucket("test-get-iam-policy");
        client.get_bucket_iam_policy(&bucket)?;
        client.delete_bucket(bucket)?;
        Ok(())
    }

    #[test]
    fn set_iam_policy() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();
        let bucket = crate::create_test_bucket("test-set-iam-policy");
        let iam_policy = IamPolicy {
            bindings: vec![Binding {
                role: IamRole::Standard(StandardIamRole::ObjectViewer),
                members: vec!["allUsers".to_string()],
                condition: None,
            }],
            ..Default::default()
        };
        client.set_bucket_iam_policy(&bucket, &iam_policy)?;
        assert_eq!(
            client.get_bucket_iam_policy(&bucket)?.bindings,
            iam_policy.bindings
        );
        client.delete_bucket(bucket)?;
        Ok(())
    }

    #[test]
    fn test_iam_permission() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();
        let bucket = crate::create_test_bucket("test-test-ia-permission");
        client.test_bucket_iam_permission(&bucket, "storage.buckets.get")?;
        client.delete_bucket(bucket)?;
        Ok(())
    }
}

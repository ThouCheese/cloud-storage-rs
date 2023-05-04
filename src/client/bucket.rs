use crate::{models::{create, ListResponse, IamPolicy, TestIamPermission}, Bucket, Error};


/// Operations on [`Bucket`]()s.
#[derive(Debug)]
pub struct BucketClient<'a> {
    pub(crate) client: &'a super::CloudStorageClient,
    pub(crate) bucket_url: String,
    pub(crate) project_id: String,
}

impl<'a> BucketClient<'a> {
    /// Creates a new `Bucket`. There are many options that you can provide for creating a new
    /// bucket, so the `create::Bucket` resource contains all of them. Note that `create::Bucket` implements
    /// `Default`, so you don't have to specify the fields you're not using. And error is returned
    /// if that bucket name is already taken.
    /// ### Example
    /// ```
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::CloudStorageClient;
    /// # use cloud_storage::models::{Bucket, create};
    /// # use cloud_storage::models::{Location, MultiRegion};
    ///
    /// let client = CloudStorageClient::default();
    /// let new_bucket = create::Bucket {
    ///    name: "cloud-storage-rs-doc-1".to_string(), // this is the only mandatory field
    ///    location: Location::Multi(MultiRegion::Eu),
    ///    ..Default::default()
    /// };
    /// let bucket = client.bucket().create(&new_bucket).await?;
    /// # client.bucket().delete(bucket).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, new_bucket: &create::Bucket) -> Result<Bucket, Error> {
        let headers = self.client.get_headers().await?;
        let url = format!("{}/", self.bucket_url);
        let project = &self.project_id;
        let query = [("project", project)];
        let result: crate::models::Response<Bucket> = self.client.reqwest.post(&url).headers(headers).query(&query).json(new_bucket).send().await?.json().await?;
        Ok(result?)
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
    /// # use cloud_storage::CloudStorageClient;
    /// # use cloud_storage::Bucket;
    ///
    /// let client = CloudStorageClient::default();
    /// let buckets = client.bucket().list().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self) -> Result<Vec<Bucket>, Error> {
        let headers = self.client.get_headers().await?;
        let url = format!("{}/", self.bucket_url);
        let project = &self.project_id;
        let query = [("project", project)];
        let result: crate::models::Response<ListResponse<Bucket>> = self.client.reqwest.get(&url).headers(headers).query(&query).send().await?.json().await?;
        Ok(result?.items)
    }

    /// Returns a single `Bucket` by its name. If the Bucket does not exist, an error is returned.
    /// ### Example
    /// ```
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::CloudStorageClient;
    /// # use cloud_storage::Bucket;
    ///
    /// let client = CloudStorageClient::default();
    /// # use cloud_storage::models::create;
    /// # let new_bucket = create::Bucket {
    /// #   name: "cloud-storage-rs-doc-2".to_string(),
    /// #    ..Default::default()
    /// # };
    /// # let _ = client.bucket().create(&new_bucket).await?;
    ///
    /// let bucket = client.bucket().read("cloud-storage-rs-doc-2").await?;
    /// # client.bucket().delete(bucket).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn read(&self, name: &str) -> Result<Bucket, Error> {
        let headers = self.client.get_headers().await?;
        let url = format!("{}/{}", self.bucket_url, crate::percent_encode(name));
        let result: crate::models::Response<Bucket> = self.client.reqwest.get(&url).headers(headers).send().await?.json().await?;
        Ok(result?)
    }

    /// Update an existing `Bucket`. If you declare you bucket as mutable, you can edit its fields.
    /// You can then flush your changes to Google Cloud Storage using this method.
    /// ### Example
    /// ```
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::CloudStorageClient;
    /// # use cloud_storage::models::{Bucket, RetentionPolicy};
    ///
    /// let client = CloudStorageClient::default();
    /// # use cloud_storage::models::create;
    /// # let new_bucket = create::Bucket {
    /// #   name: "cloud-storage-rs-doc-3".to_string(),
    /// #    ..Default::default()
    /// # };
    /// # let _ = client.bucket().create(&new_bucket).await?;
    ///
    /// let mut bucket = client.bucket().read("cloud-storage-rs-doc-3").await?;
    /// bucket.retention_policy = Some(RetentionPolicy {
    ///     retention_period: 50,
    ///     effective_time: time::OffsetDateTime::now_utc() + std::time::Duration::from_secs(50),
    ///     is_locked: Some(false),
    /// });
    /// client.bucket().update(&bucket).await?;
    /// # client.bucket().delete(bucket).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(&self, bucket: &Bucket) -> Result<Bucket, Error> {
        let headers = self.client.get_headers().await?;
        let url = format!("{}/{}", self.bucket_url, crate::percent_encode(&bucket.name),);
        let result: crate::models::Response<Bucket> = self.client.reqwest.put(&url).headers(headers).json(bucket).send().await?.json().await?;
        Ok(result?)
    }

    /// Delete an existing `Bucket`. This permanently removes a bucket from Google Cloud Storage.
    /// An error is returned when you don't have sufficient permissions, or when the
    /// `retention_policy` prevents you from deleting your Bucket.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::CloudStorageClient;
    /// # use cloud_storage::Bucket;
    ///
    /// let client = CloudStorageClient::default();
    /// # use cloud_storage::models::create;
    /// # let new_bucket = create::Bucket {
    /// #   name: "unnecessary-bucket".to_string(),
    /// #    ..Default::default()
    /// # };
    /// # let _ = client.bucket().create(&new_bucket).await?;
    ///
    /// let bucket = client.bucket().read("unnecessary-bucket").await?;
    /// client.bucket().delete(bucket).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, bucket: Bucket) -> Result<(), Error> {
        let headers = self.client.get_headers().await?;
        let url = format!("{}/{}", self.bucket_url, crate::percent_encode(&bucket.name));
        let response = self.client.reqwest.delete(&url).headers(headers).send().await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(crate::Error::Google(response.json().await?))
        }
    }

    /// Returns the [IAM Policy](https://cloud.google.com/iam/docs/) for this bucket.
    /// ### Example
    /// ```
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::CloudStorageClient;
    /// # use cloud_storage::Bucket;
    ///
    /// let client = CloudStorageClient::default();
    /// # use cloud_storage::models::create;
    /// # let new_bucket = create::Bucket {
    /// #   name: "cloud-storage-rs-doc-4".to_string(),
    /// #    ..Default::default()
    /// # };
    /// # let _ = client.bucket().create(&new_bucket).await?;
    ///
    /// let bucket = client.bucket().read("cloud-storage-rs-doc-4").await?;
    /// let policy = client.bucket().get_iam_policy(&bucket).await?;
    /// # client.bucket().delete(bucket).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_iam_policy(&self, bucket: &Bucket) -> Result<IamPolicy, Error> {
        let headers = self.client.get_headers().await?;
        let url = format!("{}/{}/iam", self.bucket_url, crate::percent_encode(&bucket.name));
        let result: crate::models::Response<IamPolicy> = self.client.reqwest.get(&url).headers(headers).send().await?.json().await?;
        Ok(result?)
    }

    /// Updates the [IAM Policy](https://cloud.google.com/iam/docs/) for this bucket.
    /// ### Example
    /// ```
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::CloudStorageClient;
    /// # use cloud_storage::Bucket;
    /// # use cloud_storage::models::{IamPolicy, Binding, IamRole, StandardIamRole, Entity};
    ///
    /// let client = CloudStorageClient::default();
    /// # use cloud_storage::models::create;
    /// # let new_bucket = create::Bucket {
    /// #   name: "cloud-storage-rs-doc-5".to_string(),
    /// #    ..Default::default()
    /// # };
    /// # let _ = client.bucket().create(&new_bucket).await?;
    ///
    /// let bucket = client.bucket().read("cloud-storage-rs-doc-5").await?;
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
    /// let policy = client.bucket().set_iam_policy(&bucket, &iam_policy).await?;
    /// # client.bucket().delete(bucket).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_iam_policy(
        &self,
        bucket: &Bucket,
        iam: &IamPolicy,
    ) -> Result<IamPolicy, Error> {
        let headers = self.client.get_headers().await?;
        let url = format!("{}/{}/iam", self.bucket_url, crate::percent_encode(&bucket.name));
        let result: crate::models::Response<IamPolicy> = self.client.reqwest.put(&url).headers(headers).json(iam).send().await?.json().await?;
        Ok(result?)
    }

    /// Checks whether the user provided in the service account has this permission.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use cloud_storage::CloudStorageClient;
    /// let cloud_storage_client = CloudStorageClient::default();
    /// let bucket_client = cloud_storage_client.bucket();
    /// let bucket = bucket_client.read("my_bucket").await?;
    /// bucket_client.test_iam_permission(&bucket, "storage.buckets.get").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn test_iam_permission(
        &self,
        bucket: &Bucket,
        permission: &str,
    ) -> Result<TestIamPermission, Error> {
        if permission == "storage.buckets.list" || permission == "storage.buckets.create" {
            return Err(crate::Error::new(
                "tested permission must not be `storage.buckets.list` or `storage.buckets.create`",
            ));
        }
        let url = format!(
            "{}/{}/iam/testPermissions",
            self.bucket_url,
            crate::percent_encode(&bucket.name)
        );
        let headers = self.client.get_headers().await?;
        let result: crate::models::Response<TestIamPermission> = self.client.reqwest.get(&url).headers(headers).query(&[("permissions", permission)]).send().await?.json().await?;
        Ok(result?)
    }
}

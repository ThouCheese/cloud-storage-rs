use crate::{
    bucket_access_control::{BucketAccessControl, Entity, NewBucketAccessControl},
    error::GoogleResponse,
    object::percent_encode,
    resources::common::ListResponse,
};

/// Operations on [`BucketAccessControl`](BucketAccessControl)s.
pub struct BucketAccessControlClient<'a>(pub(super) &'a super::Client);

impl<'a> BucketAccessControlClient<'a> {
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
    /// use cloud_storage::Client;
    /// use cloud_storage::bucket_access_control::{BucketAccessControl, NewBucketAccessControl};
    /// use cloud_storage::bucket_access_control::{Role, Entity};
    ///
    /// let client = Client::default();
    /// let new_bucket_access_control = NewBucketAccessControl {
    ///     entity: Entity::AllUsers,
    ///     role: Role::Reader,
    /// };
    /// client.bucket_access_control().create("mybucket", &new_bucket_access_control).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(
        &self,
        bucket: &str,
        new_bucket_access_control: &NewBucketAccessControl,
    ) -> crate::Result<BucketAccessControl> {
        let url = dbg!(format!(
            "{}/b/{}/acl",
            crate::BASE_URL,
            percent_encode(bucket),
        ));
        let result: GoogleResponse<BucketAccessControl> = self
            .0
            .client
            .post(&url)
            .headers(self.0.get_headers().await?)
            .json(new_bucket_access_control)
            .send()
            .await?
            .json()
            .await?;
        match result {
            GoogleResponse::Success(s) => Ok(s),
            GoogleResponse::Error(e) => Err(e.into()),
        }
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
    /// use cloud_storage::Client;
    /// use cloud_storage::bucket_access_control::BucketAccessControl;
    ///
    /// let client = Client::default();
    /// let acls = client.bucket_access_control().list("mybucket").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self, bucket: &str) -> crate::Result<Vec<BucketAccessControl>> {
        let url = format!("{}/b/{}/acl", crate::BASE_URL, percent_encode(bucket));
        let result: GoogleResponse<ListResponse<BucketAccessControl>> = self
            .0
            .client
            .get(&url)
            .headers(self.0.get_headers().await?)
            .send()
            .await?
            .json()
            .await?;
        match result {
            GoogleResponse::Success(s) => Ok(s.items),
            GoogleResponse::Error(e) => Err(e.into()),
        }
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
    /// use cloud_storage::Client;
    /// use cloud_storage::bucket_access_control::{BucketAccessControl, Entity};
    ///
    /// let client = Client::default();
    /// let controls = client.bucket_access_control().read("mybucket", &Entity::AllUsers).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn read(&self, bucket: &str, entity: &Entity) -> crate::Result<BucketAccessControl> {
        let url = format!(
            "{}/b/{}/acl/{}",
            crate::BASE_URL,
            percent_encode(bucket),
            percent_encode(&entity.to_string())
        );
        let result: GoogleResponse<BucketAccessControl> = self
            .0
            .client
            .get(&url)
            .headers(self.0.get_headers().await?)
            .send()
            .await?
            .json()
            .await?;
        match result {
            GoogleResponse::Success(s) => Ok(s),
            GoogleResponse::Error(e) => Err(e.into()),
        }
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
    /// use cloud_storage::Client;
    /// use cloud_storage::bucket_access_control::{BucketAccessControl, Entity};
    ///
    /// let client = Client::default();
    /// let mut acl = client.bucket_access_control().read("mybucket", &Entity::AllUsers).await?;
    /// acl.entity = Entity::AllAuthenticatedUsers;
    /// client.bucket_access_control().update(&acl).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(
        &self,
        bucket_access_control: &BucketAccessControl,
    ) -> crate::Result<BucketAccessControl> {
        let url = format!(
            "{}/b/{}/acl/{}",
            crate::BASE_URL,
            percent_encode(&bucket_access_control.bucket),
            percent_encode(&bucket_access_control.entity.to_string()),
        );
        let result: GoogleResponse<BucketAccessControl> = self
            .0
            .client
            .put(&url)
            .headers(self.0.get_headers().await?)
            .json(bucket_access_control)
            .send()
            .await?
            .json()
            .await?;
        match result {
            GoogleResponse::Success(s) => Ok(s),
            GoogleResponse::Error(e) => Err(e.into()),
        }
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
    /// use cloud_storage::Client;
    /// use cloud_storage::bucket_access_control::{BucketAccessControl, Entity};
    ///
    /// let client = Client::default();
    /// let controls = client.bucket_access_control().read("mybucket", &Entity::AllUsers).await?;
    /// client.bucket_access_control().delete(controls).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, bucket_access_control: BucketAccessControl) -> crate::Result<()> {
        let url = format!(
            "{}/b/{}/acl/{}",
            crate::BASE_URL,
            percent_encode(&bucket_access_control.bucket),
            percent_encode(&bucket_access_control.entity.to_string()),
        );
        let response = self
            .0
            .client
            .delete(&url)
            .headers(self.0.get_headers().await?)
            .send()
            .await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(crate::Error::Google(response.json().await?))
        }
    }
}

use crate::{
    bucket_access_control::Entity,
    default_object_access_control::{DefaultObjectAccessControl, NewDefaultObjectAccessControl},
    error::GoogleResponse,
    object::percent_encode,
    resources::common::ListResponse,
};

/// Operations on [`DefaultObjectAccessControl`](DefaultObjectAccessControl)s.
#[derive(Debug)]
pub struct DefaultObjectAccessControlClient<'a>(pub(super) &'a super::Client);

impl<'a> DefaultObjectAccessControlClient<'a> {
    /// Create a new `DefaultObjectAccessControl` entry on the specified bucket.
    /// ### Important
    /// Important: This method fails with a `400 Bad Request` response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Client;
    /// use cloud_storage::default_object_access_control::{
    ///     DefaultObjectAccessControl, NewDefaultObjectAccessControl, Role, Entity,
    /// };
    ///
    /// let client = Client::default();
    /// let new_acl = NewDefaultObjectAccessControl {
    ///     entity: Entity::AllAuthenticatedUsers,
    ///     role: Role::Reader,
    /// };
    /// let default_acl = client.default_object_access_control().create("mybucket", &new_acl).await?;
    /// # client.default_object_access_control().delete(default_acl).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(
        &self,
        bucket: &str,
        new_acl: &NewDefaultObjectAccessControl,
    ) -> crate::Result<DefaultObjectAccessControl> {
        let url = format!(
            "{}/b/{}/defaultObjectAcl",
            crate::BASE_URL,
            percent_encode(bucket)
        );
        let result: GoogleResponse<DefaultObjectAccessControl> = self
            .0
            .client
            .post(&url)
            .headers(self.0.get_headers().await?)
            .json(new_acl)
            .send()
            .await?
            .json()
            .await?;
        match result {
            GoogleResponse::Success(mut s) => {
                s.bucket = bucket.to_string();
                Ok(s)
            }
            GoogleResponse::Error(e) => Err(e.into()),
        }
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
    /// use cloud_storage::Client;
    /// use cloud_storage::default_object_access_control::DefaultObjectAccessControl;
    ///
    /// let client = Client::default();
    /// let default_acls = client.default_object_access_control().list("mybucket").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self, bucket: &str) -> crate::Result<Vec<DefaultObjectAccessControl>> {
        let url = format!(
            "{}/b/{}/defaultObjectAcl",
            crate::BASE_URL,
            percent_encode(bucket)
        );
        let result: GoogleResponse<ListResponse<DefaultObjectAccessControl>> = self
            .0
            .client
            .get(&url)
            .headers(self.0.get_headers().await?)
            .send()
            .await?
            .json()
            .await?;
        match result {
            GoogleResponse::Success(s) => Ok(s
                .items
                .into_iter()
                .map(|item| DefaultObjectAccessControl {
                    bucket: bucket.to_string(),
                    ..item
                })
                .collect()),
            GoogleResponse::Error(e) => Err(e.into()),
        }
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
    /// use cloud_storage::Client;
    /// use cloud_storage::default_object_access_control::{DefaultObjectAccessControl, Entity};
    ///
    /// let client = Client::default();
    /// let default_acl = client.default_object_access_control().read("mybucket", &Entity::AllUsers).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn read(
        &self,
        bucket: &str,
        entity: &Entity,
    ) -> crate::Result<DefaultObjectAccessControl> {
        let url = format!(
            "{}/b/{}/defaultObjectAcl/{}",
            crate::BASE_URL,
            percent_encode(bucket),
            percent_encode(&entity.to_string()),
        );
        let result: GoogleResponse<DefaultObjectAccessControl> = self
            .0
            .client
            .get(&url)
            .headers(self.0.get_headers().await?)
            .send()
            .await?
            .json()
            .await?;
        match result {
            GoogleResponse::Success(mut s) => {
                s.bucket = bucket.to_string();
                Ok(s)
            }
            GoogleResponse::Error(e) => Err(e.into()),
        }
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
    /// use cloud_storage::Client;
    /// use cloud_storage::default_object_access_control::{DefaultObjectAccessControl, Entity};
    ///
    /// let client = Client::default();
    /// let mut default_acl = client.default_object_access_control().read("my_bucket", &Entity::AllUsers).await?;
    /// default_acl.entity = Entity::AllAuthenticatedUsers;
    /// client.default_object_access_control().update(&default_acl).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(
        &self,
        default_object_access_control: &DefaultObjectAccessControl,
    ) -> crate::Result<DefaultObjectAccessControl> {
        let url = format!(
            "{}/b/{}/defaultObjectAcl/{}",
            crate::BASE_URL,
            percent_encode(&default_object_access_control.bucket),
            percent_encode(&default_object_access_control.entity.to_string()),
        );
        let result: GoogleResponse<DefaultObjectAccessControl> = self
            .0
            .client
            .put(&url)
            .headers(self.0.get_headers().await?)
            .json(default_object_access_control)
            .send()
            .await?
            .json()
            .await?;
        match result {
            GoogleResponse::Success(mut s) => {
                s.bucket = default_object_access_control.bucket.to_string();
                Ok(s)
            }
            GoogleResponse::Error(e) => Err(e.into()),
        }
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
    /// use cloud_storage::Client;
    /// use cloud_storage::default_object_access_control::{DefaultObjectAccessControl, Entity};
    ///
    /// let client = Client::default();
    /// let mut default_acl = client.default_object_access_control().read("my_bucket", &Entity::AllUsers).await?;
    /// client.default_object_access_control().delete(default_acl).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(
        &self,
        default_object_access_control: DefaultObjectAccessControl,
    ) -> Result<(), crate::Error> {
        let url = format!(
            "{}/b/{}/defaultObjectAcl/{}",
            crate::BASE_URL,
            percent_encode(&default_object_access_control.bucket),
            percent_encode(&default_object_access_control.entity.to_string()),
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

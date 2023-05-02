use crate::{models::{create, DefaultObjectAccessControl, ListResponse, Entity}, Error};


/// Operations on [`DefaultObjectAccessControl`](DefaultObjectAccessControl)s.
#[derive(Debug)]
pub struct DefaultObjectAccessControlClient<'a> {
    pub(crate) client: &'a super::client::Client,
    pub(crate) base_url: String,
    pub(crate) bucket: String,
}

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
    ///     DefaultObjectAccessControl, create::DefaultObjectAccessControl, Role, Entity,
    /// };
    ///
    /// let client = Client::default();
    /// let new_acl = create::DefaultObjectAccessControl {
    ///     entity: Entity::AllAuthenticatedUsers,
    ///     role: Role::Reader,
    /// };
    /// let default_acl = client.default_object_access_control("my_bucket").create(&new_acl).await?;
    /// # client.default_object_access_control().delete(default_acl).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(
        &self,
        new_acl: &create::DefaultObjectAccessControl,
    ) -> Result<DefaultObjectAccessControl, Error> {
        let headers = self.client.get_headers().await?;
        let url = format!(
            "{}",
            self.base_url
        );
        let result: crate::models::Response<DefaultObjectAccessControl> = self.client.reqwest
            .post(&url)
            .headers(headers)
            .json(new_acl)
            .send()
            .await?
            .json()
            .await?;
        match result {
            crate::models::Response::Success(mut s) => {
                // todo:
                // s.bucket = bucket.to_string();
                Ok(s)
            }
            crate::models::Response::Error(e) => Err(e.into()),
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
    /// let default_acls = client.default_object_access_control("my_bucket").list().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self) -> Result<Vec<DefaultObjectAccessControl>, Error> {
        let headers = self.client.get_headers().await?;
        let result: crate::models::Response<ListResponse<DefaultObjectAccessControl>> = self.client.reqwest.get(&self.base_url).headers(headers).send().await?.json().await?;
        match result {
            crate::models::Response::Success(s) => Ok(s.items.into_iter()
            .map(|item| DefaultObjectAccessControl {
                bucket: self.bucket.to_string(),
                ..item
            }).collect()),
            crate::models::Response::Error(e) => Err(e.into()),
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
    /// let default_acl = client.default_object_access_control("my_bucket").read(&Entity::AllUsers).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn read(
        &self,
        entity: &Entity,
    ) -> Result<DefaultObjectAccessControl, Error> {
        let headers = self.client.get_headers().await?;
        let url = format!(
            "{}/{}",
            self.base_url,
            crate::percent_encode(&entity.to_string()),
        );
        let result: crate::models::Response<DefaultObjectAccessControl> = self.client.reqwest
            .get(&url)
            .headers(headers)
            .send()
            .await?
            .json()
            .await?;
        match result {
            crate::models::Response::Success(mut s) => {
                s.bucket = self.bucket.to_string();
                Ok(s)
            }
            crate::models::Response::Error(e) => Err(e.into()),
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
    /// let mut default_acl = client.default_object_access_control("my_bucket").read(&Entity::AllUsers).await?;
    /// default_acl.entity = Entity::AllAuthenticatedUsers;
    /// client.default_object_access_control().update(&default_acl).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(
        &self,
        default_object_access_control: &DefaultObjectAccessControl,
    ) -> Result<DefaultObjectAccessControl, Error> {
        let headers = self.client.get_headers().await?;
        let url = format!(
            "{}/{}",
            self.base_url,
            crate::percent_encode(&default_object_access_control.entity.to_string()),
        );
        let result: crate::models::Response<DefaultObjectAccessControl> = self.client.reqwest.put(&url).headers(headers).json(default_object_access_control).send().await?.json().await?;
        match result {
            crate::models::Response::Success(mut s) => {
                s.bucket = default_object_access_control.bucket.to_string();
                Ok(s)
            }
            crate::models::Response::Error(e) => Err(e.into()),
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
    /// let mut default_acl = client.default_object_access_control("my_bucket").read(&Entity::AllUsers).await?;
    /// client.default_object_access_control().delete(default_acl).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(
        &self,
        default_object_access_control: DefaultObjectAccessControl,
    ) -> Result<(), crate::Error> {
        let headers = self.client.get_headers().await?;
        let url = format!(
            "{}/{}",
            self.base_url,
            crate::percent_encode(&default_object_access_control.entity.to_string()),
        );
        let response = self.client.reqwest
            .delete(&url)
            .headers(headers)
            .send()
            .await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(crate::Error::Google(response.json().await?))
        }
    }
}

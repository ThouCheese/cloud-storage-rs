use crate::{models::{create, ObjectAccessControl, ListResponse, Entity, Response}, Error};


/// Operations on [`ObjectAccessControl`](ObjectAccessControl)s.
#[derive(Debug)]
pub struct ObjectAccessControlClient<'a> {
    pub(crate) client: &'a super::CloudStorageClient,
    pub(crate) acl_url: String,
}

impl<'a> ObjectAccessControlClient<'a> {
    /// Creates a new ACL entry on the specified `object`.
    ///
    /// ### Important
    /// This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    pub async fn create(
        &self,
        new_object_access_control: &create::ObjectAccessControl,
    ) -> Result<ObjectAccessControl, Error> {
        let result: crate::models::Response<ObjectAccessControl> = self.client.reqwest
            .post(&self.acl_url)
            .headers(self.client.get_headers().await?)
            .json(new_object_access_control)
            .send()
            .await?
            .json()
            .await?;
        Ok(result.ok()?)
    }

    /// Retrieves `ACL` entries on the specified object.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    pub async fn list(
        &self
    ) -> Result<Vec<ObjectAccessControl>, Error> {
        let result = self.client.reqwest
            .get(&self.acl_url)
            .headers(self.client.get_headers().await?)
            .send()
            .await?
            .json::<Response<ListResponse<ObjectAccessControl>>>()
            .await?.ok()?;
        Ok(result.items)
    }

    /// Returns the `ACL` entry for the specified entity on the specified bucket.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    pub async fn read(
        &self,
        entity: &Entity,
    ) -> Result<ObjectAccessControl, Error> {
        let url = format!(
            "{}/{}",
            &self.acl_url,
            crate::percent_encode(&entity.to_string())
        );
        let result: crate::models::Response<ObjectAccessControl> = self.client.reqwest
            .get(&url)
            .headers(self.client.get_headers().await?)
            .send()
            .await?
            .json()
            .await?;
        Ok(result.ok()?)
    }

    /// Updates an ACL entry on the specified object.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    pub async fn update(
        &self,
        object_access_control: &ObjectAccessControl,
    ) -> Result<ObjectAccessControl, Error> {
        let url = format!(
            "{}/{}",
            &self.acl_url,
            crate::percent_encode(&object_access_control.entity.to_string()),
        );
        let result: crate::models::Response<ObjectAccessControl> = self.client.reqwest
            .put(&url)
            .headers(self.client.get_headers().await?)
            .json(object_access_control)
            .send()
            .await?
            .json()
            .await?;
        Ok(result.ok()?)
    }

    /// Permanently deletes the ACL entry for the specified entity on the specified object.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    pub async fn delete(&self, object_access_control: ObjectAccessControl) -> Result<(), Error> {
        let url = format!(
            "{}/{}",
            &self.acl_url,
            crate::percent_encode(&object_access_control.entity.to_string()),
        );
        let response = self.client.reqwest
            .delete(&url)
            .headers(self.client.get_headers().await?)
            .send()
            .await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(crate::Error::Google(response.json().await?))
        }
    }
}

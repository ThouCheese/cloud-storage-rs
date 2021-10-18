use crate::{
    bucket_access_control::Entity,
    error::GoogleResponse,
    object::percent_encode,
    object_access_control::{NewObjectAccessControl, ObjectAccessControl},
    resources::common::ListResponse,
};

/// Operations on [`ObjectAccessControl`](ObjectAccessControl)s.
#[derive(Debug)]
pub struct ObjectAccessControlClient<'a>(pub(super) &'a super::Client);

impl<'a> ObjectAccessControlClient<'a> {
    /// Creates a new ACL entry on the specified `object`.
    ///
    /// ### Important
    /// This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    pub async fn create(
        &self,
        bucket: &str,
        object: &str,
        new_object_access_control: &NewObjectAccessControl,
    ) -> crate::Result<ObjectAccessControl> {
        let url = format!(
            "{}/b/{}/o/{}/acl",
            crate::BASE_URL,
            percent_encode(bucket),
            percent_encode(object),
        );
        let result: GoogleResponse<ObjectAccessControl> = self
            .0
            .client
            .post(&url)
            .headers(self.0.get_headers().await?)
            .json(new_object_access_control)
            .send()
            .await?
            .json()
            .await?;
        match result {
            GoogleResponse::Success(s) => Ok(s),
            GoogleResponse::Error(e) => Err(e.into()),
        }
    }

    /// Retrieves `ACL` entries on the specified object.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    pub async fn list(
        &self,
        bucket: &str,
        object: &str,
    ) -> crate::Result<Vec<ObjectAccessControl>> {
        let url = format!(
            "{}/b/{}/o/{}/acl",
            crate::BASE_URL,
            percent_encode(bucket),
            percent_encode(object),
        );
        let result: GoogleResponse<ListResponse<ObjectAccessControl>> = self
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

    /// Returns the `ACL` entry for the specified entity on the specified bucket.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    pub async fn read(
        &self,
        bucket: &str,
        object: &str,
        entity: &Entity,
    ) -> crate::Result<ObjectAccessControl> {
        let url = format!(
            "{}/b/{}/o/{}/acl/{}",
            crate::BASE_URL,
            percent_encode(bucket),
            percent_encode(object),
            percent_encode(&entity.to_string())
        );
        let result: GoogleResponse<ObjectAccessControl> = self
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

    /// Updates an ACL entry on the specified object.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    pub async fn update(
        &self,
        object_access_control: &ObjectAccessControl,
    ) -> crate::Result<ObjectAccessControl> {
        let url = format!(
            "{}/b/{}/o/{}/acl/{}",
            crate::BASE_URL,
            percent_encode(&object_access_control.bucket),
            percent_encode(&object_access_control.object),
            percent_encode(&object_access_control.entity.to_string()),
        );
        let result: GoogleResponse<ObjectAccessControl> = self
            .0
            .client
            .put(&url)
            .headers(self.0.get_headers().await?)
            .json(object_access_control)
            .send()
            .await?
            .json()
            .await?;
        match result {
            GoogleResponse::Success(s) => Ok(s),
            GoogleResponse::Error(e) => Err(e.into()),
        }
    }

    /// Permanently deletes the ACL entry for the specified entity on the specified object.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    pub async fn delete(&self, object_access_control: ObjectAccessControl) -> crate::Result<()> {
        let url = format!(
            "{}/b/{}/o/{}/acl/{}",
            crate::BASE_URL,
            percent_encode(&object_access_control.bucket),
            percent_encode(&object_access_control.object),
            percent_encode(&object_access_control.entity.to_string()),
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

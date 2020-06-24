use crate::error::GoogleResponse;
use crate::resources::common::ListResponse;
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
    /// The kind of item this is. For bucket access control entries, this is always storage#bucketAccessControl.
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
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::bucket_access_control::{BucketAccessControl, NewBucketAccessControl};
    /// use cloud_storage::bucket_access_control::{Role, Entity};
    ///
    /// let new_bucket_access_control = NewBucketAccessControl {
    ///     entity: Entity::AllUsers,
    ///     role: Role::Reader,
    /// };
    /// BucketAccessControl::create("mybucket", &new_bucket_access_control)?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "sync")]
    pub fn create(
        bucket: &str,
        new_bucket_access_control: &NewBucketAccessControl,
    ) -> Result<Self, crate::Error> {
        let url = format!("{}/b/{}/acl", crate::BASE_URL, bucket);
        let client = reqwest::blocking::Client::new();
        let result: GoogleResponse<Self> = client
            .post(&url)
            .headers(crate::get_headers()?)
            .json(new_bucket_access_control)
            .send()?
            .json()?;
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
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::bucket_access_control::BucketAccessControl;
    ///
    /// let acls = BucketAccessControl::list("mybucket")?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "sync")]
    pub fn list(bucket: &str) -> Result<Vec<Self>, crate::Error> {
        let url = format!("{}/b/{}/acl", crate::BASE_URL, bucket);
        let client = reqwest::blocking::Client::new();
        let result: GoogleResponse<ListResponse<Self>> = client
            .get(&url)
            .headers(crate::get_headers()?)
            .send()?
            .json()?;
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
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::bucket_access_control::{BucketAccessControl, Entity};
    ///
    /// let controls = BucketAccessControl::read("mybucket", &Entity::AllUsers)?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "sync")]
    pub fn read(bucket: &str, entity: &Entity) -> Result<Self, crate::Error> {
        let url = format!("{}/b/{}/acl/{}", crate::BASE_URL, bucket, entity);
        let client = reqwest::blocking::Client::new();
        let result: GoogleResponse<Self> = client
            .get(&url)
            .headers(crate::get_headers()?)
            .send()?
            .json()?;
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
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::bucket_access_control::{BucketAccessControl, Entity};
    ///
    /// let mut acl = BucketAccessControl::read("mybucket", &Entity::AllUsers)?;
    /// acl.entity = Entity::AllAuthenticatedUsers;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "sync")]
    pub fn update(&self) -> Result<Self, crate::Error> {
        let url = format!("{}/b/{}/acl/{}", crate::BASE_URL, self.bucket, self.entity);
        let client = reqwest::blocking::Client::new();
        let result: GoogleResponse<Self> = client
            .put(&url)
            .headers(crate::get_headers()?)
            .json(self)
            .send()?
            .json()?;
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
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::bucket_access_control::{BucketAccessControl, Entity};
    ///
    /// let controls = BucketAccessControl::read("mybucket", &Entity::AllUsers)?;
    /// controls.delete()?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "sync")]
    pub fn delete(self) -> Result<(), crate::Error> {
        let url = format!("{}/b/{}/acl/{}", crate::BASE_URL, self.bucket, self.entity);
        let client = reqwest::blocking::Client::new();
        let response = client.delete(&url).headers(crate::get_headers()?).send()?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(crate::Error::Google(response.json()?))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "sync")]
    mod sync {
        use super::*;

        #[test]
        fn create() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::read_test_bucket();
            let new_bucket_access_control = NewBucketAccessControl {
                entity: Entity::AllUsers,
                role: Role::Reader,
            };
            BucketAccessControl::create(&bucket.name, &new_bucket_access_control)?;
            Ok(())
        }

        #[test]
        fn list() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::read_test_bucket();
            BucketAccessControl::list(&bucket.name)?;
            Ok(())
        }

        #[test]
        fn read() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::read_test_bucket();
            BucketAccessControl::read(&bucket.name, &Entity::AllUsers)?;
            Ok(())
        }

        #[test]
        fn update() -> Result<(), Box<dyn std::error::Error>> {
            // use a seperate bucket to prevent synchronization issues
            let bucket = crate::create_test_bucket("test-update-bucket-access-controls");
            let new_bucket_access_control = NewBucketAccessControl {
                entity: Entity::AllUsers,
                role: Role::Reader,
            };
            BucketAccessControl::create(&bucket.name, &new_bucket_access_control)?;
            let mut acl = BucketAccessControl::read(&bucket.name, &Entity::AllUsers)?;
            acl.entity = Entity::AllAuthenticatedUsers;
            acl.update()?;
            bucket.delete()?;
            Ok(())
        }

        #[test]
        fn delete() -> Result<(), Box<dyn std::error::Error>> {
            // use a seperate bucket to prevent synchronization issues
            let bucket = crate::create_test_bucket("test-delete-bucket-access-controls");
            let new_bucket_access_control = NewBucketAccessControl {
                entity: Entity::AllUsers,
                role: Role::Reader,
            };
            BucketAccessControl::create(&bucket.name, &new_bucket_access_control)?;
            let acl = BucketAccessControl::read(&bucket.name, &Entity::AllUsers)?;
            acl.delete()?;
            bucket.delete()?;
            Ok(())
        }
    }
}

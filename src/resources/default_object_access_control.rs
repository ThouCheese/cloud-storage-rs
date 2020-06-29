#![allow(unused_imports)]

use crate::error::GoogleResponse;
use crate::resources::common::ListResponse;
pub use crate::resources::common::{Entity, ProjectTeam, Role};

/// The DefaultObjectAccessControls resources represent the Access Control Lists (ACLs) applied to a
/// new object within Google Cloud Storage when no ACL was provided for that object. ACLs let you
/// specify who has access to your data and to what extent.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
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
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::default_object_access_control::{
    ///     DefaultObjectAccessControl, NewDefaultObjectAccessControl, Role, Entity,
    /// };
    ///
    /// let new_acl = NewDefaultObjectAccessControl {
    ///     entity: Entity::AllAuthenticatedUsers,
    ///     role: Role::Reader,
    /// };
    /// let default_acl = DefaultObjectAccessControl::create("mybucket", &new_acl)?;
    /// # default_acl.delete()?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "sync")]
    pub fn create(
        bucket: &str,
        new_acl: &NewDefaultObjectAccessControl,
    ) -> Result<Self, crate::Error> {
        let url = format!("{}/b/{}/defaultObjectAcl", crate::BASE_URL, bucket);
        let client = reqwest::blocking::Client::new();
        let result: GoogleResponse<Self> = client
            .post(&url)
            .headers(crate::get_headers()?)
            .json(new_acl)
            .send()?
            .json()?;
        match result {
            GoogleResponse::Success(mut s) => {
                s.bucket = bucket.to_string();
                Ok(s)
            },
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
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::default_object_access_control::DefaultObjectAccessControl;
    ///
    /// let default_acls = DefaultObjectAccessControl::list("mybucket")?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "sync")]
    pub fn list(bucket: &str) -> Result<Vec<Self>, crate::Error> {
        let url = format!("{}/b/{}/defaultObjectAcl", crate::BASE_URL, bucket);
        let client = reqwest::blocking::Client::new();
        let result: GoogleResponse<ListResponse<Self>> = client
            .get(&url)
            .headers(crate::get_headers()?)
            .send()?
            .json()?;
        match result {
            GoogleResponse::Success(s) => {
                Ok(s.items
                    .into_iter()
                    .map(|item| DefaultObjectAccessControl {
                        bucket: bucket.to_string(),
                        ..item
                    })
                    .collect())
            },
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
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::default_object_access_control::{DefaultObjectAccessControl, Entity};
    ///
    /// let default_acl = DefaultObjectAccessControl::read("mybucket", &Entity::AllUsers)?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "sync")]
    pub fn read(bucket: &str, entity: &Entity) -> Result<Self, crate::Error> {
        let url = dbg!(format!(
            "{}/b/{}/defaultObjectAcl/{}",
            crate::BASE_URL,
            bucket,
            entity
        ));
        let client = reqwest::blocking::Client::new();
        let result: GoogleResponse<Self> = client
            .get(&url)
            .headers(crate::get_headers()?)
            .send()?
            .json()?;
        match result {
            GoogleResponse::Success(mut s) => {
                s.bucket = bucket.to_string();
                Ok(s)
            },
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
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::default_object_access_control::{DefaultObjectAccessControl, Entity};
    ///
    /// let mut default_acl = DefaultObjectAccessControl::read("my_bucket", &Entity::AllUsers)?;
    /// default_acl.entity = Entity::AllAuthenticatedUsers;
    /// default_acl.update()?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "sync")]
    pub fn update(&self) -> Result<Self, crate::Error> {
        let url = format!(
            "{}/b/{}/defaultObjectAcl/{}",
            crate::BASE_URL,
            self.bucket,
            self.entity
        );
        let client = reqwest::blocking::Client::new();
        let result: GoogleResponse<Self> = client
            .put(&url)
            .headers(crate::get_headers()?)
            .json(self)
            .send()?
            .json()?;
        match result {
            GoogleResponse::Success(mut s) => {
                s.bucket = self.bucket.to_string();
                Ok(s)
            },
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
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::default_object_access_control::{DefaultObjectAccessControl, Entity};
    ///
    /// let mut default_acl = DefaultObjectAccessControl::read("my_bucket", &Entity::AllUsers)?;
    /// default_acl.delete()?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "sync")]
    pub fn delete(self) -> Result<(), crate::Error> {
        let url = format!(
            "{}/b/{}/defaultObjectAcl/{}",
            crate::BASE_URL,
            self.bucket,
            self.entity
        );
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
            let new_acl = NewDefaultObjectAccessControl {
                entity: Entity::AllUsers,
                role: Role::Reader,
            };
            DefaultObjectAccessControl::create(&bucket.name, &new_acl)?;
            Ok(())
        }

        #[test]
        fn read() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::read_test_bucket();
            NewDefaultObjectAccessControl {
                entity: Entity::AllUsers,
                role: Role::Reader,
            };
            DefaultObjectAccessControl::read(&bucket.name, &Entity::AllUsers)?;
            Ok(())
        }

        #[test]
        fn list() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::read_test_bucket();
            DefaultObjectAccessControl::list(&bucket.name)?;
            Ok(())
        }

        #[test]
        fn update() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::read_test_bucket();
            let new_acl = NewDefaultObjectAccessControl {
                entity: Entity::AllUsers,
                role: Role::Reader,
            };
            let mut default_acl = DefaultObjectAccessControl::create(&bucket.name, &new_acl)?;
            default_acl.entity = Entity::AllAuthenticatedUsers;
            default_acl.update()?;
            Ok(())
        }

        #[test]
        fn delete() -> Result<(), Box<dyn std::error::Error>> {
            let bucket = crate::read_test_bucket();
            let default_acl =
                DefaultObjectAccessControl::read(&bucket.name, &Entity::AllAuthenticatedUsers)?;
            default_acl.delete()?;
            Ok(())
        }
    }
}

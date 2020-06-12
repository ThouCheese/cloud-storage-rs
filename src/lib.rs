//! This crate aims to simplify interacting with the Google Cloud Storage JSON API. Use it until
//! Google releases a Cloud Storage Client Library for Rust. Shoutout to
//! [MyEmma](https://myemma.io/) for funding this free and open source project.
//!
//! Google Cloud Storage is a product by Google that allows for cheap file storage, with a
//! relatively sophisticated API. The idea is that storage happens in `Bucket`s, which are
//! filesystems with a globally unique name. You can make as many of these `Bucket`s as you like!
//!
//! This project talks to Google using a `Service Account`. A service account is an account that you
//! must create in the [cloud storage console](https://console.cloud.google.com/). When the account
//! is created, you can download the file `service-account-********.json`. Store this file somewhere
//! on your machine, and place the path to this file in the environment parameter `SERVICE_ACCOUNT`.
//! Environment parameters declared in the `.env` file are also registered. The service account can
//! then be granted `Roles` in the cloud storage console. The roles required for this project to
//! function are `Service Account Token Creator` and `Storage Object Admin`.
//!
//! # Quickstart
//! Add the following line to your `Cargo.toml`
//! ```toml
//! [dependencies]
//! cloud-storage = "0.3"
//! ```
//! The two most important concepts are [Buckets](bucket/struct.Bucket.html), which represent
//! file systems, and [Objects](object/struct.Object.html), which represent files.
//!
//! ## Examples:
//!
//! TODO: example of creating a client; discussion of sync vs async
//!
//! Creating a new Bucket in Google Cloud Storage:
//! ```rust
//! # use cloud_storage::NewBucket;
//! let client = cloud_storage::sync::Client::new();
//! let bucket = client.create_bucket(&NewBucket {
//!     name: "doctest-bucket".to_string(),
//!     ..Default::default()
//! }).unwrap();
//! # client.delete_bucket(bucket);
//! ```
//! Connecting to an existing Bucket in Google Cloud Storage:
//! ```no_run
//! # use cloud_storage::Bucket;
//! let client = cloud_storage::sync::Client::new();
//! let bucket = client.read_bucket("mybucket").unwrap();
//! ```
//! Read a file from disk and store it on googles server:
//! ```rust,no_run
//! # use cloud_storage::Object;
//! # use std::fs::File;
//! # use std::io::Read;
//! let mut bytes: Vec<u8> = Vec::new();
//! for byte in File::open("myfile.txt").unwrap().bytes() {
//!     bytes.push(byte.unwrap())
//! }
//! Object::create("mybucket", &bytes, "myfile.txt", "text/plain");
//! ```
//! Renaming/moving a file
//! ```rust,no_run
//! # use cloud_storage::Object;
//! let mut object = Object::read("mybucket", "myfile").unwrap();
//! object.name = "mybetterfile".to_string();
//! object.update().unwrap();
//! ```
//! Removing a file
//! ```rust,no_run
//! # use cloud_storage::Object;
//! Object::delete("mybucket", "myfile");
//! ```
#![forbid(unsafe_code, missing_docs)]

/// Contains objects as represented by Google, to be used for serialization and deserialization.
mod error;
mod resources;
/// TODO
pub mod sync;
mod token;

pub use crate::error::*;
use crate::resources::service_account::ServiceAccount;
pub use crate::resources::{
    bucket::{Bucket, NewBucket},
    object::Object,
    *,
};
use crate::token::Token;
use std::sync::Mutex;

use crate::resources::{
    bucket::{IamPolicy, TestIamPermission},
    common::ListResponse,
};

/// Async client
#[derive(Debug, Clone)]
pub struct Client {
    reqwest: reqwest::Client,
}

impl Client {
    /// TODO
    pub fn new() -> Self {
        Self {
            reqwest: reqwest::Client::new(),
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
    pub async fn create_bucket(&self, new_bucket: &NewBucket) -> Result<Bucket, Error> {
        let url = format!("{}/b/", crate::BASE_URL);
        let project = crate::SERVICE_ACCOUNT.project_id.clone();
        let query = [("project", project)];
        let result: GoogleResponse<Bucket> = self.reqwest
            .post(&url)
            .headers(crate::get_headers()?)
            .query(&query)
            .json(new_bucket)
            .send()
            .await?
            .json()
            .await?;
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
    pub async fn read_bucket(&self, name: &str) -> Result<Bucket, Error> {
        let url = format!("{}/b/{}", crate::BASE_URL, name);
        let result: GoogleResponse<Bucket> = self.reqwest
            .get(&url)
            .headers(crate::get_headers()?)
            .send()
            .await?
            .json()
            .await?;
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
    pub async fn list_buckets(&self) -> Result<Vec<Bucket>, Error> {
        let url = format!("{}/b/", crate::BASE_URL);
        let project = crate::SERVICE_ACCOUNT.project_id.clone();
        let query = [("project", project)];
        let result: GoogleResponse<ListResponse<Bucket>> = self.reqwest
            .get(&url)
            .headers(crate::get_headers()?)
            .query(&query)
            .send()
            .await?
            .json()
            .await?;
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
    pub async fn update_bucket(&self, bucket: &Bucket) -> Result<Bucket, Error> {
        let url = format!("{}/b/{}", crate::BASE_URL, bucket.name);
        let result: GoogleResponse<Bucket> = self.reqwest
            .put(&url)
            .headers(crate::get_headers()?)
            .json(bucket)
            .send()
            .await?
            .json()
            .await?;
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
    pub async fn delete_bucket(&self, bucket: Bucket) -> Result<(), Error> {
        let url = format!("{}/b/{}", crate::BASE_URL, bucket.name);
        let response = self.reqwest.delete(&url).headers(crate::get_headers()?).send().await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::Google(response.json().await?))
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
    pub async fn get_bucket_iam_policy(&self, bucket: &Bucket) -> Result<IamPolicy, Error> {
        let url = format!("{}/b/{}/iam", crate::BASE_URL, bucket.name);
        let result: GoogleResponse<IamPolicy> = self.reqwest
            .get(&url)
            .headers(crate::get_headers()?)
            .send()
            .await?
            .json()
            .await?;
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
    pub async fn set_bucket_iam_policy(&self, bucket: &Bucket, iam: &IamPolicy) -> Result<IamPolicy, Error> {
        let url = format!("{}/b/{}/iam", crate::BASE_URL, bucket.name);
        let result: GoogleResponse<IamPolicy> = self.reqwest
            .put(&url)
            .headers(crate::get_headers()?)
            .json(iam)
            .send()
            .await?
            .json()
            .await?;
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
    pub async fn test_bucket_iam_permission(&self, bucket: &Bucket, permission: &str) -> Result<TestIamPermission, Error> {
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
            .send()
            .await?
            .json()
            .await?;
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
    use crate::resources::bucket::{IamRole, StandardIamRole, RetentionPolicy, IamPolicy, Binding};

    // This is written as one long test rather than multiple smaller tests to avoid hitting
    // Google's rate limit on creation and deletion of buckets.
    #[tokio::test]
    async fn bucket_operations() -> Result<(), Box<dyn std::error::Error>> {
        dotenv::dotenv().ok();
        let base_name = std::env::var("TEST_BUCKET")?;

        // Create bucket
        let new_bucket = NewBucket {
            name: format!("{}-test-async-bucket-ops", base_name),
            ..Default::default()
        };
        let client = Client::new();
        let mut bucket = client.create_bucket(&new_bucket).await?;

        // List buckets
        client.list_buckets().await?;

        // Read bucket
        let also_bucket = client.read_bucket(&bucket.name).await?;
        assert_eq!(bucket, also_bucket);

        // Update bucket
        bucket.retention_policy = Some(RetentionPolicy {
            retention_period: 50,
            effective_time: chrono::Utc::now() + chrono::Duration::seconds(50),
            is_locked: Some(false),
        });
        client.update_bucket(&bucket).await?;
        let updated = client.read_bucket(&bucket.name).await?;
        assert_eq!(updated.retention_policy.unwrap().retention_period, 50);

        // Get IAM policy
        client.get_bucket_iam_policy(&bucket).await?;

        // Set IAM policy
        let iam_policy = IamPolicy {
            bindings: vec![Binding {
                role: IamRole::Standard(StandardIamRole::ObjectViewer),
                members: vec!["allUsers".to_string()],
                condition: None,
            }],
            ..Default::default()
        };
        client.set_bucket_iam_policy(&bucket, &iam_policy).await?;
        assert_eq!(
            client.get_bucket_iam_policy(&bucket).await?.bindings,
            iam_policy.bindings
        );

        // Test IAM permission
        client.test_bucket_iam_permission(&bucket, "storage.buckets.get").await?;

        // Delete bucket
        client.delete_bucket(bucket).await?;

        // Delete already deleted bucket should error
        assert!(client.delete_bucket(also_bucket).await.is_err());
        Ok(())
    }
}

lazy_static::lazy_static! {
    /// Static `Token` struct that caches
    static ref TOKEN_CACHE: Mutex<Token> = Mutex::new(Token::new(
        "https://www.googleapis.com/auth/devstorage.full_control",
    ));

    static ref IAM_TOKEN_CACHE: Mutex<Token> = Mutex::new(Token::new(
        "https://www.googleapis.com/auth/iam"
    ));

    /// The struct is the parsed service account json file. It is publicly exported to enable easier
    /// debugging of which service account is currently used. It is of the type
    /// [ServiceAccount](service_account/struct.ServiceAccount.html).
    pub static ref SERVICE_ACCOUNT: ServiceAccount = ServiceAccount::get();
}

const BASE_URL: &'static str = "https://www.googleapis.com/storage/v1";

fn get_headers() -> Result<reqwest::header::HeaderMap, Error> {
    let mut result = reqwest::header::HeaderMap::new();
    let mut guard = TOKEN_CACHE.lock().unwrap();
    let token = guard.get()?;
    result.insert(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {}", token).parse().unwrap(),
    );
    Ok(result)
}

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
    D: serde::Deserializer<'de>,
{
    use serde::de::Deserialize;
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(serde::de::Error::custom)
}

fn from_str_opt<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
    D: serde::Deserializer<'de>,
{
    let s: Result<serde_json::Value, _> = serde::Deserialize::deserialize(deserializer);
    println!("{:?}", s);
    match s {
        Ok(serde_json::Value::String(s)) => T::from_str(&s)
            .map_err(serde::de::Error::custom)
            .map(Option::from),
        Ok(serde_json::Value::Number(num)) => T::from_str(&num.to_string())
            .map_err(serde::de::Error::custom)
            .map(Option::from),
        Ok(_value) => Err(serde::de::Error::custom("Incorrect type")),
        Err(_) => Ok(None),
    }
}

#[cfg(test)]
fn read_test_bucket() -> Bucket {
    dotenv::dotenv().ok();
    let name = std::env::var("TEST_BUCKET").unwrap();
    let client = sync::Client::new();
    match client.read_bucket(&name) {
        Ok(bucket) => bucket,
        Err(_not_found) => client.create_bucket(&NewBucket {
            name,
            ..Default::default()
        })
        .unwrap(),
    }
}

// since all tests run in parallel, we need to make sure we do not create multiple buckets with
// the same name in each test.
#[cfg(test)]
fn create_test_bucket(name: &str) -> Bucket {
    dotenv::dotenv().ok();
    let base_name = std::env::var("TEST_BUCKET").unwrap();
    let name = format!("{}-{}", base_name, name);
    let client = sync::Client::new();
    let new_bucket = NewBucket {
        name,
        ..Default::default()
    };
    match client.create_bucket(&new_bucket) {
        Ok(bucket) => bucket,
        Err(_alread_exists) => client.read_bucket(&new_bucket.name).unwrap(),
    }
}

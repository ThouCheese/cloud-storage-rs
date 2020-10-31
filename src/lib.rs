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
//! cloud-storage = "0.6"
//! ```
//! The two most important concepts are [Buckets](bucket/struct.Bucket.html), which represent
//! file systems, and [Objects](object/struct.Object.html), which represent files.
//!
//! ## Examples:
//! Creating a new Bucket in Google Cloud Storage:
//! ```rust
//! # use cloud_storage::{Bucket, NewBucket};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let bucket = Bucket::create(&NewBucket {
//!     name: "doctest-bucket".to_string(),
//!     ..Default::default()
//! }).await?;
//! # bucket.delete().await?;
//! # Ok(())
//! # }
//! ```
//! Connecting to an existing Bucket in Google Cloud Storage:
//! ```no_run
//! # use cloud_storage::Bucket;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let bucket = Bucket::read("mybucket").await?;
//! # Ok(())
//! # }
//! ```
//! Read a file from disk and store it on googles server:
//! ```rust,no_run
//! # use cloud_storage::Object;
//! # use std::fs::File;
//! # use std::io::Read;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut bytes: Vec<u8> = Vec::new();
//! for byte in File::open("myfile.txt")?.bytes() {
//!     bytes.push(byte?)
//! }
//! Object::create("mybucket", bytes, "myfile.txt", "text/plain").await?;
//! # Ok(())
//! # }
//! ```
//! Renaming/moving a file
//! ```rust,no_run
//! # use cloud_storage::Object;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut object = Object::read("mybucket", "myfile").await?;
//! object.name = "mybetterfile".to_string();
//! object.update().await?;
//! # Ok(())
//! # }
//! ```
//! Removing a file
//! ```rust,no_run
//! # use cloud_storage::Object;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! Object::delete("mybucket", "myfile").await?;
//! # Ok(())
//! # }
//! ```
#![forbid(unsafe_code, missing_docs)]

mod download_options;
/// Contains objects as represented by Google, to be used for serialization and deserialization.
mod error;
mod resources;
mod token;

pub use crate::error::*;
use crate::resources::service_account::ServiceAccount;
pub use crate::resources::{
    bucket::{Bucket, NewBucket},
    object::Object,
    *,
};
use crate::token::Token;
pub use download_options::DownloadOptions;
use tokio::sync::Mutex;

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

    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

/// A type alias where the error is set to be `cloud_storage::Error`.
pub type Result<T> = std::result::Result<T, crate::Error>;

const BASE_URL: &str = "https://www.googleapis.com/storage/v1";

async fn get_headers() -> Result<reqwest::header::HeaderMap> {
    let mut result = reqwest::header::HeaderMap::new();
    let mut guard = TOKEN_CACHE.lock().await;
    let token = guard.get().await?;
    result.insert(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {}", token).parse().unwrap(),
    );
    Ok(result)
}

fn from_str<'de, T, D>(deserializer: D) -> std::result::Result<T, D::Error>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
    D: serde::Deserializer<'de>,
{
    use serde::de::Deserialize;
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(serde::de::Error::custom)
}

fn from_str_opt<'de, T, D>(deserializer: D) -> std::result::Result<Option<T>, D::Error>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
    D: serde::Deserializer<'de>,
{
    let s: std::result::Result<serde_json::Value, _> =
        serde::Deserialize::deserialize(deserializer);
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
#[cfg(feature = "sync")]
#[tokio::main]
async fn read_test_bucket_sync() -> Bucket {
    read_test_bucket().await
}

#[cfg(test)]
async fn read_test_bucket() -> Bucket {
    dotenv::dotenv().ok();
    let name = std::env::var("TEST_BUCKET").unwrap();
    match Bucket::read(&name).await {
        Ok(bucket) => bucket,
        Err(_not_found) => Bucket::create(&NewBucket {
            name,
            ..NewBucket::default()
        })
        .await
        .unwrap(),
    }
}

// since all tests run in parallel, we need to make sure we do not create multiple buckets with
// the same name in each test.
#[cfg(test)]
#[cfg(feature = "sync")]
#[tokio::main]
async fn create_test_bucket_sync(name: &str) -> Bucket {
    create_test_bucket(name).await
}

// since all tests run in parallel, we need to make sure we do not create multiple buckets with
// the same name in each test.
#[cfg(test)]
async fn create_test_bucket(name: &str) -> Bucket {
    std::thread::sleep(std::time::Duration::from_millis(1500)); // avoid getting rate limited

    dotenv::dotenv().ok();
    let base_name = std::env::var("TEST_BUCKET").unwrap();
    let name = format!("{}-{}", base_name, name);
    let new_bucket = NewBucket {
        name,
        ..NewBucket::default()
    };
    match Bucket::create(&new_bucket).await {
        Ok(bucket) => bucket,
        Err(_alread_exists) => Bucket::read(&new_bucket.name).await.unwrap(),
    }
}

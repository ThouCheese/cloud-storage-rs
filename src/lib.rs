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
//! Creating a new Bucket in Google Cloud Storage:
//! ```rust
//! # use cloud_storage::{Bucket, NewBucket};
//! let bucket = Bucket::create(&NewBucket {
//!     name: "doctest-bucket".to_string(),
//!     ..Default::default()
//! }).unwrap();
//! # bucket.delete();
//! ```
//! Connecting to an existing Bucket in Google Cloud Storage:
//! ```no_run
//! # use cloud_storage::Bucket;
//! let bucket = Bucket::read("mybucket").unwrap();
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
//! let object = Object::read("mybucket", "myfile").unwrap();
//! object.delete();
//! ```
#![forbid(unsafe_code, missing_docs)]

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
use std::sync::Mutex;

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
    match Bucket::read(&name) {
        Ok(bucket) => bucket,
        Err(_not_found) => Bucket::create(&NewBucket {
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
    let new_bucket = NewBucket {
        name,
        ..Default::default()
    };
    match dbg!(Bucket::create(&new_bucket)) {
        Ok(bucket) => bucket,
        Err(_alread_exists) => dbg!(Bucket::read(&new_bucket.name)).unwrap(),
    }
}

#![feature(try_trait_v2)]
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
//! cloud-storage = "0.10"
//! ```
//! The two most important concepts are [Buckets](bucket/struct.Bucket.html), which represent
//! file systems, and [Objects](object/struct.Object.html), which represent files.
//!
//! ## Examples:
//! Creating a new Bucket in Google Cloud Storage:
//! ```rust
//! # use cloud_storage::{Client, Bucket, create::Bucket};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Client::default();
//! let bucket = client.bucket().create(&create::Bucket {
//!     name: "doctest-bucket".to_string(),
//!     ..Default::default()
//! }).await?;
//! # client.bucket().delete(bucket).await?;
//! # Ok(())
//! # }
//! ```
//! Connecting to an existing Bucket in Google Cloud Storage:
//! ```no_run
//! # use cloud_storage::{Client, Bucket};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Client::default();
//! let bucket = client.bucket().read("my_bucket").await?;
//! # Ok(())
//! # }
//! ```
//! Read a file from disk and store it on googles server:
//! ```rust,no_run
//! # use cloud_storage::{Client, Object};
//! # use std::fs::File;
//! # use std::io::Read;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut bytes: Vec<u8> = Vec::new();
//! for byte in File::open("myfile.txt")?.bytes() {
//!     bytes.push(byte?)
//! }
//! let client = Client::default();
//! client.object("my_bucket").create(bytes, "myfile.txt", "text/plain").await?;
//! # Ok(())
//! # }
//! ```
//! Renaming/moving a file
//! ```rust,no_run
//! # use cloud_storage::{Client, Object};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Client::default();
//! let mut object = client.object("my_bucket").read("myfile").await?;
//! object.content_type = Some("application/xml".to_string());
//! client.object().update(&object).await?;
//! # Ok(())
//! # }
//! ```
//! Removing a file
//! ```rust,no_run
//! # use cloud_storage::{Client, Object};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Client::default();
//! client.object("my_bucket").delete("myfile").await?;
//! # Ok(())
//! # }
//! ```
#![forbid(unsafe_code, missing_docs)]
pub mod client;
#[cfg(feature = "sync")]
pub mod sync;

// export time, so implementing libraries can use it
pub extern crate time;

mod configuration;
pub mod models;
mod download_options;
mod error;
mod token;

#[cfg(feature = "global-client")]
mod global_client;
mod crypto;
mod sized_byte_stream;
#[cfg(feature = "global-client")]
use crate::global_client::CLOUD_CLIENT;

pub use crate::{
    client::Client,
    download_options::DownloadOptions,
    error::Error,
    models::{Bucket, ListRequest, Object},
    configuration::ServiceAccount,
    token::{Token, TokenCache},
};

const ISO_8601_BASIC_FORMAT: &[::time::format_description::FormatItem<'_>] = time::macros::format_description!("[year][month][day]T[hour][minute][second]Z");
// todo: may or may not do stuff?
time::serde::format_description!(rfc3339_date, Date, "[year]-[month]-[day]");

fn from_str<'de, T, D>(deserializer: D) -> std::result::Result<T, D::Error> where T: std::str::FromStr, T::Err: std::fmt::Display, D: serde::Deserializer<'de>
{
    use serde::de::Deserialize;
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(serde::de::Error::custom)
}

fn from_str_opt<'de, T, D>(deserializer: D) -> std::result::Result<Option<T>, D::Error>
where T: std::str::FromStr, T::Err: std::fmt::Display, D: serde::Deserializer<'de>,
{
    let s: std::result::Result<serde_json::Value, _> = serde::Deserialize::deserialize(deserializer);
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

use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};

const ENCODE_SET: &AsciiSet = &NON_ALPHANUMERIC.remove(b'*').remove(b'-').remove(b'.').remove(b'_');
const NOSLASH_ENCODE_SET: &AsciiSet = &ENCODE_SET.remove(b'/').remove(b'~');

// We need to be able to percent encode stuff, but without touching the slashes in filenames. To
// this end we create an implementation that does this, without touching the slashes.
fn percent_encode_noslash(input: &str) -> String {
    utf8_percent_encode(input, NOSLASH_ENCODE_SET).to_string()
}

pub(crate) fn percent_encode(input: &str) -> String {
    utf8_percent_encode(input, ENCODE_SET).to_string()
}

#[cfg(feature = "sync")]
fn runtime() -> Result<tokio::runtime::Runtime, Error> {
    Ok(tokio::runtime::Builder::new_current_thread().thread_name("cloud-storage-worker").enable_time().enable_io().build()?)
}
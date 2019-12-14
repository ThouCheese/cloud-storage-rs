//! This crate aims to simplify interacting with the Google Cloud Storage JSON API. Use it until
//! Google releases a Cloud Storage Client Library for Rust. Shoutout to [YMF](https://ymf.io/) for
//! funding this free and open source project.
//!
//! It lazily generates tokens that are cached for one hour. After this hour, at the next request,
//! a new token is fetched from google, which is then again cached for one hour. Caching these
//! tokens is thread-safe, but does require locking.
//!
//! This project talks to Google using a `Service Account`. A service account is an account that you
//! must create in the [cloud storage console](https://console.cloud.google.com/). When the account
//! is created, you can download the file `service-account-********.json`. Store this file somewhere
//! on your machine, and place the path to this file in the environment parameter `SERVICE_ACCOUNT`.
//! Environment parameters declared in the `.env` file are also registered. The service account can
//! then be granted `Roles` in the cloud storage console. The roles required for this project to
//! function are `Service Account Token Creator` and `Storage Object Admin`.
//!
//!
//! ## Examples:
//! Creating a new Bucket in Google Cloud Storage:
//! ```rust,no_run
//! let bucket = Bucket::create("mybucket").unwrap();
//! ```
//! Connecting to an existing Bucket in Google Cloud Storage:
//! ```
//! let bucket = Bucket::existing("mybucket"); // note: doesn't fail, even if the name is incorrect
//! ```
//! Read a file from disk and store it on googles server:
//! ```rust,no_run
//! let mut bytes: Vec<u8> = Vec::new();
//! for byte in File::open("myfile.txt").unwrap().bytes() {
//!     bytes.push(byte.unwrap())
//! }
//! let bucket = Bucket::existing("mybucket");
//! bucket.upload(&bytes, "mydifferentfilename.txt", "text/plain");
//! ```
//! Renaming or and moving a file
//! ```rust,no_run
//! let bucket = Bucket::existing("mybucket");
//! bucket.update("old/path/to/resource.txt", "newname.txt").unwrap();
//! ```
//! Removing a file
//! ```rust,no_run
//! let bucket = Bucket::existing("mybucket");
//! bucket.update("old/path/to/resource.txt", "newname.txt").unwrap();
//! ```

#![deny(unsafe_code, missing_docs)]

mod error;
mod resources;
mod service_account;
mod token;

pub use crate::error::Error;
use crate::service_account::SERVICE_ACCOUNT;
use crate::token::Token;
use chrono as chr;
use lazy_static::lazy_static;
use openssl::{hash::MessageDigest, pkey::PKey, sha, sign::Signer};
use reqwest::header::*;
use std::{collections::HashMap, sync::Mutex};
use url::percent_encoding::{utf8_percent_encode, PATH_SEGMENT_ENCODE_SET};

// delay construction of these since mutex are not thread-safe and we have no reason to assume that
// the user will not attempt concurrent access
lazy_static! {
    /// Static `Token` struct that caches
    static ref TOKEN_CACHE: Mutex<Token> = Mutex::new(Token::new(
        "https://www.googleapis.com/auth/devstorage.full_control",
    ));

    static ref IAM_TOKEN_CACHE: Mutex<Token> = Mutex::new(Token::new(
        "https://www.googleapis.com/auth/iam"
    ));
}

/// Represents a Bucket in Google Cloud Storage that can be used to upload, download or move files.
/// Internally, the files in the bucket live in a flat namespace, that is, the slashes that indicate
/// folders are simply part of the filename. This means that renaming a file and moving it to a
/// different directory are the same operation.
pub struct Bucket {
    name: String,
}

// We need to be able to percent encode stuff, but without touching the slashes in filenames. To
// this end we create an implementation that does this, without touching the slashes.
fn percent_encode_noslash(input: &str) -> String {
    percent_encode(input).replace("%2F", "/")
}

fn percent_encode(input: &str) -> String {
    utf8_percent_encode(input, PATH_SEGMENT_ENCODE_SET)
        .to_string()
        .replace("&", "%26")
}

impl Bucket {
    /// Returns a Bucket struct with the corresponding name. If the bucket does not exist, an
    /// attempt to store or retrieve files will always fail.
    /// ```
    /// let bucket = Bucket::existing("my-companies-production-bucket");
    /// ```
    pub fn existing(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    fn get_headers() -> HeaderMap {
        let mut result = HeaderMap::new();
        let token = TOKEN_CACHE.lock().unwrap().get();
        result.insert(AUTHORIZATION, format!("Bearer {}", token).parse().unwrap());
        result
    }

    /// Creates a new Bucket resource on Goolge's server, then returns a struct representing this
    /// Bucket.
    /// ```rust,no_run
    /// let bucket = Bucket::create("my-companies-staging-bucket").unwrap();
    /// ```
    pub fn create(name: &str, location: Option<&str>) -> Result<Self, Error> {
        const BASE_URL: &str = "https://www.googleapis.com/storage/v1/b";
        let url = format!("{}?project={}", BASE_URL, SERVICE_ACCOUNT.project_id);
        let client = reqwest::Client::new();
        let mut body = HashMap::new();
        body.insert("name", name);
        if let Some(location) = location {
            body.insert("location", location);
        }
        let mut response = client
            .post(&url)
            .headers(Self::get_headers())
            .json(&body)
            .send()?;
        if response.status() == 200 {
            Ok(Self {
                name: name.to_string(),
            })
        } else {
            Err(Error::new(&response.text().unwrap()))
        }
    }

    /// Upload a file as that is loaded in memory to google cloud storage, where it will be
    /// interpreted according to the mime type you specified.
    /// ```
    /// let bucket = Bucket::existing("cat-photos");
    /// let file: Vec<u8> = read_cute_cat("cat.png");
    /// bucket.upload(&file, "recently read cat.png", "image/png").expect("cat not uploaded");
    /// ```
    pub fn upload(&self, file: &[u8], filename: &str, mime_type: &str) -> Result<(), Error> {
        const BASE_URL: &str = "https://www.googleapis.com/upload/storage/v1/b";
        let filename = percent_encode(filename);
        let client = reqwest::Client::new();
        let url = &format!(
            "{}/{}/o?uploadType=media&name={}",
            BASE_URL, self.name, filename
        );
        let mut headers = Self::get_headers();
        headers.insert(CONTENT_TYPE, mime_type.to_string().parse().unwrap());
        headers.insert(CONTENT_LENGTH, file.len().to_string().parse().unwrap());
        let mut response = client
            .post(url)
            .headers(headers)
            .body(file.to_owned())
            .send()?;
        if response.status() == 200 {
            Ok(())
        } else {
            Err(Error::new(&response.text().unwrap()))
        }
    }

    // pub fn download(&self, filename: &str) -> Result<String, Error> {
    //     const BASE_URL: &str = "https://www.googleapis.com/storage/v1/b";
    //     let filename = percent_encode(filename);
    //     let url = &format!("{}/{}/o/{}", BASE_URL, self.name, filename);
    //     let client = reqwest::Client::new();
    //     let mut response = client.get(url).headers(Self::get_headers()).send().unwrap();
    //     if response.status() == 200 {
    //         Ok(response.text().unwrap())
    //     } else {
    //         Err(Error::new(&response.text().unwrap()))
    //     }
    // }

    /// Allows renaming a file. Note that since a files name is its full path, you can use the
    /// renaming capacities to move files as well.
    /// ```
    /// let bucket = Bucket::existing("cat-photos");
    /// bucket.update("recently read cat.png", "cuties/old cat.png").expect("cat not moved");
    /// ```
    pub fn update(&self, old_name: &str, new_name: &str) -> Result<(), Error> {
        let old_name = percent_encode(old_name.trim_matches('/'));
        let new_name = percent_encode(new_name.trim_matches('/'));
        let url = &format!(
            "https://www.googleapis.com/storage/v1/b/{}/o/{}/rewriteTo/b/{}/o/{}",
            self.name, old_name, self.name, new_name
        );

        let mut headers = Self::get_headers();
        headers.insert(CONTENT_LENGTH, "0".parse().unwrap());
        let client = reqwest::Client::new();
        let mut response = client.post(url).headers(headers).send()?;
        if response.status() == 200 {
            Ok(())
        } else {
            Err(Error::new(&response.text().unwrap()))
        }
    }

    /// Removes a file from the Bucket. Depending on your content retention policy, this may very
    /// well be permanent!
    /// ```
    /// let bucket = Bucket::existing("cat-photos");
    /// bucket.delete("dog.jpg").expect("wrong animal not removed");
    /// ```
    pub fn delete(&self, name: &str) -> Result<(), Error> {
        let name = percent_encode(name.trim_matches('/'));

        let url = &format!(
            "https://www.googleapis.com/storage/v1/b/{}/o/{}",
            self.name, name
        );

        let client = reqwest::Client::new();
        let mut response = client.delete(url).headers(Self::get_headers()).send()?;

        if response.status() == 200 || response.status() == 204 {
            Ok(())
        } else {
            Err(Error::new(&response.text().unwrap()))
        }
    }

    /// Generates a signed url that is valid for `duration` seconds, that lets anyone read the file
    /// without further authentication. Note that this function returns a `String`, so generating
    /// the signed url will never fail. If the file does not exist, or the service account has no
    /// access to the file, then the signed url will result in a `404` or a `401`.
    /// ```
    /// let bucket = Bucket::existing("cat-photos");
    /// let url = bucket.download_url("cuties/old cat.png");
    /// // now we can download the file as desired, for example:
    /// let file = reqwest::blocking::get(&url).unwrap()
    ///     .bytes().unwrap();
    /// // we now have the file again
    /// ```
    pub fn download_url(&self, file_path: &str, duration: u32) -> String {
        self.sign(file_path, duration, "GET")
    }

    // pub fn upload_url(&self, file_path: &str, duration: u32) -> String {
    //     self.sign(file_path, duration, "POST")
    // }

    #[inline(always)]
    fn sign(&self, file_path: &str, duration: u32, http_verb: &str) -> String {
        // 1 construct the canonical request
        let issue_date = chr::Utc::now();
        let file_path = self.path_to_resource(file_path);
        let query_string = Self::get_canonical_query_string(&issue_date, duration);
        let canonical_request = self.get_canonical_request(&file_path, &query_string, http_verb);

        // 2 get hex encoded SHA256 hash the canonical request
        let hash = sha::sha256(canonical_request.as_bytes());
        let hex_hash = hex::encode(hash);

        // 3 construct the string to sign
        let string_to_sign = format!(
            "{signing_algorithm}\n{current_datetime}\n{credential_scope}\n{hashed_canonical_request}",
            signing_algorithm="GOOG4-RSA-SHA256",
            current_datetime=issue_date.format("%Y%m%dT%H%M%SZ"),
            credential_scope=Self::get_credential_scope(&issue_date),
            hashed_canonical_request=hex_hash,
        );

        // 4 sign the string to sign with RSA - SHA256
        let buffer = Self::sign_str(&string_to_sign);
        let signature = hex::encode(&buffer);

        // 5 construct the signed url
        format!(
            "https://storage.googleapis.com{path_to_resource}?&{query_string}&X-Goog-Signature={request_signature}",
            path_to_resource=file_path,
            query_string=query_string,
            request_signature=signature,
        )
    }

    #[inline(always)]
    fn get_canonical_request(
        &self,
        path: &str,
        query_string: &str,
        http_verb: &str
    ) -> String {
        format!(
            "{http_verb}\n{path_to_resource}\n{canonical_query_string}\n{canonical_headers}\n\n{signed_headers}\n{payload}",
            http_verb=http_verb,
            path_to_resource=path,
            canonical_query_string=query_string,
            canonical_headers="host:storage.googleapis.com",
            signed_headers="host",
            payload="UNSIGNED-PAYLOAD",
        )
    }

    #[inline(always)]
    fn get_canonical_query_string(date: &chr::DateTime<chr::Utc>, exp: u32) -> String {
        let credential = format!(
            "{authorizer}/{scope}",
            authorizer = SERVICE_ACCOUNT.client_email,
            scope = Self::get_credential_scope(date),
        );
        format!(
            "X-Goog-Algorithm={algo}&X-Goog-Credential={cred}&X-Goog-Date={date}&X-Goog-Expires={exp}&X-Goog-SignedHeaders={signed}",
            algo="GOOG4-RSA-SHA256",
            cred=percent_encode(&credential).replace("@", "%40"),
            date=date.format("%Y%m%dT%H%M%SZ"),
            exp=exp,
            signed="host",
        )
    }

    #[inline(always)]
    fn path_to_resource(&self, path: &str) -> String {
        format!(
            "/{bucket}/{file_path}",
            bucket = self.name,
            file_path = percent_encode_noslash(path),
        )
    }

    #[inline(always)]
    fn get_credential_scope(date: &chr::DateTime<chr::Utc>) -> String {
        format!("{}/henk/storage/goog4_request", date.format("%Y%m%d"))
    }

    #[inline(always)]
    fn sign_str(message: &str) -> Vec<u8> {
        let key = PKey::private_key_from_pem(SERVICE_ACCOUNT.private_key.as_bytes()).unwrap();
        let mut signer = Signer::new(MessageDigest::sha256(), &key).unwrap();
        signer.update(message.as_bytes()).unwrap();
        signer.sign_to_vec().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_bucket() {
        Bucket::create("tmp-bckt", None).unwrap();
    }
}

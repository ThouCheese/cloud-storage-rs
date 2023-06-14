# Cloud Storage

[![cloud-storage-rs on crates.io](https://img.shields.io/crates/v/cloud-storage.svg)](https://crates.io/crates/cloud-storage)
[![stripe-rust on docs.rs](https://docs.rs/cloud-storage/badge.svg)](https://docs.rs/cloud-storage)

A library that can be used to push blobs to [Google Cloud Storage](https://cloud.google.com/storage/), and then generate download links to those files.
### Usage
Add the following line to your Cargo.toml
```toml
[dependencies]
cloud-storage = "1.0.0"
```
### Examples
```rust
// create a new Bucket
let new_bucket = create::Bucket { name: "my_bucket".to_string(), ..Default::default() };
let bucket = Bucket::create(&new_bucket).await?;
// upload a file to our new bucket
let content = b"Your file is now on google cloud storage!".to_vec();
let object = Object::create(&bucket.name, content, "folder/filename.txt", "application/text", None).await?;
// let's copy the file
object.copy("my_other_bucket", "otherfolder/filename.txt", None).await?;
// print a link to the file
println!("{}", object.download_url(1000)?); // download link that expires after 1000 seconds
// remove the file from the bucket
Object::delete(&bucket.name, "folder/filename.txt", None).await?;
```

When using `CloudStorageClient::default()`, `sync::CloudStorageClient::new()` or the global client, an ServiceAccount will be created based on either of the environmental variables:
 * `SERVICE_ACCOUNT` or `GOOGLE_APPLICATION_CREDENTIALS` which should contain path to the `service-account-*******.json`
 * `SERVICE_ACCOUNT_JSON` or `GOOGLE_APPLICATION_CREDENTIALS_JSON` containing the contents of `service-account-*******.json`

The service account requires the roles `Service Account Token Creator` (for generating access tokens) and `Storage Object Admin` (for generating signed urls to download the files).

### Sync
If you're not (yet) interested in running an async executor, then `cloud_storage` exposes a sync api. To use it, enable the feature flag `sync`, and then call instead of calling `function().await`, call `function_sync()`.

You will need to set both the `global-client` and `sync` flags in your Cargo.toml, for example:

```
cloud-storage = { version = "1.0.0", features = ["global-client", "sync"] }
```

### Testing
To run the tests for this project, first create an enviroment parameter (or entry in the .env file) named TEST_BUCKET. Make sure that this name is not already in use! The tests will create this bucket for its testing purposes. It will also create a couple of other buckets with this name as prefix, but these will be deleted again. Next, you will need a Google Cloud Storage project, for which you must create a service account. Download the service-account.json file and place the path to the file in the `SERVICE_ACCOUNT` environment parameter. Then, run
```bash
sh test.sh
```
The `test-threads=1` is necessary so that the tests don't exceed the 2 per second bucket creating rate limit. (Depending on your internet speed, you may be able to use more than 1 test thread)
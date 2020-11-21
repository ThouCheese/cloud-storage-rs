# Cloud Storage

[![cloud-storage-rs on crates.io](https://img.shields.io/crates/v/cloud-storage.svg)](https://crates.io/crates/cloud-storage)
[![stripe-rust on docs.rs](https://docs.rs/cloud-storage/badge.svg)](https://docs.rs/cloud-storage)

A library that can be used to push blobs to [Google Cloud Storage](https://cloud.google.com/storage/), and then generate download links to those files.
### Usage
Add the following line to you Cargo.toml
```toml
[dependencies]
cloud-storage = "0.6"
```
### Examples
```rust
// create a new Bucket
let new_bucket = NewBucket { name: "mybucket", ..Default::default() }
let bucket = Bucket::create(new_bucket).await?;
// upload a file to our new bucket
let content = b"Your file is now on google cloud storage!";
bucket.upload(content, "folder/filename.txt", "application/text").await?;
let mut object = Object::create("mybucket", content, "folder/filename.txt", "application/text").await?;
// let's copy the file
object.copy("mybucket2: electric boogaloo", "otherfolder/filename.txt").await?;
// print a link to the file
println!("{}", object.download_url(1000)); // download link for 1000 seconds
// remove the file from the bucket
object.delete().await?;
```

Authorization can be granted using the `SERVICE_ACCOUNT` or `GOOGLE_APPLICATION_CREDENTIALS` environment variable, which should contain path to the `service-account-*******.json` file that contains the Google credentials. Alternatively, the service account credentials can be provided as JSON directly through the `SERVICE_ACCOUNT_JSON` or `GOOGLE_APPLICATION_CREDENTIALS_JSON` environment variable, which is useful when providing secrets in CI or k8s.

The service account requires the permission `devstorage.full_control`. This is not strictly necessary, so if you need this fixed, let me know!  
The service account should also have the roles `Service Account Token Creator` (for generating access tokens) and `Storage Object Admin` (for generating sign urls to download the files).

### Sync
If you're not (yet) interested in running an async executor, then `cloud_storage` exposes a sync api. To use it, enable the feature flag `sync`, and then call instead of calling `function().await`, call `function_sync()`.

### Testing
To run the tests for this project, first create an enviroment parameter (or entry in the .env file) named TEST_BUCKET. Make sure that this name is not already in use! The tests will create this bucket for its testing purposes. It will also create a couple of other buckets with this name as prefix, but these will be deleted again. Next, you will need a Google Cloud Storage project, for which you must create a service account. Download the service-account.json file and place the path to the file in the `SERVICE_ACCOUNT` environment parameter. Then, run
```bash
cargo test --tests --features=sync -- --test-threads=1
```
The `test-threads=1` is necessary so that the tests don't exceed the 2 per second bucket creating rate limit. (Depending on your internet speed, you may be able to use more than 1 test thread)

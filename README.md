# Cloud Storage

[![cloud-storage-rs on crates.io](https://img.shields.io/crates/v/cloud-storage-rs.svg)](https://crates.io/crates/cloud-storage-rs)
[![stripe-rust on docs.rs](https://docs.rs/cloud-storage-rs/badge.svg)](https://docs.rs/cloud-storage-rs)

A library that can be used to push blobs to [Google Cloud Storage](https://cloud.google.com/storage/), and then generate download links to those files:

```
// create a new Bucket
let bucket = Bucket::create("mybucket").unwrap();
// upload a file to our new bucket
bucket.upload(b"Your file is now on google cloud storage!", "folder/filename.txt", "application/text").unwrap();
// let's rename the file
bucket.update("folder/filename.txt", "new filename.txt").unwrap();
// print a link to the file
println!("{}", bucket.download_url("new filename.txt", 1000)); // download link for 1000 seconds
// remove the file from the bucket
bucket.delete("folder/filename.txt").unwrap();
```

Authorization can be granted using the `SERVICE_ACCOUNT` environment variable, which should contain path to the `service-account-*******.json` file that contains the Google credentials. The service account requires the permission `devstorage.full_control`. This is not strictly necessary, so if you need this fixed, let me know! 

The service account should also have the roles `Service Account Token Creator` (for generating access tokens) and `Storage Object Admin` (for generating sign urls to download the files).

The exposed API is very bare-bones right now, only allowing storing, reading, moving and deleting. Feel free to open up a pull request is you need more complete support, it shouldn't be too much work to add more endpoints.

### Testing
To run the tests for this project, first create an enviroment parameter (or entry in the .env file) named TEST_BUCKET. Make sure that this name is not already in use! The tests will create this bucket for its testing purposes. It will also create a couple of other buckets with this name as prefix, but these will be deleted again. Next, you will need a Google Cloud Storage project, for which you must create a service account. Download the service-account.json file and place the path to the file in the `SERVICE_ACCOUNT` environment parameter. Then, run
```
cargo test --tests -- --test-threads=1
```
The `test-threads=1` is necessary so that the tests don't exceed the 2 per second bucket creating rate limit. (Depending on your internet speed, you may be able to use more than 1 test thread)
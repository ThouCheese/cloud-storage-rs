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
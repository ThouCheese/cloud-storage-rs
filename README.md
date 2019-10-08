# Cloud Storage

A library that can be used to push blobs to [Google Cloud Storage]
(https://cloud.google.com/storage/), and then generate download links to those files:

```
// create a new Bucket
let bucket = Bucket::create("mybucket").unwrap();
bucket.upload(b"Your file is now on google cloud storage!", "folder/filename.txt", "application/text").unwrap();
bucket.update("folder/filename.txt", "new filename.txt").unwrap();
println!("{}", bucket.download_url("new filename.txt", 1000)); // download link for 1000 seconds
bucket.delete("folder/filename.txt").unwrap();
```

Authorization can be granted using the `SERVICE_ACCOUNT` environment variable, which sould the path
to the `service-account-*******.json` file that contains the Google credentials. The service account
requires the permission `devstorage.full_control`. This is not strictly necessary, so if you _need_
this fixed, let me know! 

The service account should also have the roles `Service Account Token Creator` (for generating
access tokens) and `Storage Object Admin` (for generating sign urls to download the files).

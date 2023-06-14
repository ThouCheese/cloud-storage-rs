mod bucket;
mod bucket_access_control;
mod default_object_access_control;
mod hmac_key;
mod object_access_control;
mod object;
use once_cell::sync::Lazy;

pub(crate) static CLOUD_CLIENT: Lazy<crate::client::CloudStorageClient> = Lazy::new(crate::client::CloudStorageClient::default);

#[cfg(test)]
pub(crate) use self::test_helpers::*;

#[cfg(test)]
mod test_helpers {
    use crate::{Bucket, models::create};

    pub(crate) async fn read_test_bucket() -> Bucket {
        #[cfg(feature = "dotenv")]
        dotenv::dotenv().ok();
        let name = std::env::var("TEST_BUCKET").unwrap();
        match Bucket::read(&name).await {
            Ok(bucket) => bucket,
            Err(_not_found) => {
                Bucket::create(&create::Bucket {
                    name,
                    ..create::Bucket::default()
                })
                .await
                .unwrap()
            },
        }
    }

    #[cfg(feature = "sync")]
    pub(crate) fn read_test_bucket_sync() -> Bucket {
        crate::runtime().unwrap().block_on(read_test_bucket())
    }

    // since all tests run in parallel, we need to make sure we do not create multiple buckets with
    // the same name in each test.
    #[cfg(feature = "sync")]
    pub(crate) fn create_test_bucket_sync(name: &str) -> Bucket {
        crate::runtime().unwrap().block_on(create_test_bucket(name))
    }

    // since all tests run in parallel, we need to make sure we do not create multiple buckets with
    // the same name in each test.
    pub(crate) async fn create_test_bucket(name: &str) -> Bucket {
        std::thread::sleep(std::time::Duration::from_millis(1500)); // avoid getting rate limited
        
        #[cfg(feature = "dotenv")]
        dotenv::dotenv().ok();
        let base_name = std::env::var("TEST_BUCKET").unwrap();
        let name = format!("{}-{}", base_name, name);
        let new_bucket = create::Bucket {
            name,
            ..create::Bucket::default()
        };
        match Bucket::create(&new_bucket).await {
            Ok(bucket) => bucket,
            Err(_alread_exists) => Bucket::read(&new_bucket.name).await.unwrap(),
        }
    }
}
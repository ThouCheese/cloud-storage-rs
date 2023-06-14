use serde::Deserialize;
use super::ErrorResponse;

#[derive(Debug)]
pub(crate) struct Response<T>(Result<T, ErrorResponse>);

#[derive(serde::Deserialize)]
#[serde(untagged)]
/// Private Response that will be transformed into Response<T> in the Deserialize trait of Response<T>
enum EnumResponse<T> {
    Success(T),
    Error(ErrorResponse),
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Response<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
            match EnumResponse::<T>::deserialize(deserializer)? {
                EnumResponse::Success(value) => Ok(Response(Ok(value))),
                EnumResponse::Error(value) => Ok(Response(Err(value))),
            }
    }
}

impl<T> Response<T> {
    /// Transform the output into an result
    pub fn ok(self) -> Result<T, ErrorResponse> {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::{Error, Bucket};

    use super::Response;

    #[test]
    fn test_try_impl() -> Result<(), Error> {
        let response = "{\n  \"kind\": \"storage#bucket\",\n  \"selfLink\": \"https://www.googleapis.com/storage/v1/b/test-bucket-test-create\",\n  \"id\": \"test-bucket-test-create\",\n  \"name\": \"test-bucket-test-create\",\n  \"projectNumber\": \"543254\",\n  \"metageneration\": \"1\",\n  \"location\": \"US-EAST1\",\n  \"storageClass\": \"STANDARD\",\n  \"etag\": \"CAE=\",\n  \"defaultEventBasedHold\": true,\n  \"timeCreated\": \"2023-05-03T16:44:38.911Z\",\n  \"updated\": \"2023-05-03T16:44:38.911Z\",\n  \"acl\": [\n    {\n      \"kind\": \"storage#bucketAccessControl\",\n      \"id\": \"test-bucket-test-create/allUsers\",\n      \"selfLink\": \"https://www.googleapis.com/storage/v1/b/test-bucket-test-create/acl/allUsers\",\n      \"bucket\": \"test-bucket-test-create\",\n      \"entity\": \"allUsers\",\n      \"role\": \"READER\",\n      \"etag\": \"CAE=\"\n    },\n    {\n      \"kind\": \"storage#bucketAccessControl\",\n      \"id\": \"test-bucket-test-create/project-owners-454645\",\n      \"selfLink\": \"https://www.googleapis.com/storage/v1/b/test-bucket-test-create/acl/project-owners-45645\",\n      \"bucket\": \"test-bucket-test-create\",\n      \"entity\": \"project-owners-456456\",\n      \"role\": \"OWNER\",\n      \"etag\": \"CAE=\",\n      \"projectTeam\": {\n        \"projectNumber\": \"45674\",\n        \"team\": \"owners\"\n      }\n    }\n  ],\n  \"defaultObjectAcl\": [\n    {\n      \"kind\": \"storage#objectAccessControl\",\n      \"entity\": \"allUsers\",\n      \"role\": \"READER\",\n      \"etag\": \"CAE=\"\n    }\n  ],\n  \"owner\": {\n    \"entity\": \"project-owners-4564\"\n  },\n  \"iamConfiguration\": {\n    \"bucketPolicyOnly\": {\n      \"enabled\": false\n    },\n    \"uniformBucketLevelAccess\": {\n      \"enabled\": false\n    },\n    \"publicAccessPrevention\": \"inherited\"\n  },\n  \"locationType\": \"region\"\n}\n";
        let response = serde_json::from_slice::<Response<Bucket>>(response.as_bytes());
        let response = response.expect("failed to map response as a response");

        let output = response.ok()?;
        assert_eq!(output.kind, "storage#bucket");
        Ok(())
    }

    #[test]
    fn test_try_impl_error() -> Result<(), Error> {
        let function = || {
            let response = r#"{"error":{"errors":[{"domain":"global","reason":"required","message":"Login Required","locationType":"header","location":"Authorization"}],"code":401,"message":"Login Required"}}"#;
            let response = serde_json::from_slice::<Response<Bucket>>(response.as_bytes());
            response?.ok()?;
            Ok::<(), Error>(())
        };
        let result = function();
        let value = format!("{:?}", result);
        println!("{}", value);
        assert_eq!(result.is_err(), true);
        Ok(())
    }
}
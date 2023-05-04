use std::ops::ControlFlow;

use super::{ErrorResponse};
use crate::Error;

#[derive(Debug, serde::Deserialize)]
#[serde(rename = "camelCase")]
#[serde(untagged)]
pub(crate) enum Response<T> {
    Success(T),
    Error(ErrorResponse),
}

/// Enable desugaring for `Response<T>`, e.g. the use of the `?` on an object of type `Response<T>`
/// ```ignore,no_run
/// if let Response::Error(error) = my_response {
///    return error;
/// }
/// let my_response = my_response.unwrap();
/// ```
/// becomes:
/// ```ignore,no_run
/// my_response?;
/// ```
impl<T> std::ops::Try for Response<T> {
    type Output = T;
    type Residual = Result<std::convert::Infallible, Error>;
    #[inline]
    fn from_output(output: Self::Output) -> Self {
        Response::Success(output)
    }
    #[inline]
    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Response::Success(t) => ControlFlow::Continue(t),
            Response::Error(error) => ControlFlow::Break(Err(Error::Google(error))),
        }
    }
}


impl<T> std::ops::FromResidual<Result<std::convert::Infallible, Error>> for Response<T> {
    #[inline]
    #[track_caller]
    fn from_residual(residual: <Self as std::ops::Try>::Residual) -> Self {
        if let Err(Error::Google(err)) = residual {
            Response::Error(err)
        } else {
            panic!("Non expected residual type encountered")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{models::{ErrorResponse, ErrorList}, Error, Bucket};

    use super::Response;

    #[test]
    fn test_try_impl() -> Result<(), Error> {
        let response = "{\n  \"kind\": \"storage#bucket\",\n  \"selfLink\": \"https://www.googleapis.com/storage/v1/b/test-bucket-test-create\",\n  \"id\": \"test-bucket-test-create\",\n  \"name\": \"test-bucket-test-create\",\n  \"projectNumber\": \"543254\",\n  \"metageneration\": \"1\",\n  \"location\": \"US-EAST1\",\n  \"storageClass\": \"STANDARD\",\n  \"etag\": \"CAE=\",\n  \"defaultEventBasedHold\": true,\n  \"timeCreated\": \"2023-05-03T16:44:38.911Z\",\n  \"updated\": \"2023-05-03T16:44:38.911Z\",\n  \"acl\": [\n    {\n      \"kind\": \"storage#bucketAccessControl\",\n      \"id\": \"test-bucket-test-create/allUsers\",\n      \"selfLink\": \"https://www.googleapis.com/storage/v1/b/test-bucket-test-create/acl/allUsers\",\n      \"bucket\": \"test-bucket-test-create\",\n      \"entity\": \"allUsers\",\n      \"role\": \"READER\",\n      \"etag\": \"CAE=\"\n    },\n    {\n      \"kind\": \"storage#bucketAccessControl\",\n      \"id\": \"test-bucket-test-create/project-owners-454645\",\n      \"selfLink\": \"https://www.googleapis.com/storage/v1/b/test-bucket-test-create/acl/project-owners-45645\",\n      \"bucket\": \"test-bucket-test-create\",\n      \"entity\": \"project-owners-456456\",\n      \"role\": \"OWNER\",\n      \"etag\": \"CAE=\",\n      \"projectTeam\": {\n        \"projectNumber\": \"45674\",\n        \"team\": \"owners\"\n      }\n    }\n  ],\n  \"defaultObjectAcl\": [\n    {\n      \"kind\": \"storage#objectAccessControl\",\n      \"entity\": \"allUsers\",\n      \"role\": \"READER\",\n      \"etag\": \"CAE=\"\n    }\n  ],\n  \"owner\": {\n    \"entity\": \"project-owners-4564\"\n  },\n  \"iamConfiguration\": {\n    \"bucketPolicyOnly\": {\n      \"enabled\": false\n    },\n    \"uniformBucketLevelAccess\": {\n      \"enabled\": false\n    },\n    \"publicAccessPrevention\": \"inherited\"\n  },\n  \"locationType\": \"region\"\n}\n";
        let response = serde_json::from_slice::<Response<Bucket>>(response.as_bytes());
        let response = response.expect("failed to map response as a response");

        let output = response?;
        assert_eq!(output.kind, "storage#bucket");
        Ok(())
    }

    #[test]
    fn test_try_impl_error() -> Result<(), Error> {
        let function = || {
            let response = Response::Error::<()>(ErrorResponse {
                error: ErrorList {
                    errors: Vec::new(),
                    code: 250,
                    message: "Some error occurred".to_string(),
                },
            });
            response?;
            Ok::<(), Error>(())
        };
        assert_eq!(function().is_err(), true);
        Ok(())
    }
}
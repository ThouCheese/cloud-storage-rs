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
/// ```no_run
/// if let Response::Error(error) = my_response {
///    return error;
/// }
/// let my_response = my_response.unwrap();
/// ```
/// becomes:
/// ```no_run
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
    use crate::{models::{ErrorResponse, ErrorList}, Error};

    use super::Response;

    #[test]
    fn test_try_impl() -> Result<(), Error> {
        let response = Response::Success(());
        let output = response?;
        assert_eq!(output, ());
        Ok(())
    }

    #[test]
    fn test_try_impl_error() -> Result<(), Error> {
        let response = Response::Error::<()>(ErrorResponse {
            error: ErrorList {
                errors: Vec::new(),
                code: 250,
                message: "Some error occurred".to_string(),
            },
        });
        let output = response?;
        assert_eq!(output, ());
        Ok(())
    }
}
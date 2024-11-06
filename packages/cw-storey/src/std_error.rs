/// A trait for converting *Storey* errors into [`cosmwasm_std::StdError`].
pub trait IntoStdError {
    /// Converts the error into a [`cosmwasm_std::StdError`] for use with CosmWasm.
    ///
    /// The error ends up as a [`cosmwasm_std::StdError::GenericErr`] with the error message
    /// being the result of calling `to_string` on the error.
    /// 
    /// # Example
    /// ```
    /// use cosmwasm_std::StdError;
    /// use storey::containers::map::key::ArrayDecodeError;
    /// use cw_storey::IntoStdError as _;
    ///
    /// let error = ArrayDecodeError::InvalidLength;
    /// assert_eq!(error.into_std_error(), StdError::generic_err(error.to_string()));
    /// ```
    fn into_std_error(self) -> cosmwasm_std::StdError;
}

impl<T> IntoStdError for T
where
    T: storey::error::StoreyError,
{
    fn into_std_error(self) -> cosmwasm_std::StdError {
        cosmwasm_std::StdError::generic_err(self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::StdError;
    use storey::error::StoreyError;

    use super::*;

    #[derive(Debug)]
    struct MockError {
        msg: String,
    }

    impl std::fmt::Display for MockError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.msg)
        }
    }

    impl StoreyError for MockError {}

    #[test]
    fn test_into_std_error() {
        let error = MockError {
            msg: "An error occurred".to_string(),
        };
        let std_error: StdError = error.into_std_error();
        assert_eq!(std_error, StdError::generic_err("An error occurred"));
    }
}

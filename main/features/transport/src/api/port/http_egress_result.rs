//! Result type for HTTP outbound operations.

use super::http_egress_error::HttpEgressError;

/// Result type for HTTP outbound operations.
pub type HttpEgressResult<T> = Result<T, HttpEgressError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_egress_result_ok_wraps_value() {
        let result: HttpEgressResult<u32> = Ok(42);
        if let Ok(val) = result {
            assert_eq!(val, 42);
        } else {
            panic!("expected Ok");
        }
    }

    #[test]
    fn test_http_egress_result_err_wraps_error() {
        let result: HttpEgressResult<u32> =
            Err(HttpEgressError::ConnectionFailed("refused".into()));
        if let Err(err) = result {
            assert!(err.to_string().contains("refused"));
        } else {
            panic!("expected Err");
        }
    }
}

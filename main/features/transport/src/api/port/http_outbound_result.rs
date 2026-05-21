//! Result type for HTTP outbound operations.

use super::http_outbound_error::HttpOutboundError;

/// Result type for HTTP outbound operations.
pub type HttpOutboundResult<T> = Result<T, HttpOutboundError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_outbound_result_ok_wraps_value() {
        let result: HttpOutboundResult<u32> = Ok(42);
        if let Ok(val) = result {
            assert_eq!(val, 42);
        } else {
            panic!("expected Ok");
        }
    }

    #[test]
    fn test_http_outbound_result_err_wraps_error() {
        let result: HttpOutboundResult<u32> =
            Err(HttpOutboundError::ConnectionFailed("refused".into()));
        if let Err(err) = result {
            assert!(err.to_string().contains("refused"));
        } else {
            panic!("expected Err");
        }
    }
}

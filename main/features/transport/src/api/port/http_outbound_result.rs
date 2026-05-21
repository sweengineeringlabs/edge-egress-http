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
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_http_outbound_result_err_wraps_error() {
        let result: HttpOutboundResult<u32> =
            Err(HttpOutboundError::ConnectionFailed("refused".into()));
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("refused"), "got: {msg:?}");
    }
}

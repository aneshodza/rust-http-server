pub const BAD_REQUEST: &str = r#"HTTP/1.1 400 Bad Request
Server: Anes HTTP
Content-Type: text/plain
Connection: close

The request could not be understood by the server due to malformed syntax."#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bad_request_contains_code() {
        assert!(BAD_REQUEST.contains("400 Bad Request"), "The bad request response does not contain the 400 Bad Request code");
    }
}

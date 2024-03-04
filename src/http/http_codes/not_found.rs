/// This contains the 404 Not Found response
pub const NOT_FOUND: &str = r#"HTTP/1.1 404 Not Found
Server: Anes HTTP
Content-Type: text/html
Content-Encoding: gzip
Connection: close
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bad_request_contains_code() {
        assert!(NOT_FOUND.contains("404 Not Found"), "The bad request response does not contain the 404 Not Found code");
    }
}

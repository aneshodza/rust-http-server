/// This holds the static part of an 200 OK response
pub const OK: &str = r#"HTTP/1.1 200 OK
Server: Anes HTTP
Content-Type: text/html; charset=utf-8
Content-Encoding: gzip
Connection: Keep-Alive
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok_contains_code() {
        assert!(OK.contains("200 OK"), "The ok response does not contain the 200 OK code");
    }
}

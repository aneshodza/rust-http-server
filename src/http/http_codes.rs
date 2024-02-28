use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::prelude::*;
use std::net::TcpStream;

mod bad_request;
mod ok;

pub fn bad_request() -> &'static [u8] {
    bad_request::BAD_REQUEST.as_bytes()
}

pub fn ok(mut stream: &TcpStream, data: String) {
    let mut response = ok::OK.to_string();
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(data.as_bytes())
        .expect("Failed to write data to gzip encoder");
    let compressed_data = encoder.finish().expect("Failed to compress data");

    response.push_str(&format!(
        "Content-Length: {}\r\n\r\n",
        compressed_data.len()
    ));
    let _ = stream.write_all(response.as_bytes());

    let _ = stream.write_all(&compressed_data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bad_request_contains_code() {
        let response = String::from_utf8(bad_request().to_vec()).expect("Something went wrong");
        assert!(
            response.contains("400 Bad Request"),
            "The bad request response does not contain the 400 Bad Request code"
        );
    }
}

use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;

mod bad_request;
mod not_found;
mod ok;

/// This function writes a 400 Bad Request response to the client. The 400 file served is under
/// `public/400.html`.
/// If the file is not found, the response is a simple string.
pub fn bad_request(stream: &TcpStream) {
    err_handler(stream, "static/400.html", "400 - Bad Request", bad_request::BAD_REQUEST.to_string())
}

pub fn not_found(stream: &TcpStream) {
    err_handler(stream, "static/404.html", "404 - Not Found", not_found::NOT_FOUND.to_string())

}

/// This function sends a 200 OK response to the client. It does so by sending the header and body
/// separately.
pub fn ok(mut stream: &TcpStream, data: Vec<u8>, mime_type: String) {
    let mut response = ok::OK.to_string();
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(&data)
        .expect("Failed to write data to gzip encoder");
    let compressed_data = encoder.finish().expect("Failed to compress data");

    response.push_str(&format!(
        "Content-Length: {}\r\nContent-Type: {}\r\n\r\n",
        compressed_data.len(),
        mime_type
    ));

    let _ = stream.write_all(response.as_bytes());
    let _ = stream.write_all(&compressed_data);
}

/// This is the generic error handler for all error responses. It sends the response base and the
/// compressed data to the client.
fn err_handler(mut stream: &TcpStream, filename: &'static str, default_html: &'static str, response_base: String) {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    let html_path = Path::new(filename);
    let mut html_content = default_html.to_string();

    if html_path.exists() {
        html_content = std::fs::read_to_string(html_path).expect("Failed to open error html");
    }
    let mut response: String = response_base;
    encoder
        .write_all(&html_content.as_bytes())
        .expect("Failed to write data to gzip encoder");
    let compressed_data = encoder.finish().expect("Failed to compress data");

    response.push_str(&format!(
        "Content-Length: {}\r\n\r\n",
        compressed_data.len()
    ));

    println!("{}", response);

    let _ = stream.write_all(response.as_bytes());
    let _ = stream.write_all(&compressed_data);
}

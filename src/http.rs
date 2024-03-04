use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};

use mime_guess::Mime;

mod http_codes;
mod http_object;

/// This is the internal request gate, which writes everything but the 400 Bad Request HTTP
/// Response to the client.
fn internal_request_gate(stream: &TcpStream) -> Result<(), String> {
    println!("New connection from: {}", stream.peer_addr().unwrap());
    let mut buffer = [0; 1024];
    let mut mutable_stream = stream.try_clone().unwrap();

    match mutable_stream.read(&mut buffer) {
        Ok(0) => Err("No data received from the client.".to_string()),
        Ok(_) => {
            println!("Received data: \n{}", String::from_utf8_lossy(&buffer));
            let request = request_tokenizer(&String::from_utf8_lossy(&buffer));
            if !request.is_http() {
                println!("This is not an http request");
                return Err("This is not an http request".to_string());
            }

            let req_path = request.request_path();
            let mime = request.weighted_mimes();
            let fileresult;
            match mime {
                Ok(unwrapped_mime) => {
                    fileresult = file_browser(req_path, unwrapped_mime);
                }
                Err(e) => return Err(e),
            }
            match fileresult {
                Some(fileresult) => {
                    http_codes::ok(stream, fileresult.0, fileresult.1);
                    Ok(())
                }
                None => {
                    http_codes::not_found(stream);
                    Ok(())
                }
            }
        }
        Err(e) => Err(format!("Failed to receive data: {}", e)),
    }
}

/// This is the request gate function used by a TCP-Server to handle incoming requests. It directly
/// writes the HTTP-Response to the client.
pub fn request_gate(mut stream: TcpStream) {
    let to_be_sent_response = internal_request_gate(&mut stream);
    match to_be_sent_response {
        Ok(_) => {
            println!("The response was sent");
        }
        Err(e) => {
            println!("Request handling gave an error: {}", e);
            http_codes::bad_request(&stream);
        }
    }
}

/// This function tokenizes the incoming http request and returns a struct that contains every
/// necessary attribute
///
/// # Returns
///
/// Returns an instance of the HttpResponse struct
fn request_tokenizer(request: &str) -> http_object::HttpObject {
    let lines: Vec<&str> = request.split("\r\n").collect();

    let mut request_line = String::new();
    let mut accept_header = String::new();

    for line in &lines {
        if line.starts_with("GET") {
            request_line = line.to_string();
        } else if line.starts_with("Accept:") || line.starts_with("accept:") {
            accept_header = line.to_string();
            break; // Assuming we only need the first Accept header.
        }
    }

    http_object::HttpObject::new(request_line, accept_header)
}

/// This function searches for a matching file in the file system and returns the file if it
/// exists
fn file_browser(filepath: &str, accepted_mimes: Vec<(String, f32)>) -> Option<(Vec<u8>, String)> {
    let base_path = "public".to_string();
    let parsed_filepath = parse_filepath(filepath);
    let path_pattern = base_path + &parsed_filepath;
    let mut matching_files: Vec<(PathBuf, f32)> = Vec::new();

    for entry in glob::glob(&path_pattern).unwrap() {
        append_to_matching_files(&mut matching_files, entry, accepted_mimes.clone());
    }

    if matching_files.len() == 0 {
        return None;
    }

    let used_file = best_match(matching_files);
    let used_filepath = used_file.to_str().unwrap().to_string();
    let used_mime = mime_guess::from_path(used_filepath).first_or_octet_stream();
    let file_content = std::fs::read(used_file);
    match file_content {
        Ok(content) => {
            return Some((content, correct_mime(used_mime, accepted_mimes)));
        }
        Err(e) => {
            println!("Error: {}", e);
            return None;
        }
    }
}

/// This function parses the filepath and returns a string that can be used to search for the file in
/// the file system
///
/// # Parameters
///
/// - `filepath`: This is the filepath that is to be parsed
///
/// # Returns
///
/// Returns a `String` that can be used to search for the file in the file system
fn parse_filepath(filepath: &str) -> String {
    let mut parsed_filepath = filepath.to_string();
    // TODO: Check if allowing ../ is a security risk
    if parsed_filepath == "/" {
        parsed_filepath = "/index.*".to_string();
    } else if parsed_filepath.contains('.') {
        parsed_filepath = format!("/{}", parsed_filepath);
    } else {
        parsed_filepath = format!("{}*", parsed_filepath);
    }
    parsed_filepath
}

/// This function appends the matching files to the matching_files vector
///
/// # Parameters
///
/// - `matching_files`: This is a reference to a vector of tuples that contain the file path and the
/// - `entry`: This is the result of the glob search
/// - `accepted_mimes`: This is a vector of tuples that contain the mime type and the weight
fn append_to_matching_files(
    matching_files: &mut Vec<(PathBuf, f32)>,
    entry: glob::GlobResult,
    accepted_mimes: Vec<(String, f32)>,
) {
    match entry {
        Ok(path) => {
            if let Some(file_extension) = path.extension() {
                let mime = file_extension.to_str().unwrap_or_default().to_string();
                let mut weight = default_weight(&accepted_mimes);

                if let Some(w) = find_weight_for_mime(mime, &accepted_mimes) {
                    weight = w;
                }

                println!("Serving file: {:?}", path);
                matching_files.push((path, weight));
            }
        }
        Err(e) => println!("{:?}", e),
    }
}

/// This function returns the default weight for the accept attribute
///
/// # Parameters
///
/// - `mimes`: This is a reference to a vector of tuples that contain the mime type and the weight
///
/// # Returns
///
/// Returns a `f32` that is the default weight
fn default_weight(mimes: &Vec<(String, f32)>) -> f32 {
    for (m, w) in mimes {
        if m == "*/*" {
            return w.clone();
        }
    }
    0.0
}

/// This function finds the weight for a specific mime type
///
/// # Parameters
///
/// -`extension`: This is the extension of the file
/// -`mimes`: This is a reference to a vector of tuples that contain the mime type and the weight
///
/// # Returns
///
/// The function returns an `Option<f32>` that is the weight of the mime type
fn find_weight_for_mime(extension: String, mimes: &Vec<(String, f32)>) -> Option<f32> {
    for (m, w) in mimes {
        let current_extension = m.split("/").collect::<Vec<&str>>()[1];
        if extension == current_extension {
            return Some(w.clone());
        }
    }
    None
}

/// This function returns the best matching file
///
/// # Parameters
///
/// - `matching_files`: This is a vector of tuples that contain the file path and the weight
///
/// # Returns
///
/// This function returns a `PathBuf` that is the best matching file
fn best_match(matching_files: Vec<(PathBuf, f32)>) -> PathBuf {
    let mut best_match = PathBuf::new();
    let mut best_weight = -1.0;

    for matching_file in matching_files {
        if matching_file.1 > best_weight {
            best_match = matching_file.0.to_path_buf();
            best_weight = matching_file.1;
        }
    }

    best_match.to_path_buf()
}

fn correct_mime(mime: Mime, accepted_mimes: Vec<(String, f32)>) -> String {
    let mime_string = mime.to_string();
    for (m, _) in accepted_mimes {
        if m.eq(&mime_string) {
            return mime_string;
        }
    }
    "*/*".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tcp;

    use std::thread;
    use tokio::test as tokio_test;

    #[tokio_test]
    async fn test_ok_writes_ok() -> Result<(), reqwest::Error> {
        let listener = tcp::spawn_tcp_server("127.0.0.1:0");

        let _port = listener
            .local_addr()
            .expect("Failed to get the local address")
            .port();

        thread::spawn(move || {
            tcp::handle_incoming_connections(listener, &request_gate);
        });

        let client = reqwest::Client::new();

        let res = client
            .get(format!("http://127.0.0.1:{}", _port.to_string()))
            .send()
            .await?;

        assert!(res.status().is_success(), "The response was not successful");
        Ok(())
    }

    #[test]
    fn test_malformed_request_triggers_bad_request() -> std::io::Result<()> {
        use std::io::{Read, Write};
        use std::net::TcpStream;

        let address = "127.0.0.1:0";
        let listener = tcp::spawn_tcp_server(address);
        let port = listener.local_addr().unwrap().port();

        std::thread::spawn(move || {
            tcp::handle_incoming_connections(listener, &request_gate);
        });

        let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port))?;

        stream.write_all(b"NOT HTTP DATA\r\n\r\n")?;

        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer)?;

        let headers = String::from_utf8_lossy(&buffer);

        assert!(
            headers.contains("400 Bad Request"),
            "The response does not contain the expected 400 Bad Request status"
        );
        Ok(())
    }
}

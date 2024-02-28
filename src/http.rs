use std::io::{Read, Write};
use std::net::TcpStream;

mod http_codes;

/// This is the internal request gate, which writes everything but the 400 Bad Request HTTP
/// Response to the client.
fn internal_request_gate(stream: &TcpStream) -> Result<(), String> {
    println!("New connection from: {}", stream.peer_addr().unwrap());
    let mut buffer = [0; 1024];
    let mut mutable_stream = stream.try_clone().unwrap();

    match mutable_stream.read(&mut buffer) {
        Ok(0) => Err("No data received from the client.".to_string()),
        Ok(_) => {
            let request = request_tokenizer(&String::from_utf8_lossy(&buffer));
            if !request.is_http() {
                println!("This is not an http request");
                return Err("This is not an http request".to_string());
            }
            http_codes::ok(stream, "Hello, world!".to_string());
            Some(":)".to_string());
            Ok(())
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
            let _ = stream.write(http_codes::bad_request());
        }
    }
}

/// This function tokenizes the incoming http request and returns a struct that contains every
/// necessary attribute
///
/// # Returns
///
/// Returns an instance of the HttpResponse struct
fn request_tokenizer(request: &str) -> HttpRequest {
    let lines: Vec<&str> = request.split("\r\n").collect();
    HttpRequest {
        request: lines[0].to_string(),
        // accept: lines[3].to_string(),
        // accept_encoding: lines[5].to_string(),
    }
}

/// This struct is used to store the attributes of the incoming http request
struct HttpRequest {
    request: String,
    // accept: String,
    // accept_encoding: String,
}

/// This is the implementation of the HttpResponse. It gives the user methods to more easily
/// interact with the HttpResponse struct by giving helper functions
impl HttpRequest {
    fn is_http(&self) -> bool {
        self.request.split_whitespace().collect::<Vec<&str>>()[2] == "HTTP/1.1"
    }
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

        println!("{:?}", res);

        assert!(res.status().is_success(), "The response was not successful");
        Ok(())
    }

    #[test]
    fn test_malformed_request_triggers_bad_request() -> std::io::Result<()> {
        use std::io::{Read, Write};
        use std::net::TcpStream;

        let address = "127.0.0.1:0"; // Use the actual address and port
        let listener = tcp::spawn_tcp_server(address);
        let port = listener.local_addr().unwrap().port();

        std::thread::spawn(move || {
            tcp::handle_incoming_connections(listener, &request_gate);
        });

        // Connect to the server
        let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port))?;

        // Send non-HTTP data
        stream.write_all(b"NOT HTTP DATA\r\n\r\n")?;

        // Read the response
        let mut response = String::new();
        stream.read_to_string(&mut response)?;

        // Check if the response matches the expected BAD_REQUEST
        assert!(
            response.contains("400 Bad Request"),
            "The response does not contain the expected 400 Bad Request status"
        );

        Ok(())
    }
}

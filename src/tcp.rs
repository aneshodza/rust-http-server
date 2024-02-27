use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

const TCP_ADDRESS: &str = "127.0.0.1:7878";
const RECONNECT_TRIES: u8 = 5;

/// Attempts to spawn a TCP-Server to port 7878. It retries 5 times, after which the function
/// panics.
/// 
/// # Parameters
///
/// - `tries`: This is the current try, which the function is on. Normally `0` would be passed, as
/// the function handles the incrementing recursively.
///
/// # Returns
///
/// Returns the TCP-Server as a `TcpListener` object, from which further operations can be
/// performed.
///
/// # Errors
///
/// In case the function fails to bind to port 7878 five times it panics and also prints out the
/// reason for not being able to.
fn internal_spawn_tcp_server(tries: u8, tcp_address: &str) -> TcpListener {
    match TcpListener::bind(tcp_address) {
        Ok(listener) => listener,
        Err(e) if tries < RECONNECT_TRIES => {
            eprintln!("Failed to bind to port. Reason: {}", e);
            println!("Trying {} more times", RECONNECT_TRIES - tries);
            thread::sleep(Duration::from_secs(5));
            internal_spawn_tcp_server(tries + 1, tcp_address)
        },
        Err(e) => {
            panic!("Couldn't bind to port! Reason: {}", e);
        }
    }
}

/// Attempts to spawn a TCP-Server to port 7878. It retries 5 times, after which the function
/// panics.
///
/// # Returns
///
/// Returns the TCP-Server as a `TcpListener` object, from which further operations can be
/// performed.
/// 
/// # Errors
///
/// In case the function fails to bind to port 7878 five times it panics and also prints out the
/// reason for not being able to.
pub fn spawn_tcp_server() -> TcpListener {
    internal_spawn_tcp_server(0, "127.0.0.1:7878")
}

/// This function handles the traffic that comes into the TcpServer and spawns a new thread for
/// every incoming connection.
///
/// # Parameters
///
/// - `listener`: This is a `TcpListener` object. Ideally this is spawned from the
/// `spawn_tcp_server()` function.
///
/// - `resolved`: This is the function that handles the actual business logic of every incoming
/// connection. The functions parameters should be a simple `TcpStream` object.
pub fn handle_incoming_connections(listener: TcpListener, responder: &dyn Fn(TcpStream)) {
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        responder(stream);
        println!("Connection established!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::io::Write;

    #[test]
    fn test_spawn_server() {
        let listener = internal_spawn_tcp_server(0, "127.0.0.1:0");
        assert!(listener.local_addr().is_ok(), "Listener should have a valid local address");
    }

    #[test]
    fn test_panic_return() {
        // This purposefully occupies the port our server wants to connect to, so when we spawn the
        // server it fails.
        let _guard = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind the initial listener");

        let _port = _guard.local_addr().expect("Failed to get the local address").port();

        let result = std::panic::catch_unwind(|| {
            internal_spawn_tcp_server(5, ("127.0.0.1:".to_owned() + &_port.to_string()).as_str());
        });

        assert!(result.is_err(), "The binding should have paniced, as it cannot bind to the port but it didn't");
    }

    #[test]
    fn test_check_if_data_arrives() {
        let (tx, rx) = mpsc::channel();
        let listener = internal_spawn_tcp_server(0, "127.0.0.1:0");

        let port = listener.local_addr().expect("Failed to get local address").port();
        thread::spawn(move || {
            handle_incoming_connections(listener, &|_stream| {
                println!("Connection established!");
                tx.send(true).expect("Failed to send success signal");
            })
        });

        let mut stream = TcpStream::connect(("127.0.0.1:".to_owned() + &port.to_string()).as_str()).expect("Failed to create tcp client");
        let request = "Hello, server!";
        stream.write_all(request.as_bytes());
        thread::sleep(Duration::from_secs(5));
        match rx.recv_timeout(Duration::from_secs(5)) {
            Ok(_) => println!("Test succeeded, connection was established and handled."),
            Err(e) => panic!("Test failed: {:?}", e),
        }
    }
}

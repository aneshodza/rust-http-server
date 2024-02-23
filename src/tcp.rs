use std::net::TcpListener;
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
fn internal_spawn_tcp_server(tries: u8) -> TcpListener {
    match TcpListener::bind(TCP_ADDRESS) {
        Ok(listener) => listener,
        Err(e) if tries < RECONNECT_TRIES => {
            eprintln!("Failed to bind to port 7878. Reason: {}", e);
            println!("Trying {} more times", RECONNECT_TRIES - tries);
            thread::sleep(Duration::from_secs(5));
            internal_spawn_tcp_server(tries + 1)
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
    internal_spawn_tcp_server(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_server() {
        let listener = spawn_tcp_server();
        assert!(listener.local_addr().is_ok(), "Listener should have a valid local address");
    }

    #[test]
    fn test_panic_return() {
        // This purposefully occupies the port our server wants to connect to, so when we spawn the
        // server it fails.
        let _guard = std::net::TcpListener::bind(TCP_ADDRESS).expect("Failed to bind the initial listener");

        let result = std::panic::catch_unwind(|| {
            internal_spawn_tcp_server(4)
        });

        assert!(result.is_err(), "The binding should have paniced, as it cannot bind to the port but it didn't");
    }
}

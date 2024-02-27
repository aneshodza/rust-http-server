use std::{env, io::Read, net::TcpStream};
use dotenv::dotenv;

mod utils;
mod tcp;

fn main() {
    dotenv().ok();

    utils::greet_user();
  
    println!("The server will run on {}", env::var("TCP_ADDRESS").unwrap());
    let listener = tcp::spawn_tcp_server();
    tcp::handle_incoming_connections(listener, &responder);
}

/// This is a TEMPORARY function to handle the incoming connections. This will be replaced with a
/// proper handler.
fn responder(stream: TcpStream) {
    println!("New connection from: {}", stream.peer_addr().unwrap());
    let mut buffer = [0; 1024]; // A buffer to store incoming data.
    let mut mutable_stream = stream.try_clone().unwrap();

    match mutable_stream.read(&mut buffer) {
        Ok(0) => {
            // No data was received
            println!("No data received from the client.");
        },
        Ok(_) => {
            // Data was received, process it
            println!("Data received: {}", String::from_utf8_lossy(&buffer));
            // Respond to the client, if necessary
        },
        Err(e) => {
            // An error occurred
            println!("Failed to receive data: {}", e);
        },
    }
}

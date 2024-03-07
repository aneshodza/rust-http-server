use std::env;
use dotenv::dotenv;

mod utils;
mod tcp;
mod http;

fn main() {
    dotenv().ok();

    utils::greet_user();
  
    println!("The server will run on {}", env::var("TCP_ADDRESS").unwrap());
    let listener = tcp::spawn_tcp_server("127.0.0.1:7878");
    tcp::handle_incoming_connections(listener, &http::request_gate);
}

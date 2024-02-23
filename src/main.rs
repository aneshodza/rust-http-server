use std::env;
use dotenv::dotenv;

mod utils;
mod tcp;

fn main() {
    dotenv().ok();

    utils::greet_user();
  
    println!("The server will run on {}", env::var("TCP_ADDRESS").unwrap());
    let _server = tcp::spawn_tcp_server();
}

mod utils;
mod tcp;

fn main() {
    println!("Hello, world!");
    utils::greet_user();
    let _server = tcp::spawn_tcp_server();
}

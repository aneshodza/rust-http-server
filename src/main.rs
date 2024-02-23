use std::env;
use dotenv::dotenv;

mod utils;

fn main() {
    dotenv().ok();

    utils::greet_user();

    println!("The server will run on {}", env::var("TCP_ADDRESS").unwrap());
}

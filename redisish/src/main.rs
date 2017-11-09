extern crate bufstream;

mod types;
mod server;

use std::net::TcpListener;
use server::Redisish;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    listener.set_ttl(10).expect("could not set TTL");
    let mut redisish_server = Redisish::default();
    println!("Server started at 127.0.0.1:8888");
    // accept connections and process them serially
    for stream in listener.incoming() {
        println!("New connection");
        redisish_server.handle_client(stream.unwrap());
    }
}

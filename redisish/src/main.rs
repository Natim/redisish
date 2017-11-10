extern crate bufstream;
mod types;
mod server;

use server::Redisish;
use std::net::TcpListener;
use std::thread;
use std::sync::Arc;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").expect("Couldn't bind 127.0.0.1:8888");
    listener.set_ttl(10).expect("could not set TTL");
    println!("Server started at 127.0.0.1:8888");
    let mut redisish_server = Arc::new(Redisish::new());
    // accept connections and process them serially
    for stream in listener.incoming() {
        println!("New connection");
        thread::spawn(move || {
            let server = Arc::clone(&redisish_server);
            server.handle_client(stream.expect("Stream is not usable"));
        });
    }
}

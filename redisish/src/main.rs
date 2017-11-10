extern crate bufstream;
mod types;
mod server;
use server::{Redisish, handle_client};
use std::net::TcpListener;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").expect("Couldn't bind 127.0.0.1:8888");
    listener.set_ttl(10).expect("could not set TTL");
    println!("Server started at 127.0.0.1:8888");
    let redisish_server = Arc::new(Mutex::new(Redisish::default()));
    // accept connections and process them serially
    for stream in listener.incoming() {
        println!("New connection");
        match stream {
            Ok(stream) => {
                let mut server = Arc::clone(&redisish_server);
                thread::spawn(move || {
                    handle_client(&mut server, stream);
                });
            },
            Err(_) => {
                println!("Connection closed.");
            }
        }
    }
}

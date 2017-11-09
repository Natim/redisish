extern crate bufstream;

use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, Write};
use std::collections::VecDeque;
use bufstream::BufStream;


enum Message {
    Get,
    Put(String),
    Invalid(String),
}

#[derive(Default)]
struct Redisish {
    messages: VecDeque<String>,
}

impl Redisish {
    fn handle_message(&mut self, content: String) -> Message {
        let content = String::from(content.trim());
        if content.to_lowercase() == String::from("get") {
            Message::Get
        } else if content.to_lowercase().starts_with("put ") {
            let mut args: Vec<&str> = content.trim().split_whitespace().collect();
            args.remove(0);
            Message::Put(args.join(" "))
        } else {
            Message::Invalid(content)
        }
    }
    
    fn handle_client(&mut self, stream: TcpStream) {
        let mut stream = BufStream::new(stream);

        loop {
            let mut content = String::new();
            let line = stream.read_line(&mut content);

            match line {
                Ok(_) => {
                    if content.len() == 0 {
                        break;
                    }
                    let message = self.handle_message(content);

                    match message {
                        Message::Get => {
                            match self.messages.pop_front() {
                                Some(value) => {
                                    stream.write(&value.as_bytes()).unwrap();
                                    stream.write(b"\n").unwrap();
                                },
                                None => {
                                    stream.write(b"- Empty Queue\n").unwrap();
                                }
                            }
                        },
                        Message::Put(value) => {
                            self.messages.push_front(value);
                            stream.write(b"+ OK\n").unwrap();
                        },
                        Message::Invalid(content) => {
                            stream.write(b"- Unknown command: ").unwrap();
                            stream.write(content.as_bytes()).unwrap();
                            stream.write(b"\n").unwrap();
                        }
                    }
                    match stream.flush() {
                        Err(_) => {
                            break;
                        },
                        _ => {}
                    }
                },
                Err(_) => {
                    break;
                }
            }
        }
    }
}


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

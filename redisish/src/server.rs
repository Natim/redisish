use std::net::TcpStream;
use std::io::{BufRead, Write};
use std::collections::HashMap;
use std::collections::VecDeque;
use bufstream::BufStream;
use types::Channel;
use types::Value;
use types::Message;


#[derive(Default)]
pub struct Redisish {
    channels: HashMap<Channel, VecDeque<Value>>,
}

impl Redisish {
    fn handle_message(&mut self, content: String) -> Message {
        let content = String::from(content.trim());
        let mut parts = content.splitn(3, ' ');
        
        match (parts.next(), parts.next(), parts.next()) {
            (Some("RETRIEVE"), None, None) => {
                Message::Retrieve(String::from("default"))
            },
            (Some("RETRIEVE"), Some(channel), None) => {
                Message::Retrieve(String::from(channel))
            },
            (Some("PUSH"), Some(value), None) => {
                Message::Push(String::from("default"), String::from(value))
            },
            (Some("PUSH"), Some(channel), Some(value)) => {
                Message::Push(String::from(channel), String::from(value))
            },
            (_, _, _) => Message::Invalid(content.clone())
        }
    }
    
    pub fn handle_client(&mut self, stream: TcpStream) {
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
                        Message::Retrieve(channel) => {
                            let mut queue = self.channels.entry(String::from(channel))
                                .or_insert(VecDeque::new());

                            match queue.pop_front() {
                                Some(value) => {
                                    stream.write(&value.as_bytes()).unwrap();
                                    stream.write(b"\n").unwrap();
                                },
                                None => {
                                    stream.write(b"- Empty Queue\n").unwrap();
                                }
                            }
                        },
                        Message::Push(channel, value) => {
                            let mut queue = self.channels.entry(String::from(channel))
                                .or_insert(VecDeque::new());
                            queue.push_front(value);
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

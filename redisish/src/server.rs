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
                    let message = Message::from(content);

                    match message {
                        Message::Retrieve(channel) => {
                            let mut queue = self.channels.entry(channel.clone())
                                .or_insert(VecDeque::new());

                            match queue.pop_front() {
                                Some(value) => {
                                    write!(stream, "+ {}\n", value).unwrap();
                                },
                                None => {
                                    write!(stream, "- Queue `{}` is empty.\n", channel).unwrap();
                                }
                            }
                        },
                        Message::Push(channel, value) => {
                            let mut queue = self.channels.entry(channel)
                                .or_insert(VecDeque::new());
                            queue.push_front(value);
                            stream.write(b"+ OK\n").unwrap();
                        },
                        Message::Invalid(content) => {
                            write!(stream, "- Unknown command: {}\n", content).unwrap();
                        }
                    }
                    match stream.flush() {
                        Err(_) => break,
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

use std::net::TcpStream;
use std::io::{BufRead, Write};
use std::collections::VecDeque;
use bufstream::BufStream;
use types::Message;


#[derive(Default)]
pub struct Redisish {
    messages: VecDeque<String>,
}

impl Redisish {
    fn handle_message(&mut self, content: String) -> Message {
        let content = String::from(content.trim());
        let mut parts = content.splitn(2, ' ');
        
        let (command, value) = (parts.next(), parts.next());

        match (command, value) {
            (Some("RETRIEVE"), None) => {
                Message::Retrieve
            },
            (Some("PUSH"), Some(value)) => {
                Message::Push(String::from(value))
            },
            (_, _) => Message::Invalid(content.clone())
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
                        Message::Retrieve => {
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
                        Message::Push(value) => {
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

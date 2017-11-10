use std::io::{BufRead, Write};
use std::net::TcpStream;
use std::collections::HashMap;
use std::collections::VecDeque;
use bufstream::BufStream;
use types::Channel;
use types::Value;
use types::Message;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Default)]
pub struct Redisish {
    channels: HashMap<Channel, VecDeque<Value>>,
}

impl Redisish {

    pub fn command(&mut self, message: Message) -> String {
        match message {
            Message::Retrieve(channel) => {
                let mut queue = self.channels.entry(channel.clone())
                    .or_insert(VecDeque::new());
                match queue.pop_front() {
                    Some(value) => {
                        format!("+ {}\n", value)
                    },
                    None => {
                        format!("- Queue `{}` is empty.\n", channel)
                    }
                }
            },
            Message::Push(channel, value) => {
                let mut queue = self.channels.entry(channel.clone())
                    .or_insert(VecDeque::new());
                queue.push_front(value);
                format!("+ OK\n")
            },
            Message::Invalid(content) => {
                format!("- Unknown command: {}\n", content)
            }
        }
    }
}

pub fn handle_client(server: &mut Arc<Mutex<Redisish>>, stream: TcpStream) {
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
                let mut s = server.lock().unwrap();
                let response = s.command(message);
                stream.write(response.as_bytes()).unwrap();
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

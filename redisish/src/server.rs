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

type Response = String;

#[derive(Default)]
pub struct Redisish {
    channels: HashMap<Channel, VecDeque<Value>>,
}

impl Redisish {
    pub fn command(&mut self, message: Message) -> Response {
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
                let mut s = server.lock().expect("Was enable to lock the database mutex,
                                                  it was poisoned");
                let response = s.command(message);
                match stream.write(response.as_bytes()).and_then(|_| stream.flush()) {
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


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_retrieve_command_on_emtpy_channel() {
        let mut redisish = Redisish::default();
        let response = redisish.command(Message::from(String::from("RETRIEVE\n")));
        assert_eq!(response, "- Queue `default` is empty.\n");
    }

    #[test]
    fn test_retrieve_command_return_last_added_entry() {
        let mut redisish = Redisish::default();
        redisish.command(Message::from(String::from("PUSH remy\n")));
        redisish.command(Message::from(String::from("PUSH mago\n")));
        let response = redisish.command(Message::from(String::from("RETRIEVE\n")));
        assert_eq!(response, "+ mago\n");
        let response = redisish.command(Message::from(String::from("RETRIEVE\n")));
        assert_eq!(response, "+ remy\n");
    }

    #[test]
    fn test_invalid_command_return_error_message() {
        let mut redisish = Redisish::default();
        let response = redisish.command(Message::from(String::from("INVALID\n")));
        assert_eq!(response, "- Unknown command: INVALID\n");
    }
}

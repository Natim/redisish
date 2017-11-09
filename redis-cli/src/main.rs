extern crate bufstream;

use std::io::{BufRead, Write, stdin};
use std::net::TcpStream;
use bufstream::BufStream;

fn main() {
    let stream = TcpStream::connect("127.0.0.1:8888").unwrap();
    let mut stream = BufStream::new(stream);

    loop {
        println!("Please enter a RETRIEVE or PUSH command");
        
        let mut command = String::new();
        stdin().read_line(&mut command).unwrap();
        if command.len() == 0 {
            break;
        }
        println!("Sending: {:?}", command);

        stream.write(&command.as_bytes()).unwrap();
        stream.flush().unwrap();

        let mut result = String::new();
        stream.read_line(&mut result).unwrap();
        println!("Server answer: {:?}\n", result);
    }
}

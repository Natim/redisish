extern crate bufstream;
use std::error::{self, Error};
use std::fmt::{self, Display};
use std::io;
use std::io::{BufRead, Write, stdin};
use std::net::TcpStream;
use bufstream::BufStream;

const SERVER :&'static str = "127.0.0.1:8888";


type ConnectionResult<T> = Result<T, OperationError>;

#[derive(Debug)]
struct OperationError {
    kind: OperationErrorKind,
    message: String
}


impl Display for OperationError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Debug)]
pub enum OperationErrorKind
{
    ConnectionError,
    WriteError,
}

impl error::Error for OperationError
{
    fn description(&self) -> &str
    {
        &self.message
    }
}

impl From<io::Error> for OperationError
{
    fn from(e : io::Error) -> Self
    {
        OperationError { kind : OperationErrorKind::WriteError, message : String::from(e.description()) }
    }
}

type Response = String;


fn connect() -> ConnectionResult<TcpStream> {
    match TcpStream::connect(SERVER) {
        Err(error) => {
            Err(OperationError { kind : OperationErrorKind::ConnectionError, message : String::from(error.description()) })
        }
        Ok(connection) => Ok(connection)
    }
}

fn send_command(stream: &mut BufStream<TcpStream>, command: &String) -> ConnectionResult<Response> {
    stream.write(command.as_bytes())?;
    stream.flush()?;
    let mut result = String::new();
    stream.read_line(&mut result)?;
    if result.len() == 0 {
        Err(OperationError { kind : OperationErrorKind::WriteError, message : "Disconnected while reading".to_string() })
    } else {
        Ok(result)
    }
}


fn main() {
    let mut stream = match connect() {
        Ok(conn) => BufStream::new(conn),
        Err(error) => {
            println!("Server unreachable: {}", error);
            ::std::process::exit(1)
        }
    };

    loop {
        println!("Please enter a RETRIEVE or PUSH command");

        let mut command = String::new();
        match stdin().read_line(&mut command) {
            Ok(_) => {},
            Err(error) => {
                println!("stdin not readable: {}", error);
                ::std::process::exit(2)
            }

        }
        if command.clone().trim().len() == 0 {
            println!("Exiting");
            break;
        }

        for _ in 1..3 {
            match send_command(&mut stream, &command) {
                Ok(response) => {
                    println!("{}", response);
                    break
                },
                Err(error) => {
                    println!("Connection lost, trying to reconnect…");
                    stream = match connect() {
                        Err(_) => {
                            println!("Server is not running: {}", error);
                            break;
                        },
                        Ok(connection) => {
                            println!("Reconnecting");
                            BufStream::new(connection)
                        }
                    }
                }
            }
        }
    }
}

extern crate srv_commands;
extern crate srv_config;

use srv_commands::Command;
use srv_config::Config;
use std::collections::VecDeque;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use HashServerError::Parse;

#[derive(Debug)]
pub enum HashServerError {
    Parse(srv_commands::ParseError),
    Io(std::io::Error),
}

impl From<std::io::Error> for HashServerError {
    fn from(err: std::io::Error) -> HashServerError {
        HashServerError::Io(err)
    }
}

impl From<srv_commands::ParseError> for HashServerError {
    fn from(err: srv_commands::ParseError) -> HashServerError {
        HashServerError::Parse(err)
    }
}

fn handle_client(stream: &TcpStream, orders: &mut VecDeque<String>, v: bool) {
    let mut reader = BufReader::new(stream);
    let mut writer = BufWriter::new(stream);

    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(n) => {
                if n == 0 {
                    break;
                }

                let cmd = srv_commands::parse(&line).map_err(HashServerError::Parse);

                match cmd {
                    Ok(Command::Stage(str)) => {
                        orders.push_front(str);
                    }
                    Ok(Command::Retrieve) => {
                        if orders.is_empty() {
                            let _ = writer.write(b"No order on stage!\n");
                        } else {
                            if let Some(latest_order) = orders.pop_front() {
                                let _ = writer.write(latest_order.as_bytes());
                                let _ = writer.write(b"\n");
                            }
                        }
                    }
                    Ok(Command::Control(ref control_string)) => {
                        if v {
                            println!("Received Control: {}", control_string);
                        }
                    }
                    Err(Parse(e)) => {
                        println!("Error occurred: {:?}", e);
                    }
                    _ => {}
                }

                let _ = writer.flush();
            }
            _ => {
                break;
            }
        }
    }
}

pub fn main() {
    let c = Config::load();

    if c.verbosity > 0 {
        println!("Starting Multi Hash Server 0.1:");
        println!(
            "verbosity: {} | address: {} | port: {} | test-mode: {}",
            c.verbosity, c.address, c.port, c.testing
        );
    }

    let host = format!("{}:{}", c.address, c.port);

    let mut orders: VecDeque<String> = VecDeque::new();

    if c.testing {
        orders.push_front(String::from("Test3"));
        orders.push_front(String::from("Test2"));
        orders.push_front(String::from("Test1"));
    }

    match TcpListener::bind(host).map_err(HashServerError::Io) {
        Ok(listener) => {
            for s in listener.incoming() {
                if let Ok(stream) = s {
                    if c.verbosity > 1 {
                        println!("[DEBUG] New Client connected")
                    }
                    handle_client(&stream, &mut orders, c.verbosity > 0);
                    if c.verbosity > 1 {
                        println!("[DEBUG] Client diconnected")
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to start the MultiHashServer: {:?}", e);
        }
    }
}

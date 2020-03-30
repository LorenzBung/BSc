use std::env::args;
use std::process;

extern crate nix;

use nix::unistd::{fork, pipe, read, write};
use nix::sys::wait::{wait, WaitStatus};
use std::str;
use nix::unistd::ForkResult::{Child, Parent};
use std::os::unix::io::RawFd;

mod unit_test_pipe;

const BUFFER_SIZE: usize = 256;


fn main() {

    let arguments: Vec<String> = args().collect();

    if arguments.len() > 2 {
        if let Ok((reader, writer)) = pipe() {
            let numbers = arguments[1..].to_vec();

            let pid = fork();
            match pid {
                Ok(Child) => {
                    match read_from_pipe(reader) {
                        Ok(msg_received) => {
                            let vec = split_into_strings(&msg_received);

                            let pid = fork();
                            match pid {
                                Ok(Child) => {
                                    match sum_strings(vec) {
                                        Ok(res) => println!("Sum = {}", res),
                                        Err(e) => {
                                            println!("{}", e);
                                            process::exit(1);
                                        }
                                    }
                                }
                                Ok(Parent { .. }) => {
                                    if let Ok(ws) = wait() {
                                        if let WaitStatus::Exited(_, exit_code) = ws {
                                            match mul_strings(vec) {
                                                Ok(res) => println!("Mul = {}", res),
                                                Err(e) => {
                                                    println!("{}", e);
                                                    process::exit(1);
                                                }
                                            }
                                            process::exit(exit_code as i32);
                                        }
                                    }
                                }
                                Err(_) => println!("Fatal error: Fork failed"),
                            }

                        }
                        Err(_) => {}
                    }
                }

                Ok(Parent { .. }) => {
                    let mut args = String::new();
                    for s in numbers {
                        args = concatenate_strings(&args, &concatenate_strings(&s, " "));
                    }
                    println!("sending to childs: {}", args);

                    if let Err(_) = write(writer, args.as_bytes()) {
                        println!("Broken pipe")
                    }

                    if let Ok(ws) = wait() {
                        if let WaitStatus::Exited(_, exit_code) = ws {
                            process::exit(exit_code as i32);
                        }
                    }
                }

                Err(_) => {}
            }
        }
    } else {
        println!("Correct usage: number number <number> ...");
        process::exit(1)
    }

}

fn read_from_pipe(reader: RawFd) -> Result<String, String> {
    let mut read_buf = [0u8; BUFFER_SIZE];
    match read(reader, &mut read_buf) {
        Ok(bytes_read) => {
            match str::from_utf8(&read_buf[0..bytes_read]) {
                Ok(msg_received) => Ok(msg_received.to_string()),
                Err(_) => Err("Couldn't read".to_string()),
            }
        }
        Err(_) => Err("Couldn't read".to_string()),
    }
}

/// Concats the two given String *references* and returns them as a String.
fn concatenate_strings<'a>(s1: &'a str, s2: &'a str) -> String {
    s1.to_string() + s2
}

/// Splits the given String reference and returns it as Vector.
fn split_into_strings<'a>(s1: &'a str) -> Vec<String> {
    s1.to_string()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

/// Calculates the sum of the given Strings and returns them as a Result.
fn sum_strings(v: Vec<String>) -> Result<i64, String> {
    let mut sum: i64 = 0;
    for x in v {
        match x.parse::<i64>() {
            Ok(val) => {
                match i64::checked_add(sum, val) {
                    Some(y) => {
                        sum = y;
                    }
                    None => return Err("Overflow would happen in sum_strings()".to_string()),
                }
            }
            Err(_) => return Err("Given String is not a int".to_string()),
        }
    }
    Ok(sum)
}

/// Calculates the product of the given Strings and returns them as a Result.
fn mul_strings(v: Vec<String>) -> Result<i64, String> {
    let mut prod: i64 = 1;
    for x in v {
        match x.parse::<i64>() {
            Ok(val) => {
                match i64::checked_mul(prod, val) {
                    Some(y) => {
                        prod = y;
                    }
                    None => return Err("Overflow would happen in mul_strings()".to_string()),
                }
            }
            Err(_) => return Err("Given String is not a int".to_string()),
        }
    }
    Ok(prod)
}

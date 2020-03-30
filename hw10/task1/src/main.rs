extern crate srv_config;
extern crate srv_hasher;

use std::io::{BufReader, BufRead, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicUsize, Ordering};
use srv_config::Config;
use srv_hasher::Solution;
use std::{process, io, thread};
use std::sync::Arc;
use srv_hasher::ParallelQueue;

/// Datentyp für die beim Server auftretenden Fehlertypen.
#[derive(Debug)]
pub enum HashServerError {
    Io(std::io::Error),
}

/// Funktion zum Abarbeiten eines Clients.
fn handle_client(stream: &TcpStream, orders:ParallelQueue<String>) {
    let mut reader = BufReader::new(stream);

    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                orders.add(line.trim_right().to_string());
            }
            _ => {
                break;
            }
        }
    }
}

/// Hauptfunktion, die beim Starten des Servers ausgeführt wird.
pub fn main() {
    let c = Config::load();

    if c.verbosity > 0 {
        println!("Starting Multi Hash Server 0.1:");
        println!(
            "verbosity: {} | address: {} | port: {} | test-mode: {}",
            c.verbosity,
            c.address,
            c.port,
            c.testing
        );
    }

    let host = format!("{}:{}", c.address, c.port);

    let orders: ParallelQueue<String> = ParallelQueue::new();
	
	let orders_1 = orders.clone();
	
    thread::spawn(move || {
        match TcpListener::bind(host).map_err(HashServerError::Io) {
            Ok(listener) => {
                for s in listener.incoming() {
                    if let Ok(stream) = s {
                        let order_queue = orders_1.clone();
                        thread::spawn(move || {
                            handle_client(&stream, order_queue);
                        });
                    }
                }
            },
            Err(e) => {
                println!("Failed to start the MultiHashServer: {:?}", e);
            }
        }
    
    });
    
	let threads_arc = Arc::new(AtomicUsize::new(5));
	
	let orders_2 = orders.clone();
	
	let threads = threads_arc.clone();
	thread::spawn(move || {
		loop {
			if let Some(order) = orders_2.pop() {
				let mut words = order.split(',');
				let t = threads.load(Ordering::SeqCst);
				
				match (words.next(),words.next(),words.next()){
					(Some(difficulty), None, None) => {
						println!("\nSearching with {} threads for hash with difficulty: {}", t, difficulty);
						prompt();
						if let Some(sol) = srv_hasher::search_with_threads(t, difficulty.to_string(), 42, <usize>::max_value(), Some(1), true) {
							println!("\nFound solution {:?}", sol);
							prompt();
						}
					},
					(Some(difficulty), Some(range), None) => {
						if let Ok(rng) = range.parse::<usize>() {
							println!("\nSearching with {} threads for hash with difficulty ({}) in range 0 - {} ", t, difficulty, range);
							prompt();
							let result_queue = srv_hasher::search_multiple_with_threads(t, difficulty.to_string(), 42, rng);
							println!();
							while let Some(solution) = result_queue.pop() {
								println!("{:?}", solution);
							}
							prompt();
						}
					},
					(Some(difficulty), Some(range), Some(port)) => {
						if let Ok(rng) = range.parse::<usize>() {
							println!("\nSearching with {} threads for hash with difficulty ({}) in range 0 - {} ", t, difficulty, range);
							prompt();

							let result_queue = srv_hasher::search_multiple_with_threads(t, difficulty.to_string(), 42, rng);
							let host = format!("{}:{}", c.address, port);
							thread::spawn(move || {
								match TcpListener::bind(host) {
									Ok(listener) => {
										for s in listener.incoming() {
											if let Ok(mut stream) = s {
												while let Some(solution) = result_queue.pop() {
													stream.write(format!("{:?}\n", solution).as_bytes());
													stream.flush();
												}
												
											}
										}
									},
									Err(e) => {
										println!("Failed to start the MultiHashServer: {:?}", e);
									}
								}
							});
						}
					},
					(_, _, _) => {
					
					}
				}
			}
		}
	});
	
	
	let orders_3 = orders.clone();
	
	let stdin = io::stdin();
    let mut sl = stdin.lock();
	
    loop {
		prompt();
        let mut line = String::new();
        match sl.read_line(&mut line) {
            Ok(_) => {
                let mut lines = line.trim_right().split(' ');
                match lines.next() {
                    Some("exit") => process::exit(0),
                    Some("halt") => {
                        println!("Saving state...");
                        //Serialize here
                        orders_3.serialize();
                        process::exit(0);
                    },
                    Some("continue") => {
                        println!("Restoring state");
                        //orders.deserialize();
                    },
                    Some("threads") => {
                        if let Some(thread_string) = lines.next() {
							if let Ok(thr) = thread_string.parse::<usize>() {
								println!("Setting thread count to {}", thr);
								threads_arc.store(thr, Ordering::SeqCst);
							}
                        }
                    },
                    Some(&_) => {},
                    None => {},
                }
            }
            Err(_) => {},
        }
    }
}

fn prompt(){
	print!("HashServer> ");
	io::stdout().flush().ok().expect("Could not flush stdout");
}
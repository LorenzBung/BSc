extern crate nix;
extern crate procinfo;

use nix::unistd::getpid;
use std::env::args;
use std::process;

mod child;
mod unit_tests;
mod zombie;

fn main() {
    let arguments: Vec<String> = args().collect();

    if arguments.len() == 2 {
        let result = child::run_childs(i32::from(getpid()), &arguments[1]);
        match result {
            Ok(_) => {}
            Err(e) => {
                print!("{}", e);
                process::exit(1)
            }
        }
    } else if arguments.len() == 1 {
        zombie::run_zombie();
    } else {
        println!("Bitte nur einen oder zwei Parameter angeben!");
        process::exit(1)
    }
}

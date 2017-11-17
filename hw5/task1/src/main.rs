extern crate procinfo;
extern crate nix;

use std::env::args;
use std::process;
use nix::unistd::getpid;

mod unit_tests;
mod zombie;
mod child;


fn main() {
    let arguments: Vec<String> = args().collect();

    if arguments.len() == 2 {

        let result = child::run_childs(i32::from(getpid()), &arguments[1]);
        match result {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e);
                process::exit(1)
            }
        }

    } else {
        zombie::run_zombie();
    }

}

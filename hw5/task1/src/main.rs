extern crate procinfo;
extern crate nix;

use std::env::args;
use std::process;

mod unit_tests;
mod zombie;
mod child;


fn main() {
    let arguments:Vec<String> = args().collect();

    if arguments.len() == 2 {


        match procinfo::pid::stat_self(){
            Ok(stat) => {
                let result = child::run_childs(stat.pid, &arguments[1]);
                match result {
                    Ok(_) => {},
                    Err(e) => {
                        println!("{}", e);
                        process::exit(1)
                    },
                }
            },
            Err(_) => {
                println!("Couldn't retrieve my own PID.");
                process::exit(1)
            },
        }


    } else {
        zombie::run_zombie();
    }

}
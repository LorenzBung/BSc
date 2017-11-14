extern crate procinfo;

use std::env::args;

mod zombie;
mod child;


fn main() {
    let arguments:Vec<String> = args().collect();

    if arguments.len() == 2 {


        match procinfo::pid::stat_self(){
            Ok(stat) => {
                child::run_childs(stat.pid, &arguments[2]);
            },
            Err(_) => {},
        }


    } else {
        zombie::run_zombie();
    }

}
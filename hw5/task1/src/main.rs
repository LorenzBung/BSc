extern crate procinfo;
extern crate nix;

use std::env::args;

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
                    Err(_) => {},
                }
            },
            Err(_) => {},
        }


    } else {
        //zombie::run_zombie();
    }

}
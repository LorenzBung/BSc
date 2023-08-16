use std::env;
use std::process;
use task2::Config;
extern crate task2;

fn main() {
    let args = env::args().collect();

    let res = Config::new(&args);
    match res {
        Ok(conf) => {
            println!(
                "You asked me to count all '{}' in '{}'",
                conf.search, conf.line
            );
            let occ = task2::run(&conf);
            println!("Found {} '{}' in '{}'", occ, conf.search, conf.line);
        }
        Err(message) => {
            println!("{}", message);
            process::exit(1)
        }
    }
}

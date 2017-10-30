use std::env;
use std::process;
use task2::Config;
extern crate task2;

fn main() {
    let args = env::args().collect();

    //print_arguments(&args);
    //let conf = parse_arguments_simple(&args);
    //let res = parse_arguments(&args);

    let res = Config::new(&args);
    match res {
        Ok(conf) => {
            println!(
                "You asked me to count all '{}' in '{}'",
                conf.search,
                conf.line
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

/*
pub fn run(conf: &Config) -> i32 {
    let mut count = 0;
    for c in conf.line.chars() {
        if c == conf.search {
            count = count + 1;
        }
    }
    count
}
*/

/// Parses relevant arguments, returning the filled Config in Result
///
///
/// This function will parse the relevant arguments from the
/// given <Strings>.
/// Returns Config or Error Message in Result
#[allow(dead_code)]
fn parse_arguments(args: &Vec<String>) -> Result<Config, String> {

    if args.len() < 3 {
        return Err("not ennugh parameters".to_string());
    }

    match args[1].chars().nth(0) {
        Some(value) => {
            Ok(Config {
                search: value,
                line: args[2].clone(),
            })
        }
        None => Err("char mismatch".to_string()),
    }
}

/// Parses relevant arguments, returning the filled Config
///
///
/// This function will parse the relevant arguments from the
/// given <Strings>.
/// Returns Config
#[allow(dead_code)]
fn parse_arguments_simple(args: &Vec<String>) -> Config {
    Config {
        search: args[1].chars().nth(0).unwrap(),
        line: args[2].clone(),
    }
}

/// Prints elements of Vec
///
///
/// This function will print all elements of Vec with "args found: <elem>" in
/// each line
///
/// Returns nothing
#[allow(dead_code)]
fn print_arguments(args: &Vec<String>) {
    for s in args {
        println!("args found: {}", s);
    }
}

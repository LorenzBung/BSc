use std::env;
use std::process;

/// a struct to hold all of our configuration
#[derive(Debug,PartialEq)]
struct Config{
    search: char,
    line: String,
}

fn main() {
    let args = env::args().collect();
    print_arguments(&args);
    //let conf = parse_arguments_simple(&args);
    let res = parse_arguments(&args);
    match res {
        Ok(conf) => println!("{:?}", conf),
        Err(_) => process::exit(1),
    }

}

fn run() {

}

/// Parses relevant arguments, returning the filled Config in Result
///
///
/// This function will parse the relevant arguments from the
/// given <Strings>.
/// Returns Config or Error Message in Result
fn parse_arguments(args: &Vec<String>) -> Result<Config, String> {

    if args.len() < 3 {
        return Err("not ennugh parameters".to_string());
    }

    if args[1].chars().nth(0) == None {
        return Err("char mismatch".to_string());    
    }

    let config = Config{search: args[1].chars().nth(0).unwrap(), line: args[2].clone()};
    Ok(config)
}

/// Parses relevant arguments, returning the filled Config
///
///
/// This function will parse the relevant arguments from the
/// given <Strings>.
/// Returns Config
#[allow(dead_code)]
fn parse_arguments_simple(args: &Vec<String>) -> Config {
    Config{search: args[1].chars().nth(0).unwrap(), line: args[2].clone()}
}

/// Prints elements of Vec
///
///
/// This function will print all elements of Vec with "args found: <elem>" in
/// each line
///
/// Returns nothing
fn print_arguments(args: &Vec<String>) {
    for s in args {
        println!("args found: {}", s);
    }
}

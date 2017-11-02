/// a struct to hold all of our configuration
#[derive(Debug, PartialEq)]
pub struct Config {
    pub search: char,
    pub line: String,
}

pub fn run(conf: &Config) -> i32 {
    let mut count = 0;
    for c in conf.line.chars() {
        if c == conf.search {
            count = count + 1;
        }
    }
    count
}

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

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        match args[1].chars().nth(0) {
            Some(value) => {
                Ok(Config {
                    search: value,
                    line: args[2].clone(),
                })
            }
            None => Err("char mismatch"),
        }
    }
}

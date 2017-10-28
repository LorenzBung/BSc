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

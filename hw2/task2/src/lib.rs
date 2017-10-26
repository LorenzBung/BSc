/// a struct to hold all of our configuration
#[derive(Debug, PartialEq)]
pub struct Config {
    pub search: char,
    pub line: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("not ennugh parameters");
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

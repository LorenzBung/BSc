use std::env;
use std::ffi::OsString;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub enum Command {
    Empty,
    Exit,
    Cd(Option<OsString>),
}

#[derive(PartialEq, Debug)]
pub struct CommandNotFoundError;

impl FromStr for Command {
    type Err = CommandNotFoundError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        match parts.next() {
            Some("exit") => Ok(Command::Exit),
            Some("cd") => Ok(Command::parse_cd(parts.next())),
            Some(cmd) => {
                //For use in next homework, to execute programs.
                let _ = cmd;
                Err(CommandNotFoundError)
            }
            None => Ok(Command::Empty),
        }
    }
}

impl Command {
    /// If the passed String exists, set the path of the Cd in the enum (as OsString)
    /// Returns **Command**
    pub fn parse_cd(cmd: Option<&str>) -> Self {
        match cmd {
            None => Command::Cd(None),
            Some(dest) => Command::Cd(Some(OsString::from(dest))),
        }
    }

    /// If this instance is of Kind **Command::Cd** either go to HOME or navigate to the given path
    pub fn exec_cd(&self) {
        if let Command::Cd(None) = *self {
            let possible_home = env::home_dir();

            if let Some(home) = possible_home {
                let home_path = home.as_path();
                let _ = env::set_current_dir(home_path);
            }
        }

        if let Command::Cd(Some(ref path)) = *self {
            match path.to_str() {
                Some(_) => {
                    let _ = env::set_current_dir(path);
                }
                None => {}
            }
        }
    }
}

extern crate nix;

use std::io::{BufRead, Write};
use std::str::FromStr;
use std::env;
use std::ffi::{OsString, CString};
use nix::unistd::ForkResult::{Child, Parent};
use nix::sys::wait::wait;
use nix::unistd::{dup2, close, execvp, fork, pipe};

mod errortype;

use errortype::ErrorType;

#[derive(PartialEq, Debug)]
pub enum Command {
    Empty,
    Exit,
    Program(String),
    Cd(Option<OsString>),
}

impl FromStr for Command {
    type Err = ErrorType;
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut parts = s.split_whitespace();

        match parts.next() {
            Some("exit") => Ok(Command::Exit),
            Some("cd") => Ok(Command::parse_cd(parts.next())),
            Some(_) => Ok(Command::Program(s.to_string())),
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
pub struct Shell<R: BufRead, W: Write> {
    pub reader: R,
    pub writer: W,
    pub should_exit: bool,
    pub name: String,
}

impl<R: BufRead, W: Write> Shell<R, W> {
    /// Creates a new Shell with a name, input and output.
    pub fn new(input: R, output: W, name: String) -> Self {
        Shell {
            reader: input,
            writer: output,
            should_exit: false,
            name: name,
        }
    }
    /// Initializes the Shell Loop.
    /// Starts the shell.
    pub fn start(&mut self) -> Result<(), ErrorType> {
        self.shell_loop()
    }

    /// Loops while exit hasn't been called.
    /// When *prompt()* returns (the user hit enter) a Command is created through the FromStr-Trait.
    /// If the Command-creation succeeds, run the command.
    fn shell_loop(&mut self) -> Result<(), ErrorType> {
        while !self.should_exit {
            if let Ok(Some(line)) = self.prompt() {
                let _ = Command::from_str(&line).and_then(|cmd| self.run(cmd));
            }
        }
        Ok(())
    }

    /// Prints the shell prompt.
    /// Waits for user input and returns the read line.
    pub fn prompt(&mut self) -> Result<Option<String>, ErrorType> {
        match env::current_dir() {
            Ok(pwd) => {
                let _ = self.writer.write_all(
                    format!("{} {}> ", self.name, pwd.display())
                        .as_bytes(),
                );
                let _ = self.writer.flush();

                let mut line: String = String::new();
                let read_result = self.reader.read_line(&mut line);
                match read_result {
                    Ok(_) => Ok(Some(line)),
                    Err(_) => Ok(None),
                }
            }
            Err(_) => return Err(ErrorType::PathError),
        }
    }

    /// Runs a command.
    fn run(&mut self, command: Command) -> Result<(), ErrorType> {

        match command {
            Command::Empty => {}
            Command::Program(cmd) => {

                let commands = cmd.split('|');
                let commands_vec: Vec<&str> = commands.collect();

                if commands_vec.len() > 2 {
                    let _ = self.writer.write_all(
                        format!("Error: Only one pipe supported at the moment.\n")
                            .as_bytes(),
                    );
                    return Err(ErrorType::BrokenPipeError);
                }

                let needs_pipe = commands_vec.len() == 2;


                let fi = fork();
                match fi {
                    Ok(Parent { child: _child }) => {
                        let _ = wait();

                    }
                    Ok(Child) => {
                        if let Ok((reader, writer)) = pipe() {
                            let sec = fork();
                            match sec {
                                Ok(Parent { child: _child }) => {
                                    let _ = wait();
                                    if let Some(second) = commands_vec.get(1) {
                                        match close(writer) {
                                            Ok(_) => {}
                                            Err(_) => return Err(ErrorType::BrokenPipeError),
                                        }
                                        dup2(reader, 0).unwrap();
                                        self.execute(second);
                                    }
                                }
                                Ok(Child) => {
                                    if let Some(first) = commands_vec.get(0) {
                                        if needs_pipe {
                                            close(reader).unwrap();
                                            dup2(writer, 1).unwrap();
                                        }
                                        self.execute(first);
                                    }
                                }
                                Err(_) => return Err(ErrorType::ForkError),
                            }
                        }
                    }
                    Err(_) => return Err(ErrorType::ForkError),
                }

            }
            Command::Exit => self.should_exit = true,
            Command::Cd(_) => {
                command.exec_cd();
            }
        }
        Ok(())
    }

    fn execute(&mut self, cmd: &str) {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        let args: Vec<CString> = parts.iter().map(|f| CString::new(*f).unwrap()).collect();
        let cmd = args.clone();
        let t = args.into_boxed_slice();

        match execvp(&t[0], &t) {
            Ok(_) => {}
            Err(_) => {
                let _ = self.writer.write_all(
                    format!("{:?}: command not found\n", cmd[0])
                        .as_bytes(),
                );
            }
        }
    }
}

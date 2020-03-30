use std::io::{BufRead, Write};
use command::*;
use std::str::FromStr;
use std::env;

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
    pub fn start(&mut self) -> Result<(), &str> {
        self.shell_loop()
    }

    /// Loops while exit hasn't been called.
    /// When *prompt()* returns (the user hit enter) a Command is created through the FromStr-Trait.
    /// If the Command-creation succeeds, run the command.
    fn shell_loop(&mut self) -> Result<(), &str> {
        while !self.should_exit {
            if let Ok(Some(line)) = self.prompt() {
                let _ = Command::from_str(&line).and_then(|cmd| self.run(cmd));
            }
        }
        Ok(())
    }

    /// Prints the shell prompt.
    /// Waits for user input and returns the read line.
    pub fn prompt(&mut self) -> Result<Option<String>, &str> {
        match env::current_dir() {
            Ok(pwd) => {
                let _ = self.writer.write(
                    format!("{} {} > ", self.name, pwd.display())
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
            Err(_) => return Err("Path Error"),
        }
    }

    /// Runs a command.
    /// Currently only `cd` and `exit` are working.
    fn run(&mut self, command: Command) -> Result<(), CommandNotFoundError> {

        match command {
            Command::Empty => {}
            Command::Exit => self.should_exit = true,
            Command::Cd(_) => {
                command.exec_cd();
            }
        }
        Ok(())
    }
}

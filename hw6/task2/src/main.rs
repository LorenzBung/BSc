use shell::Shell;
use std::io;
use std::process;

mod command;
mod shell;
mod unit_tests_shell;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let mut s = Shell::new(stdin.lock(), stdout.lock(), "schell".to_string());
    match s.start() {
        Ok(_) => process::exit(0),
        Err(_) => process::exit(1),
    }
}

extern crate task2;

use std::io;
use std::process;
use task2::Shell;

/// Diese Funktion startet die Shell durch Aufruf
/// von `Shell::new()`.
fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let mut s = Shell::new(stdin.lock(), stdout.lock(), "schell".to_string());
    // Kontrolliertes bzw. unkontrolliertes Beenden der Shell
    match s.start() {
        Ok(_) => process::exit(0),
        Err(_) => process::exit(1),
    }
}

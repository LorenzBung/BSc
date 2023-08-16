#[macro_use]
extern crate clap;

use clap::App;

/// Datentyp zur Beschreibung der Konfiguration des Servers.
pub struct Config {
    pub address: String,
    pub port: String,
    pub verbosity: u64,
    pub testing: bool,
}

impl Config {
    /// Funktion zum Lesen der Konfigurationsdatei des Servers.
    pub fn load() -> Self {
        let yaml = load_yaml!("cli.yml");
        let matches = App::from_yaml(yaml).get_matches();
        let address = matches.value_of("address").unwrap_or("127.0.0.1");
        let port = matches.value_of("port").unwrap_or("7878");
        let verbosity = matches.occurrences_of("verbose");
        let mut testing = false;

        // Falls das Unterkommando timings angegeben wurde aktiviere die Zeitmessung.
        if let Some(ref sub_command) = matches.subcommand {
            if sub_command.name.eq("test") {
                testing = true;
            }
        }

        Config {
            address: address.to_string(),
            port: port.to_string(),
            verbosity: verbosity,
            testing: testing,
        }
    }
}

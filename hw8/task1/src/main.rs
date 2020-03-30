#[macro_use]
extern crate clap;
extern crate task1;
extern crate sys_info;

use clap::App;
use std::process;
use task1::search_with_threads;


pub fn main() {
    // Lade Commandline Einstellungen aus YAML-Datei
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Speichere alle Argumente in Variablen
    // Falls ein Wert für ein Argument nicht existiert, lade einen default-Wert
    let base = matches.value_of("base").unwrap_or("1");
    let diff = String::from(matches.value_of("difficulty").unwrap_or("1"));
    let cpus = sys_info::cpu_num().unwrap_or(1).to_string();
    let threads = matches.value_of("threads").unwrap_or(&cpus);
    let sync = matches.is_present("sync");
    let special = matches
        .value_of("special")
        .unwrap_or("1")
        .parse::<usize>()
        .unwrap_or(1);
    let wait = matches.is_present("wait");
    let verbosity = matches.occurrences_of("verbose");

    let mut time = false;

    // Falls irgendein Zeichen nicht hexadezimal ist, breche ab.
    if diff.chars().any(|c| !c.is_digit(16)) {
        println!("Difficulty is not hexadecimal.");
        process::exit(1)
    }

    // Falls das Unterkommando timings angegeben wurde aktiviere die Zeitmessung.
    if let Some(ref sub_command) = matches.subcommand {
        if sub_command.name.eq("timings") {
            time = true;
        }
    }

    // Gebe Headerinfos aus, wenn mindestens ein 'v' vorkommt
    if verbosity > 0 {
        println!("--------------------------------------------");
        println!(
            "Container     : \"{}\"",
            sys_info::hostname().unwrap_or(String::from("-"))
        );
        println!("Physical CPUs : {}", sys_info::cpu_num().unwrap_or(0));
        println!("Logical CPUs  : {}", sys_info::cpu_num().unwrap_or(0));
        println!("CPU Speed     : {}", sys_info::cpu_speed().unwrap_or(0));
        println!("Load Average  : {:?}", sys_info::loadavg().unwrap());
        println!("Processes     : {}", sys_info::proc_total().unwrap_or(0));
        println!("--------------------------------------------");
    }

    // Versuche Strings der Basis und Threadanzahl zu usize zu parsen
    // Anderenfalls gebe den fehlerhaften Paramter aus.
    match (base.parse::<usize>(), threads.parse::<usize>()) {
        (Ok(b), Ok(t)) => {
            println!("Please wait...");

            let mut sync_opt = None;
            if sync {
                sync_opt = Some(special);
            }

            let _ = search_with_threads(t, diff, b, time, verbosity, sync_opt, wait);

        }
        (_, Err(_)) => {
            println!("Number of threads is not integer.");
            process::exit(1)
        }
        (Err(_), _) => {
            println!("Base is not integer.");
            process::exit(1)
        }
    };
}

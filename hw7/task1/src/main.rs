extern crate clap;
extern crate time;

use clap::{App, Arg, SubCommand};
use std::process;
use time::get_time;
mod hash256;
mod hasher_sha256;

/// Hauptfunktion zum Starten des Hashers.
pub fn main() {
    let matches = create_app().get_matches();
    let base = matches.value_of("base").unwrap_or("1");
    let diff = matches.value_of("difficulty").unwrap_or("1");
    let mut time_measurement = false;

    if diff.chars().any(|c| !c.is_digit(16)) {
        println!("Difficulty is not hexadecimal.");
        process::exit(1)
    }

    if let Some(ref sub_command) = matches.subcommand {
        if sub_command.name.eq("timings") {
            time_measurement = true;
        }
    }

    match base.parse::<usize>() {
        Ok(b) => {
            println!("Using base: {}", base);
            println!("Using difficulty: {}", diff);
            println!("Please wait...");
            let start = get_time();

            let max = <usize>::max_value();
            for n in 0..max {
                if let Some(x) = hash256::verify_product(b, n, &diff.to_string()) {
                    let end = get_time();
                    println!("Number: {} --> hash: {}", x.number, x.hash);
                    if time_measurement {
                        let diff = end - start;
                        let s = diff.num_seconds();
                        let ms = diff.num_milliseconds();
                        let us = diff.num_microseconds().unwrap_or(ms * 1000);
                        println!("(Duration {}s / {}ms / {}us)", s, ms, us);
                    }
                    process::exit(0)
                }
            }
        }
        Err(_) => {
            println!("Base is not integer.");
            process::exit(1)
        }
    };
}

/// Diese Funktion dient der Konfiguration der App mit dem Paket **Clap**.
fn create_app<'a, 'b>() -> App<'a, 'b> {
    App::new("Hash256")
        .version("1.0")
        .author("Lorenz Bung & Joshua Rutschmann")
        .about("Calculates the Hashvalue of the given base, number and difficulty.")
        .arg(
            Arg::with_name("base")
                .value_name("base")
                .help("The base of the hash to be calculated on.")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("difficulty")
                .value_name("difficulty")
                .help("The difficulty of the calculated hash.")
                .takes_value(true)
                .required(true),
        )
        .subcommand(
            SubCommand::with_name("timings")
                .about("controls timing features")
                .version("1.0")
                .author("Lorenz Bung & Joshua Rutschmann"),
        )
}

extern crate sha2;
extern crate time;

mod tests;

#[cfg(feature = "SHA2")]
use self::sha2::Sha256;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::thread;
use time::{get_time, Duration};

pub struct Sha256;

pub trait Hasher {
    type Output: HashResult;
    fn hash(input: &[u8]) -> Self::Output;
}

pub trait HashResult {
    /// Get the output in hex notation.
    fn hex(&self) -> String;
    /// Size of the output in bytes.
    fn size() -> usize;
}

impl Hasher for Sha256 {
    type Output = [u8; 32];

    fn hash(input: &[u8]) -> Self::Output {
        use self::sha2::*;
        let mut tmp = Sha256::new();
        tmp.input(input);
        let r = tmp.result().as_slice().to_vec();
        [
            r[0], r[1], r[2], r[3], r[4], r[5], r[6], r[7], r[8], r[9], r[10], r[11], r[12], r[13],
            r[14], r[15], r[16], r[17], r[18], r[19], r[20], r[21], r[22], r[23], r[24], r[25],
            r[26], r[27], r[28], r[29], r[30], r[31],
        ]
    }
}

impl HashResult for [u8; 32] {
    fn hex(&self) -> String {
        const HEX: [char; 16] = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
        ];
        let mut tmp = String::with_capacity(32 * 2);
        for byte in self.iter() {
            tmp.push(HEX[*byte as usize / 16]);
            tmp.push(HEX[*byte as usize % 16]);
        }
        tmp
    }

    fn size() -> usize {
        32
    }
}

#[derive(Debug, PartialEq)]
pub struct Solution {
    pub number: usize,
    pub hash: String,
}

/// Überprüft ob *base* und *number* auf einen Hash abbilden,
/// der mit der übergebenen *difficulty* übereinstimmt.
/// Falls ja, kommt eine `Solution` in der Option mit den Ergebnissen zurück.
/// Falls nein, steht None im Optional
pub fn verify_product(base: usize, number: usize, difficulty: &String) -> Option<Solution> {
    let sol = base * number;
    let input = sol.to_string();
    let bytes = input.as_bytes();

    let hash = Sha256::hash(bytes).hex();

    if hash.ends_with(difficulty) {
        return Some(Solution {
            number: number,
            hash: hash,
        });
    }

    None
}

/// Sucht nach einem Hash für die angegebene Basis und die Schwierigkeit.
/// Wenn `total` > 1 ist, dann hat jeder Aufruf mit einem anderen `start`-Wert (von 0 - total)
/// eine disjunkte Zahlenmenge für die Suche zur Auswahl.
fn search_hash(
    hash: &String,
    base: usize,
    start: usize,
    total: usize,
    sync: bool,
    found: Arc<AtomicBool>,
    special: usize,
    solution_tx: Sender<Solution>,
    timing_tx: Sender<(Duration, usize)>,
    measure: bool,
) {
    let max = <usize>::max_value();
    let mut n = start;

    let thread_start = get_time();
    while n < max {
        // Der special Parameter begrenzt die Anzahl der load()-Aufrufe auf jeden n.ten Loop.
        if n % special == 0 && found.load(Relaxed) {
            // Anderer Thread hat eine Lösung gefunden (sync ist aktiviert). Beende Suche.
            break;
        }

        if let Some(solution) = verify_product(base, n, hash) {
            if sync {
                found.store(true, Relaxed);
            }
            // Sende gefundene Solution an den Consumer.
            let _ = solution_tx.send(solution);
            // Beende Suche.
            break;
        }
        n += total;
    }

    // Falls measure aktiviert ist Sende Zeitdauer und Anzahl Loops an den Consumer.
    if measure {
        let thread_end = get_time();
        let _ = timing_tx.send((thread_end - thread_start, (n / total)));
    }
}

/// Teilt, wenn nötig, die Suche nach dem Hash auf mehrere Threads auf.
/// Gibt ggf. die Solution (für die Tests) zurück.
pub fn search_with_threads(
    threads: usize,
    diff_string: String,
    with_base: usize,
    time_measurement: bool,
    verbosity: u64,
    sync: Option<usize>,
    wait: bool,
) -> Option<Solution> {
    let diff = Arc::new(diff_string);
    let mut children = vec![];
    let mut solution = None;

    let (solution_tx, solution_rx) = channel();
    let (timing_tx, timing_rx) = channel();

    let found = Arc::new(AtomicBool::new(false));
    let m = time_measurement && verbosity > 0;

    let total_start = get_time();
    if threads > 1 {
        if verbosity > 0 {
            println!("Searching with {} threads", threads);
        }

        // Erstellt Anzahl angeforderter Threads.
        // Klont für jeden Thread die Referenz auf die gemeinsamen Variablen.
        for i in 0..threads {
            let diff = diff.clone();
            let solution_tx = solution_tx.clone();
            let timing_tx = timing_tx.clone();
            let found = found.clone();

            children.push(thread::spawn(move || {
                // Suche in jedem der Threads.
                search_hash(
                    &diff,
                    with_base,
                    i,
                    threads,
                    sync.is_some(),
                    found,
                    sync.unwrap_or(1),
                    solution_tx,
                    timing_tx,
                    m,
                );

                if verbosity > 1 {
                    println!("[DEBUG] Thread {} exited", i);
                }
            }));
        }
    } else {
        // Suche auf dem Main-Thread.
        search_hash(
            &diff,
            with_base,
            0,
            1,
            sync.is_some(),
            found,
            sync.unwrap_or(1),
            solution_tx,
            timing_tx,
            m,
        );

        if verbosity > 1 {
            println!("[DEBUG] Finished search on main thread.");
        }
    }

    // Empfängt die Lösung von einem der Producer.
    match solution_rx.recv() {
        Ok(sol) => {
            solution = Some(Solution {
                number: sol.number,
                hash: sol.hash.clone(),
            });
            let total_end = get_time();
            println!("Number: {} --> hash: {}", sol.number, sol.hash);
            if time_measurement && verbosity == 0 {
                let diff = total_end - total_start;
                let s = diff.num_seconds();
                let ms = diff.num_milliseconds();
                let us = diff.num_microseconds().unwrap_or(ms * 1000);
                println!("(Duration {}s / {}ms / {}us)", s, ms, us);
            }
        }
        Err(_) => {}
    }

    let mut sum_loops = 0usize;
    let mut sum_time: Duration = Duration::zero();

    for child in children {
        if time_measurement && verbosity > 0 {
            // Warte auf Zeitstatistik-Nachricht von jedem Thread auf dem zweiten MPSC Channel.
            if let Ok(stats) = timing_rx.recv() {
                // Addiere Werte dieses threads zu der Summe.
                sum_time = sum_time + stats.0;
                sum_loops += stats.1;
            }
        }

        // Falls *wait* wahr ist, warte auf aktuellen thread (child)
        if wait {
            let _ = child.join();
        }
    }

    // Gebe die Gesamtergebnisse der Threads aus.
    if time_measurement && verbosity > 0 {
        println!("Sum Loops in Producers:       {}", sum_loops);
        let s = sum_time.num_seconds();
        let ms = sum_time.num_milliseconds();
        let us = sum_time.num_microseconds().unwrap_or(ms * 1000);
        println!("Sum Duration in Producers:    {}s / {}ms / {}us", s, ms, us);
    }

    // Gebe die Option einer Solution zurück.
    solution
}

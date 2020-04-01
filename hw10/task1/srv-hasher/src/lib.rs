extern crate sha2;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[cfg(feature = "SHA2")]
use self::sha2::Sha256;
use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::mpsc::{Sender, channel};
use std::fs::File;
use std::io::Write;
use std::ops::Deref;

#[derive(Serialize)]
struct InnerQueue<T> {
    inner: VecDeque<T>,
}

impl<'a, T: Send + Clone + serde::Serialize> InnerQueue<T> {

    fn new() -> Self {
        Self { inner: VecDeque::new() }
    }

}

#[derive(Clone)]
pub struct ParallelQueue<T> {
    inner: Arc<Mutex<InnerQueue<T>>>,
}

impl<'a, T: Send + Clone + serde::Serialize> ParallelQueue<T> {

    pub fn new() -> Self {
        Self { inner: Arc::new(Mutex::new(InnerQueue::new())) }
    }

    pub fn serialize(&self) {
        let inner_queue = self.inner.lock();
        if let Ok(queue) = inner_queue {
            let queue_json = serde_json::to_string(queue.deref()).unwrap();
            let mut file = File::create("orders.save").unwrap();
            file.write_all(queue_json.as_bytes());
        }
    }

    pub fn pop(&self) -> Option<T> {
        let inner_queue = self.inner.lock();
        if let Ok(mut queue) = inner_queue {
            queue.inner.pop_front()
        } else {
            panic!("MutexError");
        }
    }

    pub fn add(&self, order: T) {
        if let Ok(mut queue) = self.inner.lock() {
            queue.inner.push_back(order);
        } else {
            panic!("MutexError");
        }
    }
}



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
            r[0],
            r[1],
            r[2],
            r[3],
            r[4],
            r[5],
            r[6],
            r[7],
            r[8],
            r[9],
            r[10],
            r[11],
            r[12],
            r[13],
            r[14],
            r[15],
            r[16],
            r[17],
            r[18],
            r[19],
            r[20],
            r[21],
            r[22],
            r[23],
            r[24],
            r[25],
            r[26],
            r[27],
            r[28],
            r[29],
            r[30],
            r[31],
        ]
    }
}

impl HashResult for [u8; 32] {
    fn hex(&self) -> String {
        const HEX: [char; 16] = [
            '0',
            '1',
            '2',
            '3',
            '4',
            '5',
            '6',
            '7',
            '8',
            '9',
            'a',
            'b',
            'c',
            'd',
            'e',
            'f',
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

#[derive(Debug, PartialEq, Clone, Serialize)]
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
    end: usize,
    total: usize,
    sync: bool,
    found: Arc<AtomicBool>,
    special: usize,
    solution_tx: Sender<Solution>,
) {
    let mut n = start;

    while n < end {

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
}

/// Teilt, wenn nötig, die Suche nach dem Hash auf mehrere Threads auf.
/// Gibt ggf. die Solution (für die Tests) zurück.
pub fn search_with_threads(
    threads: usize,
    diff_string: String,
    with_base: usize,
    range_of_numbers: usize,
    sync: Option<usize>,
    wait: bool,
) -> Option<Solution> {
    let diff = Arc::new(diff_string);
    let mut children = vec![];
    let mut solution = None;

    let (solution_tx, solution_rx) = channel();

    let found = Arc::new(AtomicBool::new(false));

    if threads > 1 {

        // Erstellt Anzahl angeforderter Threads.
        // Klont für jeden Thread die Referenz auf die gemeinsamen Variablen.
        for i in 0..threads {
            let diff = diff.clone();
            let solution_tx = solution_tx.clone();
            let found = found.clone();

            children.push(thread::spawn(move || {

                // Suche in jedem der Threads.
                search_hash(
                    &diff,
                    with_base,
                    i,
                    range_of_numbers,
                    threads,
                    sync.is_some(),
                    found,
                    sync.unwrap_or(1),
                    solution_tx,
                );

            }));
        }
    } else {
        // Suche auf dem Main-Thread.
        search_hash(
            &diff,
            with_base,
            0,
            range_of_numbers,
            1,
            sync.is_some(),
            found,
            sync.unwrap_or(1),
            solution_tx,
        );

    }
	
	// Empfängt die Lösung von einem der Producer.
    match solution_rx.recv() {
        Ok(sol) => {
            solution = Some(Solution {
                number: sol.number,
                hash: sol.hash.clone(),
            });
        }
        Err(_) => {}
    }

    for child in children {
        // Falls *wait* wahr ist, warte auf aktuellen thread (child)
        if wait {
            let _ = child.join();
        }
    }

    // Gebe die Option einer Solution zurück.
    solution
}



/// Sucht nach einem Hash für die angegebene Basis und die Schwierigkeit.
/// Wenn `total` > 1 ist, dann hat jeder Aufruf mit einem anderen `start`-Wert (von 0 - total)
/// eine disjunkte Zahlenmenge für die Suche zur Auswahl.
fn search_multiple_hash(
    hash: &String,
    base: usize,
    start: usize,
    end: usize,
    total: usize,
    queue: ParallelQueue<Solution>,
) {
    let mut n = start;

    while n < end {
        if let Some(solution) = verify_product(base, n, hash) {

            // Sende gefundene Solution an den Consumer.
			queue.add(solution);
            break;
        }
        n += total;
    }
}

/// Teilt, wenn nötig, die Suche nach dem Hash auf mehrere Threads auf.
/// Gibt ggf. die Solution (für die Tests) zurück.
pub fn search_multiple_with_threads(
    threads: usize,
    diff_string: String,
    with_base: usize,
    range_of_numbers: usize,
) -> ParallelQueue<Solution> {	
    let diff = Arc::new(diff_string);
    let mut children = vec![];
    let result_queue:ParallelQueue<Solution> = ParallelQueue::new();

    if threads > 1 {

        // Erstellt Anzahl angeforderter Threads.
        // Klont für jeden Thread die Referenz auf die gemeinsamen Variablen.
        for i in 0..threads {
            let diff = diff.clone();
			let queue = result_queue.clone();

            children.push(thread::spawn(move || {

                // Suche in jedem der Threads.
                search_multiple_hash(
                    &diff,
                    with_base,
                    i,
                    range_of_numbers,
                    threads,
                    queue,
                );

            }));
        }
    } else {
        // Suche auf dem Main-Thread.
		let queue = result_queue.clone();
        search_multiple_hash(
            &diff,
            with_base,
            0,
            range_of_numbers,
            1,
			queue,
		);

    }

    for child in children {
        let _ = child.join();
    }

	result_queue
}


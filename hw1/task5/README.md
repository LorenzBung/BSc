# Homework hw1 task 5

## Vorbereitungen

Rufen Sie im `task5/` Verzeichnis: `cargo init --bin` auf. Dadurch wird ein Rust Binary Projekt in `task5/` angelegt. Mit `cargo build` wird die Library erstellt, der Aufruf `cargo run` startet das Programm. Der Aufruf von `cargo test` ruft die UNIT-Tests im `src/main.rs` auf.

Ausserdem können die korrekten Outputs Ihres Programms auf der Console mit dem Aufruf von `bats tests/output.bats` getestet werden.

## task

Schreiben Sie ein Programm, in welchem eine von Ihnen selbst geschriebene Funktion

```rust
fn is_prime(n: u64) -> bool
```

genutzt wird, die überprüft, ob eine gegebene Zahl eine Primzahl ist. Die Funktion muss nicht auf Laufzeit optimiert werden.

Die *main()* Funktion gibt in einer Schleife die Zahlen von 1 bis 30 aus. Die Zahlen, die eine Primzahlt sind werden in der Ausgabe mit einem `*`` Zeichen markiert.

```
1
2*
3*
4
5*
...
```

Nutzen Sie die schon vorgegebene Datei `main.rs`!

Verwenden Sie keine Funktionen aus Bibliotheken dafür, sondern implementieren Sie die Funktion selbst.

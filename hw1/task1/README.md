# Homework hw1 task 1

## Vorbereitungen

Rufen Sie im task1/ Verzeichnis: `cargo init task1` auf. Dadurch wird ein Rust Library Projekt in task1/ angelegt. Mit `cargo build` wird die Library erstellt, der Aufruf `cargo test` ruft die CI Test im tests/ Verzeichnis auf und testet Ihre Library.

## task

Schreiben Sie die Funktion

```rust
pub fn is_leap_year(year: i32) -> bool
```

die überprüft, ob das übergebene Jahr ein Schaltjahr ist.

Das trickreiche an der Überprüfung ist, dass folgende Bedingungen für das Jahr gelten müssen:

```plain
on every year that is evenly divisible by 4
  except every year that is evenly divisible by 100
    unless the year is also evenly divisible by 400
```

Zum Beispiel ist 1997 kein Schaltjahr, aber 1996. 1900 ist kein Schaltjahr aber 2000.

Verwenden Sie keine Funktionen aus Bibliotheken dafür, sondern implementieren Sie die Funktion selbst.

## Test

Testen können Sie Ihre Library durch den Aufruf von `cargo test`. Dann werden alle Tests aus der Datei `tests/task1.rs` ausgeführt. 
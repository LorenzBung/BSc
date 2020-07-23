# Homework hw7 task1

- ['Proof-of-Work'](#proof-of-work)
- [Implementierung](#implementierung)
    - [Modul `hash256`](#modul-hash256)
    - [Parsen der Argumente](#parsen-der-argumente)
    - [Subcommand 'timings'](#subcommand-timings)
    - [Tests](#tests)
    - [Optimierung](#optimierung)
    - [Laufzeit Messungen](#laufzeit-messungen)

## 'Proof-of-Work'
Zur Vorbereitung einer Multithreading Anwendung beschäftigen wir uns mit dem Mining von Kryptowährungen.

Das Grundprinzip bei dem sogenannten 'Proof-of-Work' basiert auf der Idee, dass Miner im Netzwerk nachweisen müssen, dass sie einen gewissen Aufwand erbracht haben.

Das Durchführen eines Proof-of-Work Mechanismus bzw. das Berechnen der Ergebnisse bezeichnet man im Kontext von Blockchains als „Mining“. Die Miner versuchen dabei, durch milliardenfache Ausführung von Rechenoperationen, ein Ergebnis mit bestimmten Eigenschaften zu finden. Haben die Miner ein solches Ergebnis gefunden, werden Sie vergütet.

Die Basis dieser Berechnung stellt das Multiplizieren zweier Zahlen und Hashen des Ergebnisses dar. Gesucht wird dabei ein Hashwert, der eine besondere Eigenschaft hat, z.B. indem er ein bestimmtes Muster enthält.

Dieses zu findende Muster ist die sogenannte 'Difficulty'. Umso mehr Teilnehmer in einem Netzwerk aktiv sind und umso mehr Rechenkapazität somit im Netzwerk vorhanden ist, umso schwieriger wird die Difficulty.

Beispiel:

Gesucht ist die Zahl x, die multipliziert mit 21 (BASE) einen Hash ergibt, der am Ende das Muster "12345" hat. Eine Lösung ist die Zahl 567621, denn der Hash von 21*567621 ergibt "b6bea2a40ed1bd6d9999b2232072092f3df0e02c4b507aa3ad947367b9712345".

## Implementierung

### Modul `hash256`

Schreiben Sie im Modul die Funktion *verify_product(base, number, difficulty) -> Option<Solution>*. Die Funnktion bildet den Hashwert des Produkts aus *base* und *number* (beide Typ `usize`)und überprüft, ob am Ende des Musters die *difficulty* steht. Stimmt das Muster nicht, so gibt die Funktion 'None' zurück. Im Erfolgsfall gibt die Funktion ein Tuple Struct zurück. `usize` im Tuple Struct (.0) ist die Zahl, die bei der Multiplikation den gesuchten Hash ergibt und der Hash (.1) selbst ist der `String`

Benutzen Sie dazu die hash Methode aus der Datei `hasher_sha256.rs`, die wiederum das externe [Crate sha2][] dafür benutzt.

Schreiben Sie ein Display Trait für Solution, welche die `Solution` folgendermaßen ausgibt:

```text
Number: 110918 --> hash: 0e5e21f9f25b01c820a68025f2cffeb44df1649dbc3c8c1f4932518080651234
```

Suchen Sie mit Hilfe der *verify_product()* Funktion in *main()* solange nach der Zahl, bis diese das gewünschte Muster im Hash liefert.

### Parsen der Argumente

Um die Parameter beim Aufruf des Programms auszuwerten, benutzen Sie das externe [Crate clap][]. Halten Sie aber die Funktion *main()* so übersichtlich wie möglich. Lagern Sie dazu die benutzte Funktionalität des Crates clap in einer eigenen Funktion aus.

Die folgende Beispielausgabe von **cargo run -- --help** soll Ihnen nur als Richtlinie dienen:

```text
Running `target/debug/task1 --help`
Proof of Work Mechanism 0.1
Satoshi Nakamoto
now the basics in Rust, no C anymore

USAGE:
    task1 [FLAGS] <base> <difficulty> [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -v               Sets the level of verbosity
    -V, --version    Prints version information

ARGS:
    <base>          Sets the base to use
    <difficulty>    Sets the difficulty to use

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    timings    controls timing features
```

Der `-v` Parameter ist optional, muss somit nicht implementiert werden.

Beim Aufruf **cargo run -- 42 12345** erhalten Sie als Ausgabe:

```text
Using base: 42
Using difficulty: 12345
Please wait ...
Number: 567621 --> hash: b6bea2a40ed1bd6d9999b2232072092f3df0e02c4b507aa3ad947367b9712345

```

### Subcommand 'timings'

Das Subcommand 'timings' soll die Zeit ausgeben, die für das Suchen des Hashwertes benötigt wurde. Nutzen Sie dazu das [Crate time][]. Nur wenn beim Aufruf des Programms zusätzlich das Subcommand timings angegeben wird, wird eine Zeitmessung ausgeführt, welche als Ergebnis die Laufzeit der Suche ausgibt. Achten Sie darauf, wirklich nur die 'Suche' zu vermessen.

Beim Aufruf **cargo run -- 42 12345 timings** erhalten Sie als Ausgabe:

```text
Using base: 42
Using difficulty: 12345
Please wait ...
Number: 567621 --> hash: b6bea2a40ed1bd6d9999b2232072092f3df0e02c4b507aa3ad947367b9712345
(Duration: 9s / 9871ms / 9871841us)
```

Geben Sie das erhaltene Ergebnis jeweils in ganzen

- Sekunden
- Millisekunden und
- Mikrosekunden

aus.

### Tests

Binden Sie die `unit_tests.rs` ein. Ebenfalls befindet sich ein Test in der Datei `hasher_sha256.rs`. 

Bei den bats Tests ist ein "timeout"-Test dabei. Dieser schlägt zu, wenn Sie kein korrektes Muster als Parameter angeben. Der Test benutzt das Programm 'timeout', welches auf Linux Systemen installiert ist.

### Optimierung

Nutzen Sie das Unix Tool **time**, um zusätzliche Ausgaben über die Laufzeit Ihres Programms zu erhalten (Antworten in die Datei `ANSWERS.md`):

- Wie ist die Ausgabe von **time** zu interpretieren?
- Welche Laufzeiten erhalten Sie, wenn Sie `--release` benutzen?

### Laufzeit Messungen

Welche (ungefähre) Abhängigkeit der Laufzeit von der Difficulty können Sie im --release Mode bestimmen? Nutzen Sie für Ihre Messungen unterschiedliche base und difficulties (Diskussion Ihrer Ergebnisse in `ANSWERS.md`)

[Crate clap]: https://docs.rs/clap/
[Crate sha2]: https://docs.rs/sha2/
[Crate time]: https://docs.rs/time/

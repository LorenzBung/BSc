# Homework hw2 task2

## Inhalt

- [Überblick](#%C3%BCberblick)
- [Struktur](#struktur)
- [Funktionen](#funktionen)
  - [*parse_arguments()*](#parsearguments)
  - [Programmargumente in Rust](#programmargumente-in-rust)
  - [Warmup Übung](#warmup-%C3%BCbung)
  - [Argumente parsen](#argumente-parsen)
  - [Parsen ohne Fehlerbehandlung](#parsen-ohne-fehlerbehandlung)
  - [Parsen mit Fehlerbehandlung](#parsen-mit-fehlerbehandlung)
  - [Ablauf des Programms](#ablauf-des-programms)
- [Restructuring](#restructuring)
  - [Parsen der Config als Methode](#parsen-der-config-als-methode)
  - [Tests](#tests)
  - [Dokumentation](#dokumentation)

## Überblick

In dieser Aufgabe sollen Sie ein Programm entwickeln, welches 2 Parameter
übergeben bekommt:

- *1.* übergebene Parameter: Zu suchendes Zeichen
- *2.* übergebene Parameter: String in dem die Zeichenkette gesucht werden soll.

Das Programm gibt die Anzahl gefundener Treffer aus.

```text
You asked me to count all 'e' in "♥ The quick brown fox jumps over the lazy dog. ♥"
Found 3 'e' in "♥ The quick brown fox jumps over the lazy dog. ♥"
```

Neben einer klaren Struktur für das Programm soll eine kompetente
Fehlerbehandlung implementiert werden, sodass das Programm robust gegenüber
Eingebefehlern wird.

Ziele:

- Parameter einlesen können
- Weitere String-Funktionen kennen lernen
- Fehlerbehandlung

## Struktur

Das Programm soll aus 3 Funktionen bestehen:

- *main()*
- *run()*
- *parse_arguments()*

Die *main()* Funktion ruft zuerst die *parse_arguments()* auf, um danach
geeignet die *run()* Funktion zum Suchen der Zeichenkette zu starten.

## Funktionen

### Programmargumente in Rust

Bisher waren in Ihren Programmen die Parameter fest codiert. Wie kann man in
Rust die Parameter beim Aufruf des Programms auswerten?

Dazu existiert das [std::env][std::env] Modul, welches Strukturen und Funktionen
für die Prozessumgebung (process environment) bereit stellt.

Die Funktion, die die Parameter zur weiteren Auswertung liefert heißt
[`args()`][args]. Diese Funktion liefert einen Iterator mit Namen [`Arc`][Arc]
zurück. Was Iteratoren genau sind und was damit alles anzustellen ist werden wir
später im Semester kennen lernen. Zunächst kann man sich einen Iterator einfach
als eine 'Datenquelle' vorstellen, die - wenn wir darauf zugreifen - eine Serie
von Werten liefert. In unserem Fall liefert ein Zugriff auf [`args()`][args] die
dem Programm übergebenen Parameter als Strings, sofern welche übergeben wurden.

Und damit sind wir auch schon mitten in der Fehlerbehandlung. Auf mögliche
Fehler wird in der Dokumentation bereits hingewiesen. Wir werden im weiteren von
UNIX Systemen und korrektem unicode ausgehen, sodass uns ein paar
Fehlerbehandlungen erspart bleiben.

Um in unserem Programm auf die Parameter zugreifen zu können, werden wir diese
in einer geeigneten Datentyp speichern. Es bieten sich Arrays oder Vektoren an.

#### Warmup Übung

1. Warum sollten Sie für die Argumente den Datentyp Vector vorziehen?
1. Schreiben Sie die Funktion:

```rust
/// Prints elements of Vec
///
///
/// This function will print all elements of Vec with "args found: <elem>" in
/// each line
///
/// Returns nothing
fn print_arguments(args: &Vec<String>)
```

Die Funktion gibt den Inhalt des übergeben Vektors zeilenweise aus.

In der Main Funktion lesen Sie die bei Programmstart übergebenen Parameter ein
und erstellen einen Vektor aus den eingelesenen Parametern, den Sie als Referenz
übergeben.

> Die Methode [*collect()*][collect] ist Teil des Iterator Traits und bietet
> sich an um den Vector zu füllen. Überlegen Sie sich auch, warum es sinnvoll
> ist den Vector an die Funktion *print_arguments* nur zu leihen ('&')?

Wenn Sie nun Ihr Projekt mit `cargo run a b c`starten, so erhalten Sie folgende
Ausgabe:

```text
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/task1 a b c`
args found: target/debug/task1
args found: a
args found: b
args found: c
```

### Argumente parsen

Unsere Argumente speichern wir am Besten in einer eigenen Datenstruktur
*Config*. Dann sind unsere Parameter als entsprechende Datentypen einfach
verfügbar.

```rust
/// a struct to hold all of our configuration
#[derive(Debug,PartialEq)]
struct Config{
    search: char,
    line: String,
}
```

> Mit der *derive* Anweisung lassen wir den Rust Compiler etwas Magie anwenden.
> Denn durch diese Makro Anweisung erhält unsere Datenstruktur automatisch 2
> Traits wodurch wiederum automatisch die nötigen Methoden für uns erzeugt
> werden um unsere Struct zu printen und deren Inhalt vergleichen zu können
> ("==" Abfrage). Letzteres brauchen wir für die Tests.

#### Parsen ohne Fehlerbehandlung

1. Implementieren Sie die Funktion

```rust
/// Parses relevant arguments, returning the filled Config
///
///
/// This function will parse the relevant arguments from the
/// given <Strings>.
/// Returns Config
fn parse_arguments_simple(args: &Vec<String>) -> Config
```

Sie müssen nun in der Funktion:

- den 1. String in einen char verwandeln
- den 2. String der Config  Struktur zuweisen

>Beachten Sie, das die Strings, die die Argumente repräsentieren in der Funktion
>geliehen sind und somit jemand anderen gehören. Ihre Config Struktur benötigt
>somit EIGENE Strings.

In *main()* geben Sie die geparsten Parameter aus, sodass Sie bei Aufruf Ihres
Programms "cargo run a b c" folgende Ausgabe erhalten:

```text
    Finished dev [unoptimized + debuginfo] target(s) in 0.27 secs
     Running `target/debug/task1 a b c`
args found: target/debug/task1
args found: a
args found: b
args found: c
Config { search: 'a', line: "b" }
```

#### Parsen mit Fehlerbehandlung

1. Unter Umständen ruft ein Benutzer Ihr Programm ohne die benötigten Parameter
   auf. Welche Fehlermeldung erhalten Sie bei dem Aufruf Ihres Programms ohne
   Parameter und warum?

1. Ein Programmabsturz wäre die falsche Antwort darauf. Besser ist beim
   Auftreten von Eingabefehler ein entsprechender Hinweis, wie das Programm zu
   verwenden ist. Implementieren Sie die Funktion

```rust
/// Parses relevant arguments, returning the filled Config in Result
///
///
/// This function will parse the relevant arguments from the
/// given <Strings>.
/// Returns Config or Error Message in Result
fn parse_arguments(args: &Vec<String>) -> Result<Config, xx >
```

Implementieren Sie die folgende Fehlerbehandlung:

- Werden vom Benutzer zuwenig übergeben, so gibt die Funktion den Fehlerstring
  "not enough parameters" zurück. Werden vom Benutzer zu viele Parameter
  übergeben, so wertet die Funktion nur die ersten beiden aus. In der API der
  Funktion oben ist als Typ xx für den 'String' angedeutet und muss entsprechend
  von Ihnen mit dem richtigen Typen versehen werden.

- Tritt ein Fehler beim Wandeln des 1. Parameter in einen char auf, so geben Sie
  den String "char mismatch" als Fehlerstring in Result zurück. Wird statt eines
  chars als 1. Parameter ein String übergeben, so werten Sie nur den ersten
  Buchstaben davon aus.

Die main() Funktion kann keine Return Werte zurückliefern. Um aber der
aufrufenden Shell den Abbruch des Programms mitteilen zu können steht die
Funktion *[process::exit()]* aus der Standardbibliothek zu Verfügung. Durch die
Rückgabe von '1' signalisieren Sie der Shell, dass das Programm abgebrochen
wurde.

### Zeichen im String suchen

In Ihrer vorherigen Homework haben Sie bereits eine ähnliche Funktionalität
implementiert, die Sie nun in der *run()* Funktion übernehmen können.

### Ablauf des Programms

Zur Abgabe des Programms soll die  *main()* Funktion nur:

- *parse_arguments()* und
- *run()*

benutzen. In diesen beiden Funktionen darf kein *unwrap()* mehr verwendet
werden.

Da die Tests jedoch auch die Funktion *parse_arguments_simple()* benutzt, darf
diese nicht auskommentiert werden.

#### Dead Code

Um die Compiler Warnungen vor nicht benutzen Funktionen auszuschalten, fügen Sie
direkt in der Zeile vor der Funktion folgende Anweisung hinzu

```Rust
#[allow(dead_code)]
fn parse_arguments_simple(args: &Vec<String>) -> Config {
...
```

## Restructuring

Bisher ist alles in unserm `src/main.rs` Modul, wodurch dieses im Laufe der Zeit
immer unübersichtlicher werden würde. Ziel ist es nun, das meiste aus der
`main.rs` auszulagern, sodass nur noch die *main()* Funktion in `main.rs` steht.

Ausgelagert wird der Code in eine Bibliothek, genauer gesagt in die Datei
`src/lib.rs`.

>Um den Code aus `lib.rs` in `main.rs` verwenden zu können, müssen Sie ihre
>derzeitige 'Create' mit den entsprechenden 'extern' und 'use' Anweisungen in
>`main.rs`, erreichbar machen.

### Parsen der Config als Methode

Die *parse_arguments()* Methode steht noch in keinem Zusammenhang mit der
Struktur, die die Methode mit Daten füllt.

Schreiben Sie einen Konstruktor *new()* für die Datenstruktur *Config* und
ersetzten Sie damit die *parse_arguments()* Funktion. Verwenden Sie nun Config
geeignet in Ihrer *main()* Funktion.

### Tests

Die Test sind ausgelagert in die Datei `tests/task2.rs`

### Dokumentation

Dokumentieren Sie die Funktionen in Ihrer `lib.rs`.

Da Sie im `src/`Verzeichnis eine `main.rs` und eine `lib.rs` vorliegen haben, beschwert sich **cargo doc**, dass es nicht weiss wie es die Dokumentation erstellen soll:

```text
error: cannot document a package where a library and a binary have the same name. Consider renaming one or marking the target as `doc = false`
```

Damit **cargo doc** Ihre Dokumentation für die Funktionen in `lib.rs` erstellt,
benötigen Sie folgende Erweiterung in Ihrer `Cargo.toml` Datei.

```text
[[bin]]
doc = false
name = "task2"
```

Damit weiß cargo, dass es die Dokumentation aus `lib.rs` erstellen soll und
nicht aus `main.rs`.

[args]: https://doc.rust-lang.org/std/env/fn.args.html
[std::env]: https://doc.rust-lang.org/std/env/index.html
[Arc]: https://doc.rust-lang.org/std/env/struct.Args.html
[collect]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect
[process::exit()]: https://doc.rust-lang.org/std/process/fn.exit.html

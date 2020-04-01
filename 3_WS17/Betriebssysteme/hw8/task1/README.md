# Homework hw8 task1

- ['Proof-of-Work' als Bibliothek](#proof-of-work-als-bibliothek)
    - [Idiomatischer Code](#idiomatischer-code)
    - [`ANSWERS.md`](#answersmd)
- [Implementierung](#implementierung)
    - [Parameter _threads_](#parameter-threads)
    - [Flag _verbose_](#flag-verbose)
    - [Suchen mit mehreren Threads](#suchen-mit-mehreren-threads)
        - [-s Flag](#s-flag)
        - [-w Flag](#w-flag)
        - [Weitere Options](#weitere-options)
    - [`-help` Ausgabe](#help-ausgabe)
    - [Command `timings`](#command-timings)
    - [Tests](#tests)
    - [Laufzeit Messungen (im --release Mode)](#laufzeit-messungen-im---release-mode)

## 'Proof-of-Work' als Bibliothek

Transformieren Sie Ihr Binary Projekt in ein Library Projekt, wobei zusätzlich
ein CLI Programm bereit gestellt wird. Dieses hat seinen Ursprung in `main.rs`.
Nach der Transformation soll **cargo run** die Funktionalität Ihres `hw7/task1`
Programms aufzeigen, und **cargo test** durchläuft die Unit Tests der Library.
Erstellen Sie auf Basis der bisherigen "unit_test" Datei eine `tests/task1.rs`
Datei, die die `pub` Funktionen der Library testet.

### Idiomatischer Code

In dieser Aufgabe sollten Sie besonderen Wert auf Idiomatischen Rust Code legen.
Auch sollte Ihre `main.rs` Funktion möglichst übersichtlich bleiben. Lagern Sie
somit Funktionen in Module Ihrer Library oder für Ihr Binary aus. In den
vorherigen Homeworks haben Sie dazu alle nötigen Kniffe kennengelernt, die Sie
nun in Ihrer Lösung verwenden.

### `ANSWERS.md`

In den Aufgaben werden immer wieder Fragen gestellt. Geben Sie die Antworten
dazu in der Datei `ANSWERS.md`. Eine kurze aber nachvollziehbare Erklärung
(eigene Worte, kein Code) ist ausreichend. Aufbau und Struktur Ihrer
`ANSWERS.md` ist Ihnen überlassen. Überzeugen Sie uns Tutoren mit einer klaren
und verständlichen Struktur (und korrekter Formatierung). 

Kommentieren Sie Ihren Code. Beschreiben Sie in eigenen Worten (kein Copy/Paste
von Code oder Code Kommentaren) in `ANSWERS.md`, welche Arten der
Synchronisation und Kommunikation (außer MPSC) Sie einsetzen und wie Sie die
maximale Performance erreichen. Wichtig: Kommentare gehören in den Code,
Erklärungen in die `ANSWERS.md`.

Ihre Erklärung muss nachvollziehbar sein, ohne dass wir Tutoren Recherchen
starten müssen. Zum Beispiel die Antwort: "In der Implementierung wird die
Systemfunktion _dummy_bla_blub()_ benutzt" ist NICHT ausreichend. Ausreichend
wäre: "In der Implementierung wird die Systemfunktion _dummy_bla_blub()_
benutzt, die die Uhrzeit seit Einschalten des Rechners als UTF String liefert."

Messungen müssen auf Ihrem Container durchgeführt werden.

Und noch einmal: Achten Sie auf eine korrekte Formatierung in Markdown!

## Implementierung

### Parameter _threads_

Erweitern Sie das CLI Programm mit einem weiteren **OPTIONALEN** Parameter:
`threads`. Wird der Parameter nicht angegeben, analysiert das CLI Programm das
System und ermittelt die Anzahl der vorhandenen CPUs für 'threads'. Geben Sie
den Parameter an, können Sie diesen Wert somit 'überschreiben'.

Benutzen Sie dafür die externe [Crate sys-info](). Was ist der Unterschied
zwischen einer logischen und einer physikalischen CPU?

```text
Proof of Work Mechanism 0.2
Satoshi Nakamoto
now the basics in Rust, no C anymore

USAGE:
    task1 [FLAGS] <base> <difficulty> [threads] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <base>          Sets the base to use
    <difficulty>    Sets the difficulty to use
    <threads>       Sets the number of threads to use (default = number of cpus)

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    timings    controls timing features
```

### Flag _verbose_

Mit dem Parameter _-v_, _-vv_ oder _-vvv_ usw. lassen sich unterschiedlich viele
Zusatzinformationen ausgeben. 

Ohne die Angabe von verbose reduzieren Sie bitte die Ausgaben auf:

    Please wait ...
    Number: 567621 --> hash: b6bea2a40ed1bd6d9999b2232072092f3df0e02c4b507aa3ad947367b9712345

Wird das Flag -v angegeben, so soll ein Header mit Informationen zum System und
die Anzahl der Threads, die zur Suche benutzt werden, ausgegeben werden:

```text
--------------------------------------------
Container:    : "ct-bsys-ws17-0"
Physical CPUs : 4
Logical CPUs  : 4
CPU Speed     : 1700
Load Average  : LoadAvg { one: 2.4, five: 2.43, fifteen: 2.46 }
Processes     : 2690
--------------------------------------------
Searching with 2 threads!
Please wait ...
Number: 567621 --> hash: b6bea2a40ed1bd6d9999b2232072092f3df0e02c4b507aa3ad947367b9712345
```

Benutzen Sie für die Systeminformationen die externe [Crate sys-info][].

### Suchen mit mehreren Threads

Lagern Sie Ihre bisherige Suche in eine eigene Funktion aus. Schreiben Sie dann
eine neue Funktion, die das Suchen auf mehrere Threads aufteilt, wenn die Anzahl
der Threads >1 ist. 

Für die Aufteilung der Aufgabe bietet sich eine "Multiple Producer -> Single
Consumer" Lösung an. Viele Producer suchen nach dem Hash. Die zu untersuchende
'Menge' an Zahlen muss dazu geeignet auf alle Threads verteilt werden. Der
Producer, der ihn findet, sendet den Hash an den Consumer.  

Die Rust Standardbibliothek bietet dafür die [MPSC][] Synchronisationsprimitive
an. Einen ähnlichen Mechanismus haben Sie bereits mit der Pipe kennengelernt. 

Findet einer der Producer das Ergebnis, so sendet er dies an den Consumer. Der
Consumer hat dann die ruhmreiche Aufgabe, den Hash ausgeben zu dürfen und
beendet das Programm. 

#### -s Flag

Wird das Flag -s angegeben, so beenden die Producer-Threads die Suche, wenn ein
Ergebnis gefunden wurde. Synchronisieren Sie dazu alle Producer Threads über
eine geeignete Variable (kein Mutex, CV oder Semaphore). Die Producer-Threads
beenden sich somit wenn:

-   ein Ergebnis gefunden wurde
-   ein anderer Thread ein Ergebnis gefunden hat

Benutzen Sie die Synchronisation geeignet, um den Overhead durch die
Synchronisation gering zu halten. Ergänzen Sie Ihr Programm z.B. mit einer
weiteren eigenen 'Option' (Name frei wählbar), um Parameter an Ihrer
Synchronisation verändern zu können.

#### -w Flag

Wird dieses Flag angegeben, so wartet der Consumer Thread auf das Ende aller
Producer Threads. Das Flag macht natürlich nur Sinn, wenn das -s Flag angegeben
ist. Die Trennung von '-s' und '-w' dient in dieser Aufgabe zum
'Experimentieren'. 

#### Weitere Options

Sofern Sie weitere Optionen beim Aufruf Ihres Programms benötigen, ergänzen Sie
diese bitte mit einer knappen aber aussagekräftigen Hilfe beim Aufruf mit
`--help`. Nutzen Sie die `verbose` Ausgaben ('-vv' und '-vvv'), um den Ablauf
des Programms (und der Threads) zu 'loggen'. Das macht es für uns Tutoren als
auch für Sie einfacher, den Ablauf Ihres Programms zu 'debuggen'.

### `-help` Ausgabe

Mit den obigen Flags und Options sollten Sie nun folgende --help Ausgabe
erhalten:

```text
USAGE:
    task1 [FLAGS] [OPTIONS] <base> <difficulty> [threads] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -s, --sync       enables sync when solution found
    -v               Sets the level of verbosity
    -V, --version    Prints version information
    -w, --wait       consumer waits for all producers

OPTIONS:
        --special <VALUE>    sets special sync parameter (Default 0)

ARGS:
    <base>          Sets the base to use
    <difficulty>    Sets the difficulty to use
    <threads>       Sets the number of threads to use (default = number of cpus)

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    timings    controls timing features
```

### Command `timings`

Bei Angabe des Kommandos 'timings' wird die Gesamtzeit der Suche wie bisher
ausgegeben. Es genügt somit, bei der Standardausgabe (ohne -v) die Zeit der
Suche im Consumer Thread zu messen.

```text
...
Please wait ...
Number: 567621 --> hash: b6bea2a40ed1bd6d9999b2232072092f3df0e02c4b507aa3ad947367b9712345
(Duration: 2s / 2684ms / 2684903us)
```

Wird das Flag '-v' mit angegeben, so summiert der Consumer Thread die einzelnen
Suchzeiten der Producer Threads, sowie die insgesamt durchgeführten Loops zur
Suche. Überlegen Sie sich dazu, wie Sie die nötigen Informationen im Consumer
von den Producern erfahren. Eine Ausgabe der Ergebnis könnte folgendermaßen
aussehen:

```text
...
Sum Loops in Producers:       567600
Sum Duration in Producers:    10s / 10684ms / 10684637us
```

Diskutieren Sie die sich ergebenden Werte in `ANSWERS.md`

### Tests

Alle Integrations-Tests der Library liegen unter `tests/`.

Bei den bats Tests ist ein "timeout"-Test dabei. Dieser schlägt zu, wenn
Sie kein korrektes Muster als Parameter angeben. Der Test benutzt das Programm
'timeout', welches auf Linux Systemen installiert ist.

### Laufzeit Messungen (im --release Mode)

(Diskussion Ihrer Ergebnisse in `ANSWERS.md`)

Zeigen Sie Quantitativ den Performance-Gewinn gegenüber der Single-Threaded
Suche anhand einzelner Ergebnisse. Welche (ungefähre) Abhängigkeit der Laufzeit
von der Anzahl der Threads können Sie im --release Mode bestimmen? 

Nutzen Sie das Unix Tool **time**, um zusätzliche Ausgaben über die Laufzeit
Ihres Programms zu erhalten. Von Interesse sind nur Ihre `--release` Zeiten:

Die folgenden Fragen sollen Ihnen als Beispiel Ihrer Diskussion dienen, erheben
aber keinen Anspruch auf Vollständigkeit: 

-  Wie ist die Ausgabe von **time** zu interpretieren, wenn mehrere Threads
    laufen?
-   Welche unterschiedlichen Ergebnisse erhalten Sie bei der Option `timings`?
    Wie stehen diese im Zusammenhang mit den Ergebnissen von **time**?
-   Welche quantitativen Auswirkungen hat die Synchronisierung (Warum)? Welche
    Auswirkungen haben Ihre zusätzlichen Parameter?
-   Wie variieren `-s` und `-w` die Messungen? 
-   Welche sonstigen Arten der Synchronisierung bieten sich für die Problemstellung "Lösung gefunden (-s)" an? Was sind deren Vor- und Nachteile gegenüber Ihrer gewählten Lösung? 


[Crate clap]: https://docs.rs/clap/ 
[Crate sha2]: https://docs.rs/sha2/ 
[Crate time]: https://docs.rs/time/ 
[Crate sys-info]: https://docs.rs/sys-info/ 
[MPSC]: https://doc.rust-lang.org/std/sync/mpsc/

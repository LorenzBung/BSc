# Homework hw9 task2

[TOC]: #

# Table of Contents
- [1.1 Ziel](#11-ziel)
- [2.1 Aufgabe](#21-aufgabe)
    - [2.1.1 Optionen beim Aufruf](#211-optionen-beim-aufruf)
    - [2.1.2 Verhalten des Servers bei Kommandos](#212-verhalten-des-servers-bei-kommandos)
    - [2.1.3 Ausgaben des Servers auf der Konsole](#213-ausgaben-des-servers-auf-der-konsole)
    - [2.1.4 Unit Tests / Integration Tests](#214-unit-tests--integration-tests)
- [3.1 Fehlerbehandlung](#31-fehlerbehandlung)
- [4.1 Tipps](#41-tipps)

## 1.1 Ziel

Um die bisherige Mustersuche in einem Hashwert auf mehrere Rechner
auszuweiten, ist das Ziel dieser Aufgabe, einen kleinen TCP-Server zu
implementieren. Die ersten Kommandos des Server haben Sie bereits in
hw9:task1 implementiert.

Zuvor legen wir ein [Rust Workspace][] an, in welchem wir komfortabel
unsere eigenen Bibliotheken und ein Binary (den Server) verwalten.
Nutzen Sie die in hw9:task1 geschriebene Bibliothek, indem Sie diese als
`srv-commands` Bibliothek in task2 benutzen, um die Kommandos an den
Server zu parsen. Kopieren Sie dazu Ihre Dateien aus task1 in das
`srv-commands` Verzeichnis.

## 2.1 Aufgabe

Implementieren Sie den Webserver, in dem Sie [TCPListener][tcplistener]
verwenden.

Sorgen Sie in dieser ersten Ausbaustufe dafür, dass der Server 'Orders'
(Nachrichten) zwischen Anfragen speichert und in der Reihenfolge zurück
gibt, in der Sie gesendet wurden. ([FIFO][fifo])

Der Server darf beim Herunterfahren alle Daten verlieren. In dieser
Version Ihres Servers genügt es, wenn der Server immer nur eine Anfrage
zu jeder Zeit beantwortet.

### 2.1.1 Optionen beim Aufruf

Nun zu einer weiteren Lib im Workspace: `srv-config`. Diese Lib soll
einen geeigneten Config Typen bereitstellen. Das Parsen und Auswerten
der beim Aufruf angegebenen Parameter geschieht komplett in dieser
Library. Exportiert wird nur der Config Typ und dessen Methoden (soweit
nötig). Zum Parsen verwenden Sie wieder die externe Crate [clap][].

```text
Multi Hash Server 0.1
M. Mächtel <maechtel@htwg-konstanz.de>
Does awesome things to find a hash with specific pattern

USAGE:
    task2 [FLAGS] [OPTIONS] --port <PORT> [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v               Sets the level of verbosity

OPTIONS:
    -a, --address <ADDRESS>    the address, where the server can be reached (127.0.0.1 is default)
    -p, --port <PORT>          the port, where the server can be reached (7878 is default)

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    test    controls testing features
```

#### 2.1.1.1 `-v` Option

Mit der `-v` Ausgabe gibt der Server folgende Informationen aus:

- Beim Starten des Servers die Parameter, mit denen er läuft (verbose,
  port, address, test, ...)
- Den CONTROL-String, den der Server im CONTROL Kommando empfängt.
    ```text
    ...
    Received Control: Beam me up!
    ...
    ```

Benötigen Sie darüber hinaus weitere Ausgaben, so geben Sie diese bitte
nur bei -vv, -vvv usw. aus.

#### 2.1.1.2 `-a` Option

Mit der optionalen `-a` Option wird die Default Adresse: 127.0.0.1
überschrieben.

#### 2.1.1.3 `-p` Option

Mit der erforderlichen (required) `-p` Option wird der Default Port: 7878
überschrieben.

#### 2.1.1.4 `test` Option

Wird diese Option mit angegeben, dann erstellt der Server automatisch 3
Orders, die durch das **RETRIEVE** Kommando sofort nach dem Start des
Servers abgerufen werden können.

```text
echo "RETRIEVE" | nc localhost 7878
Test1
echo "RETRIEVE" | nc localhost 7878
Test2
echo "RETRIEVE" | nc localhost 7878
Test3
```

### 2.1.2 Verhalten des Servers bei Kommandos

- Kommando **STAGE**: Der Server speichert die Order
- Kommando **CONTROL**: Der Server gibt in dieser Aufgabe lediglich den
  Control String aus, sofern die `-v` Option gesetzt ist. Die Ausführung
  von Kommandos ist in dieser Aufgabe noch nicht zu implementieren.
- Kommando **RETRIEVE**: Der Server gibt die älteste Order zurück.
  Die Order steht danach auf dem Server nicht mehr zur Verfügung.
  Sind keine Order vorhanden, so sendet der Server dem Client "No
  order on stage"

    ```text
    $ nc localhost 7878
    RETRIEVE
    No order on stage!
    ```


### 2.1.3 Ausgaben des Servers auf der Konsole

Eventuell auftretende Fehler gibt der Server auf der Konsole aus, ohne
das Programm zu beenden. So wird z.B bei einem unbekannten Kommando der
entsprechende Error Ihrer `srv-commands` Bibliothek ausgegeben, die
folgende Form haben könnte:
```text
...
Error occurred: ParseError(UnknownCommand)
...
```

Wird die Option -v angegeben, dann gibt der Server zusätzliche
Informationen aus (siehe obiges Kapitel dazu).

### 2.1.4 Unit Tests / Integration Tests

Ergänzen Sie Ihren Code mit entsprechenden Tests, um die Funktionalität
in Ihren Libraries und Ihrem Server zu testen.

## 3.1 Fehlerbehandlung 

Wie bereits mehrfach erwähnt, geschieht auch in Rust die
Fehlerbehandlung wie mit Javas checked Exceptions, indem spezielle
Fehler und Ergebnisse in generische Fehler und Ereignisse umgeformt
werden, um diese dann nach 'oben' durch zu reichen. Auf der höchsten
Ebene werden dann die Fehler behandelt. Bei Rust kommt noch dazu, dass
man Results auch lokal in einer Funktion kombiniert, um dann alle Fehler
innerhalb dieser Funktion (Top-level Handler) zu behandeln.

Als Basiswerkzeug haben Sie dazu bereits die Traits
`[std::convert::From][]` und `[std::convert::Into][]` kennen gelernt. Es
genügt eines davon zu implementieren.

Wenn man aber nicht einfach nur Fehler durchreichen möchte, sondern
volle Results, muss man von Result<T,E> zu dem entsprechenden
Result<U,F> der aufrufenden Funktion kommen. Dazu kann man zu
'Result-Kombinatoren' greifen. Die Funktion `map` wandelt den Ok-Typ in
einen anderen um, `map_err` wandelt den Fehlertyp. Möchte man also
beides, sieht der Aufruf folgendermaßen aus:

```rust
res.map(....).map_err(....)
```

Das ist derzeit noch 'aufwendig' als Syntax zu schreiben, entsprechende
RFC's, um dies zu vereinfachen existieren bereits (vgl. Evolution der
Fehlerbehandlung in Rust: match()->try!->?).

Ihre Lösung dieser Aufgabe hat somit idealerweise einen eigenen
Fehlertypen, der wiederum den 'ParseError'-Typen aus hw9:task1 als auch
den/die in dieser Aufgabe auftretende Fehlertypen enthält.

## 4.1 Tipps

* Zum Speichern eignet sich die [VecDeque][vecdeque], mit der sich das oben genannte FIFO-Verhalten gut abbilden lässt.
* Früher an später denken: gute Aufteilung von Aufgaben in Funktionen,
  Module und Bibliotheken machen die folgenden Aufgaben einfacher.
* Zum Testen eignet sich **netcat**:

```
echo "STASH My privat hash: :) !" | nc 127.0.0.1 7878
echo "COMMAND Beam me up!" | nc 127.0.0.1 7878
echo "RETRIEVE" | nc 127.0.0.1 7878
```

Eine Kurzinfo zu **netcat** finden Sie auch in den [tldr Seiten][] zu
allen Unix Kommandos . Für ausführlichere Informationen benutzen Sie die
`man page` in Ihrem Container.

[tcplistener]: https://doc.rust-lang.org/std/net/struct.TcpListener.html
[fifo]: https://en.wikipedia.org/wiki/FIFO_(computing_and_electronics)
[vecdeque]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html
[std::convert::From]: https://doc.rust-lang.org/std/convert/trait.From.html
[std::convert::Into]: https://doc.rust-lang.org/std/convert/trait.Into.html
[Rust Workspace]: https://doc.rust-lang.org/book/second-edition/ch14-03-cargo-workspaces.html
[clap]: https://docs.rs/clap/
[tldr Seiten]: https://tldr.ostera.io/
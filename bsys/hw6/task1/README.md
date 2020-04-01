# README hw6-t1

Schreiben Sie ein Programm, welches 2 Kindprozesse erzeugt. Der Elternprozess
übergibt an die beiden Kindprozesse über eine Pipe die Daten als Strings, die
als Parameter beim Aufruf mit angegeben werden.

Bei Aufruf des Programms geben Sie eine Zahlensequenz (`i32`) von mindestens 2
Zahlen an. Diese Zahlensequenz übergibt dann der Elternprozess an die beiden
Kindprozesse. Die Zahlen, die an die Kindprozesse übergeben werden, sind alle vom Typ `i32`.

Werden zuwenig Parameter angegeben, so soll eine(!) Zeile 'Hilfe' ausgegeben
werden, wie z.B.:

```text
Correct usage: number number <number> ...
```

> Wichtig: Nur eine Zeile 'Hilfe' ausgeben.

Kindprozess1 berechnet aus der Zahlensequenz die Summe, Kindprozess2 das
Produkt. Kindprozess1 und Kindprozess2 geben dann das Ergebnis auf der Konsole
direkt aus.

```text
$ ./task1 -1 2 4 6 19 -100
sending to each child: -1 2 4 6 19 -100
Sum = -70
Mul = 91200
```

> Format der Ausgabe genau einhalten. Das bedeutet, die Ausgabe des
> Kindprozesses mit der Summenberechnung muss immer VOR der Ausgabe des
> Kindprozesses mit der Multiplikation erfolgen. Welchen einfachen, allerdings
> Laufzeit 'schädlichen' Trick können Sie dafür benutzen, den Sie bereits
> kennen? Wie lässt sich der Parameter dafür *tunen*, um eine optimale Laufzeit
> (kurz!) des Programms zu erreichen. Kennen Sie eine Möglichkeit, ohne diesen
> Trick eine eindeutige Reihenfolge der Kindprozesse vorzugeben (muss nicht
> implementiert werden)?

Werten Sie den Status der Kindprozesse im Elternprozess aus und beenden Sie das
Programm nur im Erfolgsfall beider Kindprozesse mit dem Exit-Code 0. Erfolgsfall
bedeutet dabei, dass der Kindprozess beendet wurde und den Exit-Code 0 gesendet
hat. Treten Fehler im Kindprozess auf, so sendet der Kindprozess den Exit-Code
1, so dass der Elternprozess dies auswerten kann und das Programm
ebenfalls mit Exit-Code 1 beendet.

>Tip: waitpid() sollte dazu entsprechend ausgewertet werden.

Beim Aufruf über cargo die '--' nicht vergessen, siehe dazu  **cargo help run**.

Die Daten zwischen Eltern und Kindern werden als Byte-Stream gesendet. Achten
Sie auf die Größe des Puffers, den die Kindprozesse zum Empfangen anlegen.
Definieren Sie für diese Größe eine `const` und setzen Sie diese auf 256.

Da die eingelesenen Argumente bereits als Strings vorliegen, bietet es sich an
im Programm intern mit Strings zu arbeiten. Damit ergeben sich folgende
Hilfsfunktionen:

- `concatenate_strings()` :
- `split_into_strings()`:
- `sum_strings()`:
- `mul_strings()`:

> Achten Sie auf einen geeignet großen Rückgabewert im Erfolgsfall,
> insbesondere in der Funktion, die die Multiplikation ausführt
> (`mul_strings()`).

> Achten Sie aber bei Ihren Berechnungen auch darauf, das kein Overflow
> auftreten kann, der das Programm abbricht (siehe Rust Standarddoku:
> `overflowing`). Wenn einen Berechnung aufgrund eines auftretenden Overflows
> nicht durchgeführt werden kann, so gibt der Kindprozess einen Fehler aus und
> terminiert - wie immer im Fehlerfall -  mit dem Exit-Code '1'. Die Ausgabe des
> Programms könnte dann z.B. folgendermassen aussehen:

```text
> cargo run -- 1000 1000 1000 1000 1000 1000 1000
...
Sum = 7000
... Overflow would happen in mul_strings()
...
```

> ... steht für mögliche andere Ausgaben

Darüber hinaus sollten Sie bei Code-Wiederholungen prinzipiell immer überlegen,
welche weiteren Hilfsfunktionen sich dadurch anbieten.

## Externe Crates

Benutzen Sie für Ihre Implementierung nur die externe Crate `nix`.

## Module und Tests

Ob und wie Sie den Code in Module aufteilen wollen ist Ihnen überlassen.
Schreiben Sie jedoch Ihre Unit-Tests in der Datei `unit_test_pipe.rs` oder als
eigenständigen Test, der von 'cargo test' aufgerufen wird, siehe auch
[Testing][]. Einfache Tests können auch direkt in die Dokumentation 'codiert'
werden, siehe [Documentation Tests][]

Achten Sie beim Exit-Code des Elternprozesses darauf, dass dieser nur 0 ist,
wenn BEIDE Kindprozesse jeweils einen Exit-Code 0 zurücksenden.

## Dokumentation

Es ist ausreichend, wenn Sie Ihre Dokumentation im Code soweit ergänzen, dass
dieser nachvollziehbar ist. Eine Dokumentation über `cargo doc` muss nicht
erstellt werden.

[Testing]: https://doc.rust-lang.org/book/testing.html 
[Documentation Tests]:https://doc.rust-lang.org/book/testing.html#documentation-tests

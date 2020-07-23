# Homework hw7 t2

- [Migration der Shell in ein Library
  Projekt](#migration-der-shell-in-ein-library-projekt)
- [Eigener Fehler Typ](#eigener-fehler-typ)
- [Neue Commands](#neue-commands)
    - [execv](#execv)
    - [pipe](#pipe)
- [Kommentare und Tests](#kommentare-und-tests)
- [Externe Crates](#externe-crates)

## Migration der Shell in ein Library Projekt

Legen Sie ein Library Projekt an mit einer zusätzlichen `main.rs` Datei. Wandeln
Sie Ihre Shell Lösung aus hw6 in ein Library Projekt, welche mittels cargo run
die Shell - wie bisher - im interaktiven Modus startet.

Verschieben Sie Ihre Tests der Library nach `tests/task2.rs`.

## Eigener Fehler Typ

Definieren Sie einen eigenen Fehler Typen, den Sie in Ihren Funktionen benutzen.
Definieren Sie den `ShellError` und die zugehörigen Traits im Modul
`shellerror.rs`. Der eigene Fehler soll in allen Ihren Funktionen verwendet
werden, die einen Fehlertypen zurück geben.

## Neue Commands

### execv

Die Eingabe wird als Aufruf eines Programms, inkl. Parameter, ausgewertet und es
wird versucht, das zugehörige Programm zu starten (mit *execvp()*). Wird das
Programm nicht gefunden, so wird eine Fehlermeldung ausgegeben.

```text
bsys-shell /Users/maechtel$ ps 1
  PID   TT  STAT      TIME COMMAND
    1   ??  Ss    28:42.16 /sbin/launchd
bsys-shell /Users/maechtel$ cat blabla.txt
cat: blabla.txt: No such file or directory
bsys-shell /Users/maechtel$ cat testdatei.txt
Hello you snoopy user ....
bsys-shell /Users/maechtel$ file testdatei.txt
testdatei.txt: ASCII text
bsys-shell /Users/maechtel$ exit
exit
```

### pipe

Die Pipe (**|**) ist hier als Command angegeben, da sie eine neue Art des
Programmaufrufs implementiert. Beachten Sie dazu auch die Informationen aus der
Vorlesung, insbesondere die Reihenfolge, wie die Kommandos rechts und links der
jeweiligen Pipe aufzurufen sind. Scannen Sie somit die Eingabe nach Pipes und
teilen Sie den String entsprechend in Substrings auf. Die Substrings entsprechen
den einzelnen Programmen, die miteinander per Pipe verbunden werden. Es genügt,
wenn 2 Programme per Pipe verbunden werden. Eingaben wie **cat filename.txt |
grep grading | sort**, also mehr als eine Pipe, müssen nicht interpretiert und
implementiert werden.

## Kommentare und Tests

Kommentieren Sie Ihre Funktionen und schreiben Sie eigene Integration Tests in
tests/ soweit möglich. Die Dokumentation soll sich per **cargo doc** erstellen
lassen.

Einfache Tests (manchmal sagt ein Test zu einer Funktion mehr als viele
Kommentarzeilen ...) können auch direkt in die Dokumentation 'codiert' werden,
siehe [Documentation Tests][]

## Externe Crates

Benutzen Sie für Ihre Implementierung nur die externe Crate `nix`.



[Documentation Tests]:
https://doc.rust-lang.org/book/testing.html#documentation-tests
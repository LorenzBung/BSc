# Homework hw4 task1

- [Überblick](#%C3%BCberblick)
- [Daten aus dem /proc Verzeichnis lesen](#daten-aus-dem-proc-verzeichnis-lesen)
- [Externe Crate nutzen](#externe-crate-nutzen)
- [Optionalen Parameter parsen](#optionalen-parameter-parsen)
- [Aufgaben](#aufgaben)
    - [Externe Crate in Dependencies eintragen](#externe-crate-in-dependencies-eintragen)
    - [`readproc.rs`: Eigene Prozessinformationen auslesen](#readprocrs-eigene-prozessinformationen-auslesen)
    - [`pstree.rs`: Prozesskette ausgeben](#pstreers-prozesskette-ausgeben)
- [Kontrolle Ihres Repositories](#kontrolle-ihres-repositories)

## Überblick

Ziel dieser Aufgabe ist es, mittels des externen Crates `procinfo` einige
Prozessverwaltungs-Informationen kennen zu lernen, die das Betriebssystem beim
Ausführen der Prozesse verwaltet. Fertige Programme wie **ps**, **htop** oder
**pmap** verwenden diese Informationen, um über das System und Tasks Auskunft zu
geben.

Die Funktionalität der Programms wird auf die 3 Module aufgeteilt:

- `main.rs`: die Funktion `main()
- `readproc.rs`: Handling der Funktionen, die angefragte Informationen aus dem
  `proc/` Verzeichnis zu Verfügung stellen.
- `pstree.rs`: Struct und Methoden, um einen 'pstree' darzustellen

## Daten aus dem /proc Verzeichnis lesen

"Das `/proc`-Verzeichnis ist kein wirkliches Dateisystem, sondern eine
Schnittstelle zum Kernel. Die Dateien, die in diesem Verzeichnis liegen,
benutzen keinen Speicherplatz auf der Platte, sind aber trotzdem les- und in
manchen Fällen auch beschreibbar.

Seinen Namen trägt dieses Verzeichnis daher, dass es für jeden laufenden Prozess
ein Unterverzeichnis bereithält, das Informationen über diesen Prozess zur
Verfügung stellt. Das Unterverzeichnis trägt als Namen die ProzessID (PID) des
jeweiligen Prozesses. Es enthält unter anderem folgende Dateien:

- `cmdline` Die Kommandozeile, mit der der Prozess gestartet wurde, mit allen
  verwendeten Parametern.
- `cwd` (current working directory) Ein symbolischer Link auf das Verzeichnis,
  das beim Aufruf des Prozesses das aktuelle Arbeitsverzeichnis war.
- `environ` Die komplette Umgebung des Prozesses (Variablen, Funktionen usw.)
  sofern er eine Umgebung hat.
- `exe` Ein symbolischer Link auf das aufgerufene Programm, dass den Prozess
  ausmacht.
- `root` Ein symbolischer Link auf das Verzeichnis, das für den Prozess das
  Wurzelverzeichnis darstellt.

Daneben finden sich weitere Informationen zu den verwendeten Ressourcen
(Speicher, Libraries) und ein Unterverzeichnis **fd**, das die File-Descriptoren
aller vom Prozess verwendeten Dateien enthält.

Diese Information wird beispielsweise von den Programmen verwertet, die
Prozess-Informationen ausgeben." (aus [Referenz][])

Das `/proc` Verzeichnis steht nur unter Linux zu Verfügung. Daher müssen
Programme, die auf `/proc` zugreifen auch auf einem Linux System erstellt und
getestet werden.

## Externe Crate nutzen

Um nicht selbst im Programm auf die nötigen Dateien per File E/A im ´/proc´
zugreifen zu müssen, wird zum Auslesen eine externe Crate benutzt. Die Crate
*[procinfo][]* stellt dazu die nötigen Funktionen zu Verfügung. Die Funktionen
liefern in einem Result eine Datenstruktur, aus der wir die benötigten
Informationen komfortabel auslesen können. Verwenden Sie in Ihrem Programm
soweit wie möglich die Funktionen des externen Crates *procinfo*, auch wenn in
der Standard-Bibliothek ebenfalls Funktionen für einzelne Aufgaben zu Verfügung
stehen.

## Optionalen Parameter parsen

Der Optionale Parameter PID (u32) entscheidet darüber, ob Ihr Programm die
Funktionen des Moduls `readproc.rs` oder `pstree.rs` verwendet. Wird kein
Parameter angegeben, so werden die Funktionen des Moduls `readproc.rs` benutzt.

## Aufgaben

### Externe Crate in Dependencies eintragen

- Benutzen Sie für die folgenden Aufgaben das *[procinfo][]* Crate.
- Fügen Sie dazu den notwendigen Eintrag unter `[dependencies]` in
  **Cargo.toml** hinzu.
- Um auf die nötige Funktionen des Crate zugreifen zu können schreiben Sie NUR
  in **main.rs** bitte folgenden Zeilen an den Beginn der Datei:

 ```Rust
 extern crate procinfo;
 ```

Benutzen Sie in Ihren Modulen `readproc.rs` und `pstree.rs` die `use` Anweisung
geeignet, um auf die nötigen Funktionen des Crate *procinfo* in den Modulen
zugreifen zu können.

In der Crate *procinfo* wird als Return ein Result Typ mit nur dem OK Typ
benutzt. Der Err Typ ist scheinbar nicht im Result enthalten. Wenn man
allerdings die Info der Standardbibliothek zu [io::Result][] liest, erfährt man,
dass es sich hier um einen Alias handelt, der komfortableres Arbeiten mit Errors
erlaubt. Dazu kommen wir aber erst in späteren Kapiteln, wenn wir uns dem `?`
Makro nähern. Daher bitte hier in der Aufgabe noch keine Makros wie `try` und
`?` benutzen.

### `readproc.rs`: Eigene Prozessinformationen auslesen

Ziel dieser Teilaufgabe ist es das Crate *procinfo* kennen zu lernen und sich
noch ein wenig mit dem Return Typ 'Result' und der Fehlerbehandlung zu üben.

Die folgenden Funktionen werden im Modul `readproc.rs` implementiert. Alle
Funktionen reichen mögliche Fehler über einen Result zurück zur aufrufenden
Funktion. Keine der Funktionen darf im Fehlerfall einen Programmabbruch
erzwingen.

1. Schreiben Sie die Funktion `fn self_pids() -> Result<(i32, i32), &'static
   str>`, die die eigene PID und die PPID in einem Tupel (PID,PPID) Im
   Erfolgsfall zurückgibt.

   > Hinweis: Überlegen Sie sich, ob `stat()` und der `stat_self()` die
   > geeignetere Funktion im *procinfo* Crate für diesen Fall ist.

1. Schreiben Sie die Funktion `fn get_pid_command(pid: i32) -> Result<String,
   &'static str>`, die den Command Namen zu einer PID zurück liefert. Wird die
   PID nicht gefunden im System, so soll die Funktion den String "PID not alive:
   no command name found" zurück geben.

1. Schreiben Sie die Funktion `fn get_last_created_command() -> Result<String,
   &'static str>`, die den Command Namen des zuletzt erzeugten Prozesses im
   System zurück gibt. Wird die PID nicht gefunden im System, so soll die
   Funktion den String "No last command via PID found" zurück geben.

   > Tip: `loadavg()` Funktion.

1. Schreiben Sie die Funktion `fn get_thread_count(pid: i32) -> Result<u32,
   &'static str>`, die die Anzahl der Threads pro PID zurückliefert. Wird die
   PID nicht gefunden im System, so soll die Funktion den String "PID not alive:
   no threads counted" zurück geben.

1. Benutzen Sie nun Ihre Funktionen geeignet, um in Ihrer `main()` Funktion
   folgende Ausgaben zu produzieren:

   ```text
    My PID : 31894 - process1 running 4 threads
    My PPID: 27952 - process2 running 2 threads
    Last process created in system was: process3
   ```

   > Hinweis: Die Nummer und Command Namen sind bei Ihnen verschieden. Wenn Sie
   > Probleme beim Aufruf Ihres Programms über **cargo run** haben lesen Sie
   > unbedingt die nächste Frage!

1. Sie können Ihr Programm über:

   - **cargo run** oder
   - **./target/debug/task1** starten.

   Überlegen Sie sich, wie es zu den unterschiedlichen Ausgaben und
   Programmverhalten kommt.

1. Schreiben Sie die Funktion `fn get_task_total() -> Result<u32, &'static
   str>`, die die Gesamtmenge aller Tasks im System zurück liefert. Wird die
   Gesamtmenge nicht gefunden, so soll die Funktion den String "No total count
   of tasks in system found" zurück geben.

   > Warum z.B. zeigt Ihnen das Programm **htop** eine andere Anzahl von
   > Prozessen als Ihre ausgelesene Funktion?

1. Schreiben Sie die Funktion `fn get_ownprocess_mem() ->
   Result<(usize,usize,usize), &'static str>`, die in einem Tuple die Werte für:

   - vsize
   - code und
   - data

   zurück liefert.

1. Im Modul `main.rs` produzieren Sie die Ausgabe für die Kommandozeile und
   kümmern sich um evtl. Fehler. Bei korrektem Programmablauf ergibt sich
   folgende Ausgabe (genau 5 Zeilen!):

   ```text
   My PID : 31894 - process1 running 4 threads
   My PPID: 27952 - process2 running 2 threads
   My mem : 3560 (vspace), 208 (code), 588 (data)
   Last process created in system was: process3
   Total number of tasks: 145
   ```

### `pstree.rs`: Prozesskette ausgeben

1. Auf Basis des Crate *procinfo* sollen Sie bei Aufruf des Programms mit einer
   PID, ausgehend von dieser PID der Tree bis zum aktuellen Programm ausgegeben
   werden. Wird somit als Argument eine `1` angegeben, wird eine Funktionalität
   des Kommandozeilen Programms **pstree / pstree.x11** nachgebildet. Wenn Sie sich z.B. mit
   dem Kommando **ps** die PID Ihrer aktuellen Shell anzeigen lassen, können Sie
   sich durch Aufruf von **pstree.x11 -p -s <pid>** die Kette aller Prozesse
   anzeigen lassen, in denen die <pid> enthalten ist.

   ```text
   systemd(1)---sshd(1264)---sshd(7161)---sshd(7198)---zsh(7199)---pstree.x11(47200)
   ```

   Genau diese Ausgabe soll nun Ihr Programm erzeugen bei der Option '1', wobei
   natürlich das Ende der Kette nicht mehr `pstree.x11` ist sondern Ihr Programm.

   Geben Sie im obigen Beispiel die PID 7161 als Parameter an, so soll Ihr Programm
   nur den Teil-Tree ausgegeben, startend vom Prozess mit PID 7161.

   ```text
   sshd(7161)---sshd(7198)---zsh(7199)---task1(47200)
   ```

   Ausgehend von der eigenen pid sollen alle Elternpid bis zum übergebenen PID
   (z.B. 1 für init Prozess, hier `systemd`) angezeigt werden. Der Init Prozess
   (hier systemd) hat immer die PID 1 in UNIX Systemen, aber unterschiedliche Namen.

1. Nutzen Sie zum parsen der PID im Argument die `parse()` Funktion. Behandeln
   Sie folgende Fehler:

   - Parameter ist keine Zahl
   - Parameter ist keine Elternpid
   - Mehr als 2 Parameter werden bei Aufruf mit angegeben.

   Geben Sie im Fehlerfall eine entsprechende Meldung in einer(1!) Zeile aus und
   beenden Sie das Programm mit dem Exitcode 1. Eine möglich Ausgabe:

   ```text
   $ ./task1 2 3
   Correct usage: no param or param PID
   ```

   >Wichtig: Im Fehlerfall beenden Sie das Programm kontrolliert mit exit(1).
   >Den Fehlercode '1' überprüfen die Tests in tests/output.bats!

1. Erstellen Sie eine eigene Datenstruktur und Methoden um die Aufgabe zu lösen.
   Verwenden Sie nur die externe Crate *procinfo* dazu. Die Ausgabe beim Aufruf
   Ihres Programms muss folgender Beispielformatierung entsprechen:

    ```text
    systemd(1)---sshd(1264)---sshd(7161)---sshd(7198)---zsh(7199)---task1(47151)
    ```
   > Je nachdem wie Sie Ihr Programm aufrufen wird es natürlich andere Ausgaben
   > produzieren. Durch das Kommandozeilen-Tool **pstree.x11** können Sie Ihre
   > Ausgabe kontrollieren!

1. Schreiben Sie eine eigene unit Test Datei `unit_test_pstree.rs`, die Ihre
   Funktionen im Modul `pstree.rs` geeignet testet und beim Aufruf von **cargo
   test** die Tests ausführt.

1. Schreiben Sie ausreichend Kommentare, um Ihre Implementierung in `pstree.rs`
   über die per **cargo doc** erstellten Dokumentation nachvollziehen zu können.

## Kontrolle Ihres Repositories

Haben Sie die Aufgaben komplett bearbeitet, so sollten sich folgende Dateien in
Ihrem HW (Homework) Verzeichnis befinden:

```text
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── src
│   ├── main.rs
│   ├── pstree.rs
│   ├── readproc.rs
│   ├── unit_test_pstree.rs
│   └── unit_test_readproc.rs
└── tests
    └── output.bats

2 directories, 9 files
```

[Referenz]: http://www.linux-praxis.de/lpic1/lpi101/proc.html
[procinfo]: https://docs.rs/procinfo/
[io::Result]: https://doc.rust-lang.org/std/io/type.Result.html

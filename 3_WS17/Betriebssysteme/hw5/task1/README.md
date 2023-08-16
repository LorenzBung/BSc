# Homework hw5 task1

- [1.1. Ziel](#11-ziel)
- [1.2. Externe Crate nutzen](#12-externe-crate-nutzen)
    - [1.2.1. Versionen der externen Crates](#121-versionen-der-externen-crates)
- [1.3. Aufgaben](#13-aufgaben)
    - [1.3.1. Optionale Parameter](#131-optionale-parameter)
    - [1.3.2. Externe Crate in Dependencies eintragen](#132-externe-crate-in-dependencies-eintragen)
    - [1.3.3. Die Funktion `pub fn run_zombie()`](#133-die-funktion-pub-fn-runzombie)
    - [1.3.4. Die Funktion `pub fn run_childs(start_pid: i32, arg: &str) -> Result<(), String>`](#134-die-funktion-pub-fn-runchildsstartpid-i32-arg-str---result-string)
- [1.4. Tests](#14-tests)
- [1.5. Dokumentation](#15-dokumentation)
- [1.6. Kontrolle Ihres Repositories](#16-kontrolle-ihres-repositories)

## 1.1. Ziel

Ziel dieser Aufgabe ist es, mittels des externen Crates *nix* einige systemnahe
Funktionen zur Prozesserzeugung und -synchronisierung kennen zu lernen.

Über optionale Aufrufparameter werden die verschiedenen Verhaltensweisen Ihres
Programms getrickert.

## 1.2. Externe Crate nutzen

Um möglichst direkt die Systemfunktionen zu nutzen, stellt das *[nix Crate][]*
eine einheitliche Schnittstelle dar. Insbesondere der Umgang mit der
Fehlerbehandlung ist unter Rust und der nix Crate um einiges komfortabler und
sicherer. Dazu stellt die nix Crate über den Result Typ die Informationen über
den aufgerufenen Systemcall zur Verfügung. Rust Datentypen werden in der nix
Crate überall dort benutzt wo dies Sinn macht. So werden z.B. Slices als
Standard benutzt, wenn Bereiche eines Puffers weitergegeben werden müssen.
Dieser Standard soll nun wiederum in C++ im C++17 Standard einfließen.

In dieser Aufgabe interessieren wir uns vor allem für das *nix* Modul
`nix::unistd` und `nix::sys::wait`. Darüber hinaus werden auch weitere Rust
Standard-Library Methoden benutzt, sowie die externe Crate *procinfo*
eingebunden.

### 1.2.1. Versionen der externen Crates

- *nix*: 0.9.0
- *procinfo*: 0.4.2

In Ihrer Lösung dürfen nur diese beiden externen Crates eingebunden werden.

## 1.3. Aufgaben

### 1.3.1. Optionale Parameter

Ihr Programm wird sich entsprechend der übergebenen Optionen unterschiedlich
verhalten:

- ohne Parameter: Aufruf der Funktion `pub fn run_zombie()`
- ein Parameter: Aufruf der Funktion `pub fn run_childs(start_pid: i32, arg:
  &str) -> Result<(), String>`

Details zu den einzelnen Funktionen erhalten Sie bei den entsprechenden Aufgaben
dazu.

### 1.3.2. Externe Crate in Dependencies eintragen

- Benutzen Sie für die folgenden Aufgaben das *[nix Crate][]*.
- Fügen Sie dazu den notwendigen Eintrag unter `[dependencies]` in
  **Cargo.toml** hinzu und benutzen in Ihrem Root Modul `main.rs` die
  entsprechende extern Anweisung.

### 1.3.3. Die Funktion `pub fn run_zombie()`

Wird das Programm ohne einen Parameter aufgerufen, erstellen wir als 'Belohnung'
für diesen 'müden' Programmaufruf einen Zombie! Dies geschieht im Modul
*zombie/mod.rs* über die dortige Funktion `pub fn run_zombie()`. Innerhalb
dieser Funktion behandeln Sie evtl. auftretende Fehler direkt mit
Programmabbruch. Dies gilt jedoch nur für diesen Aufgabenpunkt!

Was ist ein Zombieprozess?

>"Wenn ein Prozess einen neuen Prozess startet (mittels Forking), wird der alte
>'Elternprozess' und der neue 'Kindprozess' genannt. Wenn der Kindprozess
>beendet wird, kann der Elternprozess vom Betriebssystem erfragen, auf welche
>Art der Kindprozess beendet wurde: erfolgreich, mit Fehler, abgestürzt,
>abgebrochen, etc.
>
>Um diese Abfrage zu ermöglichen, bleibt ein Prozess, selbst nachdem er beendet
>wurde, in der Prozesstabelle stehen, bis der Elternprozess diese Abfrage
>durchführt – egal ob diese Information gebraucht wird oder nicht. Bis dahin hat
>der Kindprozess den Zustand Zombie. In diesem Zustand belegt der Prozess selbst
>keinen Arbeitsspeicher mehr (bis auf den platzmäßig vernachlässigbaren Eintrag
>in der Prozesstabelle des Kernels) und verbraucht auch keine Rechenzeit, jedoch
>behält er seine PID, die (noch) nicht für andere Prozesse wiederverwendet
>werden kann." ([Quelle Wikipedia][])

Die Funktion `run_zombie()` erstellen Sie im Modul *zombie/mod.rs*. Wenn Sie Ihr
Programm ohne Parameter aufrufen, erhalten Sie bei korrekter Zombie
'Generierung' folgende Ausgabe:

```text
PID   TTY      STAT   TIME COMMAND
27524 pts/1    Ss     0:00 -bash
27531 pts/1    S      0:00 zsh
27935 pts/1    S+     0:00 cargo run
27936 pts/1    S+     0:00 target/debug/task1
27962 pts/1    Z+     0:00 [task1] \<defunct\>
27963 pts/1    R+     0:00 ps t
```

Diese Ausgabe wird über das Programm **ps t** generiert, welches in Ihrem
Programm dann geschickterweise aufgerufen wird, wenn Sie den im vorherigen Text
beschriebenen Zustand durch Ihr Programm selbst hergestellt haben. Den
Zombizustand bekommen Sie als Z dargestellt, siehe auch [man ps][].

>Tipp: Um Ihr Programm einen Moment 'schlafen' zu lassen, stellt Ihnen Rust die
>Funktion [thread::sleep][] zur Verfügung. Ein Prozess besteht aus mindestens
>einem Thread (Main-Thread). Und da Sie keine weiteren Threads erstellen, lässt
>diese Funktion Ihren Prozess (bestehend aus einem Main-Thread) schlafen.

Um ein anderes Programm aus Ihrem Programm heraus starten zu lassen stellen
Ihnen die *nix* Crate und die `std Bibliothek verschiedene Funktionen` bereit:

- [Module nix::unistd][] der *nix* Crate: die Funktion execv(), execve() und
   execvp()

- [Module std::process][] der Standardbibliothek: Machen Sie sich mit der
   Benutzung von [Module std::process][] grob vertraut, insbesondere die
   Methoden des Typs [Command][]:

  - new
  - arg
  - spawn
  - output
  - status

Je nachdem für welche Bibliothek Sie sich entscheiden, experimentieren Sie zu
Beginn ein wenig mit den Code Beispielen der Dokumentation.

>Tipp: Das Command Interface der Standardbibliothek ist einfacher zu verwenden.

### 1.3.4. Die Funktion `pub fn run_childs(start_pid: i32, arg: &str) -> Result<(), String>`

Wird ein Parameter (`arg`) beim Aufruf Ihres Programms mit angegeben,
spezifiziert dieser Parameter die Anzahl der zu erzeugenden Kindprozesse.
Wichtig dabei ist, dass alle zu erstellenden Kindprozesse voneinander abhängen.
Rufen Sie im letzten erstellten Kindprozess Ihre bereits erstellte `pstree()`
Funktion mit der übergebenen PID des 1. Elternprozesses (`start_pid`) auf, so
können Sie aufgrund der ausgegebenen Liste erkennen, ob alle Kindprozess
voneinander abhängen. Ihr *pstree* Modul stellen Sie in *child/pstree.rs*
bereit, da dieses nur im Modul *child/mod.rs* benutzt werden muss.

> Die Pid aus dem **nix Crate** ist vom Typ `Pid`, welches ein Alias ist. Lesen
> Sie dazu die Dokumentation des nix Crates und benutzen Sie eine geeignete Rust
> Funktion (einer Trait), um diesen Typ als i32 an Ihre Funktion `run_childs()`
> weiterzugeben. Sie müssen in der **pstree Funktion** dazu nichts anpassen!

```text
> ./target/debug/task1 4
...
task1(28207)---task1(28233)---task1(28234)---task1(28235)---task1(28236)
...
```

Implementieren Sie die Funktion `pub fn run_childs(start_pid: i32, arg: &str) ->
Result<(), String>` im Modul *child/mod.rs*. Alle dazu nötigen Hilfsfunktionen
werden entweder in *child/mod.rs* oder entsprechenden Modulen im *child/*
Verzeichnis zur Verfügung gestellt. Evtl. auftretende Fehler werden an das Root
Modul (*main.rs*) zurückgegeben und dort behandelt. Bei dieser Teilaufgabe
müssen alle auftretenden Fehler entsprechend an das Root-Modul zurückgegeben
werden. Treten Fehler auf, so darf die Fehlermeldung, generiert im Root-Modul,
dazu max. 1 Zeile lang sein und der Exit-Code des Programms muss '1' sein.

Parsen Sie den übergebenen Parameter mit der `parse()` Funktion in einen `u8`
Typ! Es ist wichtig, dass Sie im weiteren die Anzahl der Kindprozesse über eine
`u8` Variable steuern!

Jedes Child soll eine Ausgabe machen, sowie jeder Parent, wenn sich der Child
beendet hat:

- Child: hello, I am child (\<pid\>)
- Eltern: I am \<pid\> and my child is \<child\>.  After I waited for
  \<waitstatuspid\>, it sent me status \<status\>

  - \<pid\> via getpid()
  - \<child\> = child pid
  - \<waitstatuspid\> = see Ok of waitpid()
  - \<status\> = see Ok of waitpid()

```text
> ./target/debug/task1 4
hello, I am child (pid:28233)
hello, I am child (pid:28234)
hello, I am child (pid:28235)
hello, I am child (pid:28236)

task1(28207)---task1(28233)---task1(28234)---task1(28235)---task1(28236)
I am 28235 and my child is 28236.  After I waited for 28236, it sent me status 0
I am 28234 and my child is 28235.  After I waited for 28235, it sent me status 0
I am 28233 and my child is 28234.  After I waited for 28234, it sent me status 0
I am 28207 and my child is 28233.  After I waited for 28233, it sent me status 0
```

>Leerzeile wichtig nach der Ausgabe aller Kinder!

## 1.4. Tests

Für das *tests/* Verzeichnis steht wieder eine *output.bats* Datei zur
Verfügung. Ausserdem erstellen Sie bitte eigene Unit Tests in einer eigenen zu
erstellenden *unit_tests.rs* Datei in Ihrem Crate Root Verzeichnis (*src/*).

## 1.5. Dokumentation

Erstellen Sie für alle Module und Funktionen eine kurze aber aussagekräftige
Dokumentation, und vergessen Sie nicht wichtige Passagen auch im Code zu
kommentieren. Als Tutoren sollte es uns möglich sein, schnell Ihre genialen
Lösungen nachvollziehen zu können.

## 1.6. Kontrolle Ihres Repositories

Haben Sie die Aufgaben komplett bearbeitet, so sollten sich folgende Dateien in
Ihrem HW (Homework) Verzeichnis befinden:

```text
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── src
│   ├── child
│   │   ├── mod.rs
│   │   └── pstree.rs
│   ├── main.rs
│   └── zombie
│       └── mod.rs
└── tests
    └── output.bats

4 directories, 8 files
```

[nix Crate]: https://docs.rs/nix/0.8.1/nix/
[Module nix::unistd]: https://docs.rs/nix/0.8.1/nix/unistd/index.html
[Module std::process]: https://doc.rust-lang.org/std/process/
[Command]: https://doc.rust-lang.org/std/process/struct.Command.html
[Quelle Wikipedia]: https://de.wikipedia.org/wiki/Zombie-Prozess
[thread::sleep]: https://doc.rust-lang.org/std/thread/fn.sleep.html
[man ps]: http://man7.org/linux/man-pages/man1/ps.1.html

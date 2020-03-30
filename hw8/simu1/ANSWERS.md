# hw8 - Simulation 1 - Antworten

1. Helgrind markiert die richtigen Stellen im Code:
    ```text
    ==1930== Possible data race during write of size 4 at 0x60207C by thread #1
    ==1930== Locks held: none
    ==1930==    at 0x400C9A: main (main-race.c:15)
    ==1930==
    ==1930== This conflicts with a previous write of size 4 by thread #2
    ==1930== Locks held: none
    ==1930==    at 0x400C5A: worker (main-race.c:8)
    ```
    Weiterhin erhält man die Information über den Lock (hier `none`), die Art des Problems (`Possible data race`) und die Größe der Daten, die geschrieben werden.

2. Codekorrektur

    1. Wenn ich eine der Zeilen lösche (bzw. auskommentiere), meldet helgrind keinen Fehler mehr (auch nicht für die zweite betreffende Zeile).

    2. Wenn man nur bei einer Variable den Lock setzt, erkennt helgrind den Fehler:
        ```text
        ==18432== Possible data race during write of size 4 at 0x602084 by thread #1
        ==18432== Locks held: none
        ==18432==    at 0x400CB2: main (main-race.c:18)
        ==18432==
        ==18432== This conflicts with a previous write of size 4 by thread #2
        ==18432== Locks held: 1, at address 0x6020A0
        ==18432==    at 0x400C68: worker (main-race.c:10)
        ```
    3. Wenn man bei beiden Variablen einen Lock setzt, findet helgrind wieder keine Fehler.

3. Vermutlich ist der Fehler, dass Thread 1 einen Lock auf v1 setzt, Thread 2 einen Lock auf v2, und anschließend warten beide Threads, bis der jeweils andere diesen Lock wieder freigibt. Auf der Labshell läuft das Programm. Dieser Deadlock tritt nur auf, wenn Thread 1 und Thread 2 wirklich absolut parallel laufen.

4. Helgrind gibt mir viele Informationen zur Lockreihenfolge. Es sagt, dass es durch das Festlegen des ersten Locks eine feste Reihenfolge gibt, in der neue Locks gesetzt werden müssen. Zusätzlich bekomme ich die Information, wo die Reihenfolge festgelegt wird und wo sie nicht eingehalten wird.

5. `main-deadlock-global.c`

    1. Das Problem der Lockreihenfolge sollte hier nicht auftreten, da im Worker ein globaler Lock gesetzt wird. Dieser blockiert während dem kritischen Bereich einfach alle anderen Threads.

    2. Beim Ausführen des Programms treten keine Fehler auf.

    3. helgrind sollte eigentlich keinen Fehler melden, da der globale Lock einen Deadlock verhindert.

    4. Da helgrind den Fehler dennoch erkennt, ist es ein sehr nützliches Werkzeug um Lockfehler zu erkennen.

6. Dieser Code ist sehr ineffizient, da der Parent-Thread durch seine while-Schleife die CPU zu 100% auslastet und somit zum einen sehr viel Strom verbraucht und zum anderen die anderen Threads ihrer Rechenzeit "beraubt". Dies ist besonders auffällig, wenn das Child sehr lange benötigt.

7.

    1. helgrind meldet einen Data-Race-Fehler in der Variable `done`:
        ```text
        ==1064== Possible data race during read of size 4 at 0x602084 by thread #1
        ==1064== Locks held: none
        ==1064==    at 0x400CEB: main (main-signal.c:16)
        ==1064==
        ==1064== This conflicts with a previous write of size 4 by thread #2
        ==1064== Locks held: none
        ```
        Dieser Fehler wird gemeldet, da der Kindthread in `done` eine 1 schreibt, sobald er fertig ist, im Elternthread jedoch eine while-Schleife mit dieser Variable läuft.

    2. Trotz der Meldung von helgrind ist der Code korrekt, da im Elternthread die Variable `done` nie geschrieben wird, sondern nur gelesen wird. "Verpasst" die while-Schleife die Änderung in `done`, so spielt das keine Rolle und die Schleife läuft einfach noch ein letztes Mal durch.

8. `main-signal-cv.c`

    1. Diese Version ist besser als die alte, da der Elternthread nicht mehr die CPU blockiert, während der Kindthread noch nicht fertig ist, sondern auf das Signal des Kinds wartet.

    2. Da der alte Code korrekt ist, spielt nur die Effizienz des Codes eine Rolle. Natürlich ist die hier angegebene Lösung außerdem deutlich eleganter.

9. helgrind meldet keine Fehler bei der korrigierten Version. Dies liegt daran, dass nun ein Lock um die Variable `done` gesetzt wird, wenn diese geschrieben wird.

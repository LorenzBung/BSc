# Homework hw10 task1

- [Ziel](#ziel)
- [Aufbau der Nachricht](#aufbau-der-nachricht)
- [Aufgaben](#aufgaben)
    - [Synchronisierung der Queue](#synchronisierung-der-queue)
    - [Einbindung Proof-of-Work Mechanismus mit mehreren Threads](#einbindung-proof-of-work-mechanismus-mit-mehreren-threads)
    - [Kommandos des Servers](#kommandos-des-servers)
    - [Datenverlust vermeiden](#datenverlust-vermeiden)
- [Fehlerbehandlung](#fehlerbehandlung)
- [Dokumentation](#dokumentation)
- [Bibliothek](#bibliothek)
- [Testabdeckung](#testabdeckung)
      Threads](#einbindung-proof-of-work-mechanismus-mit-mehreren-threads)
    - [Kommandos des Servers](#kommandos-des-servers)
    - [Datenverlust vermeiden](#datenverlust-vermeiden)
- [Fehlerbehandlung](#fehlerbehandlung)
- [Dokumentation](#dokumentation)
- [Bibliothek](#bibliothek)
- [Testabdeckung](#testabdeckung)

## Ziel

Ziel dieser Aufgabe ist es, einen Server für den Proof-of-Work Mechanismus zu
erstellen. Dazu soll auf Basis der vorherigen Aufgaben ein Server erstellt
werden, der folgende Funktionalität aufweist:

-   unbegrenzter paralleler Zugriff von Clients, um das Suchen von Hashwerten
    anzufordern und gefundene Hashes abzufragen. Die Clients (z.B. **nc**)
    übertragen dazu jeweils Nachrichten mit den Informationen:
    -   Muster,
    -   \<Zahlenraum\>
    -   \<Port\>
-   Multi-Threaded Implementierung für das Suchen der Hashes
    -   wird kein Zahlenraum angegeben, dann wird ein Hash gesucht, der das
        Muster enthält.
-   Interaktion mit dem Server über Kommandos, um den Server zu parametrisieren
    und zu steuern
-   Beenden und späteres Starten des Servers ohne Datenverluste.

Der Code muss modular erstellt werden. Wie Sie eigene Crates in Ihrem Projekt
benutzen, haben Sie bereits in hw9 Task2 kennen gelernt.

Die einzelnen Aufgabenteile sind grob beschrieben. Wie Sie im Detail die Aufgabe
umsetzen, ist Ihnen überlassen.

## Aufbau der Nachricht

Die Nachricht besteht aus einem String mit "Muster,\<Zahlenraum>,\<Port>":

-   Muster muss immer angegeben werden
-   Zahlenraum ist optional. Enthält die Nachricht diesen Wert, so werden ALLE
    Hashes im Zahlenraum gesucht, die dieses Muster enthalten.
-   Port ist optional. Enthält die Nachricht diesen Eintrag, so wird das
    Ergebnis in einer Ergebnis-Queue abgelegt, die der Client über die
    Portnummer erreicht.

## Aufgaben

### Synchronisierung der Queue

Um einen parallelen Zugriff auf den Server zu ermöglichen, muss die bisherige
Datenstruktur des Servers aus HW9 für einen parallelen Zugriff entsprechend
erweitert werden

### Einbindung Proof-of-Work Mechanismus mit mehreren Threads

Aufträge für Server werden in die Auftrags-Queue geschrieben und die Einträge in
dieser Queue vom Server nacheinander ausgeführt. Zunächst wird das gefundene
Ergebnis auf der Konsole ausgegeben. Implementieren Sie das Verhalten des
Servers entsprechend der Nachricht in der Auftrags-Queue:

-   Wird nur ein "Muster" angegeben, so sucht der Server mit mehreren Threads
    nach EINEM Hashwert, der dieses Muster enthält.
-   Wird "Muster,Zahlenraum" angegeben, so werden alle Hashes mit diesem Muster
    im Zahlenraum gesucht.
-   Wird "Muster,Zahlenraum,Portnummer" angegeben, so werden alle gefundenen
    Hashes im Zahlenraum in einer extra Queue (Ergebnis-Queue) gespeichert und
    können dort vom Client (z.B. **nc**) ausgelesen werden.

### Kommandos des Servers

-   `exit`: kontrolliertes Beenden des Servers.
-   `halt`: sofortiges Anhalten der Aufträge, so dass keine CPU Last mehr vom
    Server ausgeht. Der Status des gerade bearbeiteten Auftrags muss nicht
    gesichert werden.
-   `continue`: Fortfahren mit den Aufträgen, ein evtl. zuvor abgebrochener
    Auftrag wird wiederholt
-   `threads`: Ändern der Anzahl der Threads, die zum Suchen verwendet werden.
    Dies wirkt sich erst beim nächsten zu bearbeitenden Auftrag aus, sofern
    dieses Kommando während der Ausführung eines Auftrags auftritt. 

### Datenverlust vermeiden

Beim obigen `exit` Kommando gehen bisher alle Daten in der Auftrags-Queue, sowie
verschiedener möglicher Ergebnis-Queues verloren. Um dies zu vermeiden werden
vor dem Beenden des Servers mögliche Einträge in den Queues auf der Festplatte
gespeichert und beim Starten des Servers wieder eingelesen. Zum Serialisieren
der Queue-Einträge dürfen Sie externe Crates benutzen.

Wird der Server gestartet überprüft er somit, ob evtl. alte Zustände auf der
Festplatte vorhanden sind und falls nicht, initialisiert er eine leere
Auftrags-Queue und erstellt zunächst auch keine Ergebnis-Queue(s).

## Fehlerbehandlung

Implementieren Sie eine hierarchische Fehlerbehandlung, bei der mögliche Fehler
über generische Fehlertypen der jeweiligen Crates an den Server übergeben und
dort ausgewertet und behandelt werden. Dazu stellt jede Crate einen(!) eigenen
Fehlertypen zu Verfügung.

## Dokumentation

Erstellen Sie ausführliche Dokumentationen Ihrer Library (Sub-Crates)
Funktionen, die exportiert werden, sodass beim Aufruf von **cargo doc** eine
aussagekräftige Doku Ihrer API existiert. Schreiben Sie eine eigene 'DESIGN.md'
Datei in Markdown Syntax, in welcher Sie das Design Ihres Servers beschreiben
(Umfang 1-2 Seiten).

## Bibliothek

Für den letzten Aufgabenteil dürfen Sie externe Crates benutzen. Ansonsten sind
alle bisher in den Laboraufgaben genannten Crates erlaubt. Zum Logging Ihrer
Applikation (kann zur Fehlersuche bei komplexeren Programmen hilfreich sein)
sind ebenfalls externe Crates erlaubt.

## Testabdeckung

Achten Sie auf eine ausreichende Testabdeckung, um innerhalb Ihrer Crates alle
verwendeten Funktionen zu testen.

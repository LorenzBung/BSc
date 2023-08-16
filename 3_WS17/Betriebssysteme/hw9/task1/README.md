# Homework hw9 task1

[TOC]: #

# Table of Contents
- [1.1. Ziel](#11-ziel)
- [1.2. Protokollbeschreibung](#12-protokollbeschreibung)
    - [1.2.1 STAGE](#121-stage)
    - [1.2.3 CONTROL](#123-control)
    - [1.2.2 RETRIEVE](#122-retrieve)
- [2.1 Aufgabe](#21-aufgabe)
    - [2.2.1 Bibliothek](#221-bibliothek)
    - [2.2.2 Testabdeckung](#222-testabdeckung)
    - [2.2.3 Fehlerbehandlung und Rückgabetyp](#223-fehlerbehandlung-und-rückgabetyp)
- [3.1 Hilfen](#31-hilfen)


## 1.1. Ziel

Ziel dieser Aufgabe ist es, einen Parser für ein simples Protokoll zu
implementieren, das in den folgenden Aufgaben verwendet wird.

Das Protokoll beschreibt das Bereitstellen und Abrufen von Daten von
einem Server sowie die Steuerung des Servers.

## 1.2. Protokollbeschreibung

Das Protokoll kennt 3 Kommandos: Bereitstellen, Abrufen und Steuern.
Weitere Kommandos sind zunächst nicht zugelassen. Das Protokoll arbeitet
zeilenweise, d.h. ein newline ('\\n') beendet das Kommando.

Die Kommandos folgen diesem Format:

    <KOMMANDO> [DATEN]\n

Werden keine Daten erwartet, folgt das newline-Zeichen direkt auf das
Kommando. Das Kommando muss immer in Großbuchstaben geschrieben werden.

Der Zeilenumbruch ist _nicht_ optional.

### 1.2.1 STAGE

Das STAGE-Kommando überträgt eine Nachricht (Order) an den Server. Die
Daten dürfen selbst keine newline enthalten. Es ist erlaubt, eine
_leere_ Nachricht zu veröffentlichen.

Beispiele:

    STAGE Hallo, hier ist eine kleine Nachricht!\n
    STAGE \n

### 1.2.3 CONTROL

Das CONTROL-Kommando überträgt ein CONTROL Kommando an den Server. Das
Kommando darf selbst keine newline enthalten. Es ist erlaubt, ein
_leeres_ Kommando zu schicken.

Beispiele:

    CONTROL hello\n
    CONTROL exit\n
    CONTROL Beam me up!\n
    CONTROL \n


### 1.2.2 RETRIEVE

Das RETRIEVE-Kommando entnimmt eine Nachricht. Es erhält keine Daten.

Beispiel:

    RETRIEVE\n

## 2.1 Aufgabe

Implementieren Sie einen Parser für dieses Protokoll. Der Parser sollte so
simpel wie möglich gehalten sein.

Es ist insbesondere erlaubt, die Eingabe als einzelne Zeile zu erwarten.
Die Trennung der Eingabe in einzelne Zeilen ist dem aufrufenden Code
überlassen.

Enthält der Input mehrere newlines, darf der Rest ohne Fehler verworfen
werden.

Folgendes öffentliche Interface ist vorgegeben:

```rust
pub fn parse(message: &str) -> ... {
   //...
}
```

### 2.2.1 Bibliothek

Implementieren Sie den Parser als Bibliothek, die Sie dann in den
weiteren Aufgaben verwenden können.

### 2.2.2 Testabdeckung

Achten Sie auf eine ausreichende Testabdeckung, um möglichst alle
illegalen Fälle abzudecken. Achten Sie vor allem darauf, dass keine
Daten verloren gehen!

### 2.2.3 Fehlerbehandlung und Rückgabetyp

Finden Sie einen geeigneten Rückgabetyp, der sowohl das gefundene
Kommando, als auch eventuelle Fehler kommuniziert. Kombinieren Sie dies
mit einer geeigneten Darstellung des Kommandos.

Allokieren Sie die gefundenen Daten als `String`.

## 3.1 Hilfen

Die Dokumentation von [`str`][str] enthält viele wichtige Hinweise.


[str]: https://doc.rust-lang.org/std/primitive.str.html


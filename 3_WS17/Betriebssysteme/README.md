- [Vorbereitung](#vorbereitung)
    - [User A and User B @ Github:](#user-a-and-user-b-github)
- [Git and GitHub Preparations](#git-and-github-preparations)
    - [User A @ GitHub](#user-a-github)
    - [User A @ Container:](#user-a-container)
    - [User B @ Container:](#user-b-container)
- [Schritte zum Pull-Request](#schritte-zum-pull-request)
- [Travis-CI](#travis-ci)
- [gitignore](#gitignore)

# Laborzugang einrichten

1. Beachten Sie die Ankündigungen im Moodle Forum. Dort werden die Termine zur Anmeldung als 2er Gruppe sowie die Freischaltung Ihrer Workstation bekannt gegeben. Beachten Sie bitte unbedingt die Deadlines dazu!
1. Melden Sie sich beim Laborassistenten (Büro F029) zu den im Forum genannten Terminen zum Labor persönlich an. Sie bekommen für das Labor eine virtuelle 'Workstation' zugeteilt.
1. Testen Sie Ihre Anmeldung im Laborraum. Melden Sie sich dazu mit Ihrer Kennung auf der zugeteilten Workstation an.
1. Lesen Sie die HOWTOS unter [SYSLAB][]


# Aufgaben zum BSYS Labor

In diesem Ordner werden die Aufgaben (Homework,`hw`) veröffentlicht, die bearbeitet werden müssen, um den Schein in BSYS zu bekommen.

Eine Homework besteht aus einer oder mehreren Tasks. Sie finden somit die zu einer Homework gehörenden Aufgaben in den `README.md`, Dateien der zughörigen `hwN/taskN/` Ordner.

> `N` steht als Platzhalter für die entsprechende Homework bzw. Tasknummer.

## Vorbereitung
Die folgenden Befehle demonstrieren den prinzipiellen technischen Ablauf um die Aufgaben vorzubereiten.
Nach der Vorbereitung haben beide Gruppenmitglieder eine lokale Kopie des Git-Repositories.

> Die Variable `N` wird im Folgenden verwendet um die Gruppennummer anzugeben.
> In den Beispielbefehlen wird hierfür 99 verwendet, diese ist aber bei jeder Gruppe unterschiedlich!
>
> UserA und UserB beziehen sich jeweils auf die Gruppenmitglieder.
> Wer UserA und UserB ist, ist nicht wichtig, darf aber während des gesamten Ablaufs nicht verändert werden!

### User A and User B @ Github:
* Visit invitation link and join _grpN_

## Git and GitHub Preparations

### User A @ GitHub
* *htwg-syslab-bsys-ws17/bsys-ws17-grpN* -> fork -> *UserA/bsys-ws17-grpN*
* Add _UserB_ as collaborator to *UserA/bsys-ws17-grpN*

### User A @ Container:

Zunächst muss ein Verzeichnis angelegt werden in das das Git Repository geklont wird. Der Befehl `mkdir` erstellt ein neues, leeres Verzeichnis. Anschließend muss in das neu erstellte Verzeichnis gewechselt werden.
```bash
mkdir -p ~/src/htwg-syslab-bsys-ws17/
cd ~/src/htwg-syslab-bsys-ws17/
```

Mit dem Befehl `git clone` kann ein Repository auf den lokalen Rechner (bzw in den Container) heruntergeladen werden.
```bash
git clone git@github.com:UserA/bsys-ws17-grpN
```

Zuletzt müssen im lokalen Klon noch zusätzliche Remotes hinzugefügt werden. Ein Remote ist ein Repository auf einem anderen Computer, von dem Commits heruntergeladen bzw. auf das Commits hochgeladen werden können. [Weitere Informationen zu remotes](https://git-scm.com/book/en/v2/Git-Basics-Working-with-Remotes).

Bei den zusätzlichen Remotes handelt es sich um folgende:
* Das *template* Remote zeigt auf das Basis Repository der Homeworks. Hier werden im weiteren Verlauf die neuen Homeworks eingepflegt und können von Ihnen abgeholt werden.
* Das *upstream* Remote zeigt auf das Repository von dem Ihr Fork ausgeht. Die lokale Master-Branch wird nach jeder erfolgreichen Abgabe vom *upstream* Repository aktualisiert.

Folgende Befehle fügen die Remotes hinzu:
```bash
git remote add template git@github.com:htwg-syslab-bsys-ws17/bsys-ws17-template.git
git remote add upstream git@github.com:htwg-syslab-bsys-ws17/bsys-ws17-grpN.git
```

### User B @ Container:

Selbes Vorgehen wie User A im vorherigen Abschnitt

# Abgabe der Homeworks

Sobald Sie alle Tasks einer Homework bearbeitet haben, erstellen Sie einen sogenannten  *Pull Request* (PR) von Ihrem Branch, den Sie zum Review abgeben wollen. Der Ablauf wird hier erklärt, gleich zu Anfang in der hw0 geübt und sollte spätestens dabei klar werden.

Die Lösungen müssen in einer bestimmten Ordnerstruktur liegen, die wie folgt
aussieht:

```
...
hw1/
    README.md
    task1/
    task2/
    simu1/
hw2/
    README.md
    task1/
    task2/
    simu1/
...
```

In den `taskN`-Unterordnern muss Ihre Lösung für die entsprechende Programmieraufgabe
liegen. in den `simuN`-Unterordnern liegen Ihre Antworten zu den Simulationsaufgaben. Im jeweiligen **README.md** der Homeworks (`hwN`) finden Sie dazu die nötigen Informationen und Aufgaben.

## Schritte zum Pull-Request
1. Überprüfen Sie den Inhalt Ihres Repository. Achten Sie darauf, dass alle Dateien und die jeweiligen Versionen der Dateien die Sie abgeben wollen nicht nur lokal in Ihrem Home Verzeichnis liegen sondern auch in Ihrem Repository auf github.
1. Überprüfen Sie die Ausgabe Ihrer Programme. Stimmen die Ausgaben mit den geforderten Ausgaben überein?
1. Lassen Sie alle Tests laufen indem Sie im Wurzelverzeichnes des Repositories folgenden Befehl ausführen (nur Linux und Verwandte!):

```
./ci/run-all.sh
```

1. Sobald Sie alle Aufgaben bearbeitet haben, und zur Bewertung die Aufgabe abgeben wollen, erstellen Sie einen Branch für den Pull-Request.
1. Wählen Sie dann auf github diesen Branch aus und erstellen Sie einen Pull-Reqeust auf diesen Branch.
1. Bei Ihrem Pull-Request laufen automatische Tests durch, die Ihr Programm testen. Dies sind nicht alle Tests von `ci/run-all.sh`, daher MÜSSEN Sie unbedingt selbst im lokalen Verzeichnis auf Ihrer Workstation den `ci/run-all.sh` Test ausführen!
1. Falls Ihnen ein Fehler unterlaufen ist, so können Sie auch nach dem Pull-Request noch Änderungen am Code vornehmen. Das sollte jedoch der Ausnahmefall bleiben. Überprüfen Sie daher VOR Ihrem Pull-Request, ob die nötigen Aufgaben bearbeitet wurden und ob die Tests alle durchlaufen.


## Travis-CI

Um das Arbeiten zu erleichtern, ist für alle Lösungsrepositories ein Continuous
Integration Service aufgesetzt worden. Jedes mal, wenn ein Pull Request (PR) erstellt oder aktualisiert wird, laufen eine Reihe von Tests für Ihre Programmieraufgaben durch, die den Codestil prüfen, alle Rust Dateien kompilieren und alle Unit-Tests ausführen.

Jeder PR hat also einen Status: *passed*, *failed* oder *pending*. Ihre PR zum
Einreichen (Deadline) der Aufgaben muss den Status *passed* erreicht
haben, also planen Sie genug Zeit zum Verbessern von kleinen Fehlern ein und erstellen den PR nicht erst kurz vor der Deadline. Zur Verbesserung fügen Sie einfach weitere Commits zu der Branch hinzu von der aus der Pull-Request erstellt wurde. Dieser wird auf GitHub dann automatisch aktualisiert.

>Achtung: Damit das Testen in github nicht zu lange dauert, sind einige sehr lang laufende CI Tests deaktiviert. Bitte aktivieren Sie diese Tests NICHT für travis sondern führen Sie die Tests nur lokal aus. Github Classroom erlaubt nur immer eine laufende Instanz der Travis Tests. Erstellen Sie somit Ihren Pull-Request rechtzeitig, da ansonsten die Deadline aufgrund anderer laufender CI Tests von Ihnen u.U. nicht eingehalten werden kann.

## gitignore

Ebenfalls in Ihrem Repository ist bereits eine `.gitignore` Datei im Root Ordner. Damit werden von git gewisse Dateitypen und Directories in Ihren `hwN/` Verzeichnisse ignoriert, so dass Sie diese nicht Ihrem Repository hinzufügen. Achten Sie dennoch drauf, welche Dateien Sie in Ihr Repository hinzufügen, denn in `.gitignore` sind nicht alle Möglichkeiten abgefangen. Fügen Sie mit **git add** immer nur selektiv Dateien hinzu.

[SYSLAB]: https://htwg-syslab.github.io

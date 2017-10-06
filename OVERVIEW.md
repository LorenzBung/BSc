# Übersicht zum Ablauf der Übungsaufgaben im HTWG Modul BSYS

## Vorbereitung

### Laborzugang einrichten

1. Beachten Sie die Ankündigungen im Moodle Forum. Dort werden die Termine zur Anmeldung als 2er Gruppe sowie die Freischaltung Ihrer Workstation bekannt gegeben. Beachten Sie bitte unbedingt die Deadlines dazu!
1. Melden Sie sich beim Laborassistenten (Büro F029) zu den im Forum genannten Terminen zum Labor persönlich an. Sie bekommen für das Labor eine virtuelle 'Workstation' zugeteilt.
1. Testen Sie Ihre Anmeldung im Laborraum. Melden Sie sich dazu mit Ihrer Kennung auf der zugeteilten Workstation an.

### Fork des htwg-syslab-bsys GitHub Repository

Um auf Ihr htwg-syslab-bsys beheimatete Repository für das BSYS Labor zu erhalten, müssen Sie einige Punkte vorab durchführen:

1. Legen Sie sich - soweit noch nicht vorhanden - einen GitHub Account an.
1. Über den Einladungslink im Moodle-Forum erstellen Sie Ihre Labor-Gruppe unter htwg-syslab-bsys auf github.

>Es macht keinen Sinn eine 1er Gruppe anzumelden, wenn Sie noch keinen Partner gefunden haben, da Sie immer beide RZ-IDs zum Anmelden benötigen. Bitte suchen Sie sich zeitnah einen Partner, mit dem Sie das Labor bestreiten wollen.

1. ***WICHTIG***: Sie legen sich nun eine Gruppe für Ihr 2er Team an.
    - Name der Gruppe: Benutzen die als Namen den String Ihre Gruppe aus Moodle, also z.B. `grp0`. Achten Sie genau auf die Schreibweise (Gross/Klein-Schreibung)
    - Der 2. Teilnehmer meldet sich zu dieser neuen Gruppe an.
1. Sie landen auf der "Ready to go" Seite. Dort nehmen Sie bitte zuerst die Einladung an, indem Sie auf den *@htwg-syslab-bsys* Link klicken. Nach der Annahme der Einladung können Sie auf Ihr 'Assignment'-Repository per Browser zugreifen
1. ***WICHTIG***: Einer(!) in Ihrem 2er Team forkt nun das Projekt, so dass der Fork Ihres Projektes in IHREM Github Account liegt. Mit diesem geforkten Repository werden Sie zunächst arbeiten. Damit der Partner in Ihrem Team auch mit diesem Repository arbeiten kann, müssen Sie ihn in den Einstellungen zu Ihrem Projekt auf github dazu autorisieren.
    - Führen Sie keine Änderungen auf dem Repository im htwg-syslab-bsys Bereich durch, sondern NUR im geforkten Rep in Ihrem privaten github Account.

### ssh Key erstellen

Um Ihr git Repository von github bequem auschecken zu können, muss zuvor ein SSH-Key generiert werden. Dieser ssh Key muss PRO Rechner erstellt werden, von welchem Sie auf das Repository zugreifen wollen. Wenn Sie z.B. von einem Notebook und Ihrer zugewiesenen Workstation arbeiten wollen, so müssen Sie für beide Rechner jeweils den ssh Key erstellen und auf github in Ihren Settings den 'public' Teil des jeweiligen Keys hinzufügen.

Wenn Sie noch keinen ssh Key erstellt haben so ist in [auf Github][git-ssh-key-gen] beschrieben, wie das aus einem Linux System funktioniert. Den Teil mit dem ssh-agent können Sie überspringen. Der erzeugte Public Key muss dann noch zu [github kopiert][git-ssh-key-copy] werden. Danach kann über eine sichere ssh Verbindung der Transfer zwischen Github und dem Rechner stattfinden. Da dieser Transfer für jede Rechnerverbindung benötigt wird, müssen Sie auf jedem verwendeten Rechner einen Schlüssel erstellen.


Nachdem Sie erfolgreich eine lokale Arbeitsversion Ihres geforkten Gruppenrepositories (Developer-Repo) erstellt haben, können Sie den Inhalt editieren.

In diesem Ihrem Repository werden Sie Ihre Lösungen zu den
Homeworks aus dem BSYS Labor sammeln. Dazu wird im weiteren exemplarisch von obiger Gruppe: 'Susi Sorglos' und 'Willi Wacker' ausgegangen.

Sobald Sie alle Aufgaben eines Aufgabenblattes (homework) bearbeitet haben, erstellen Sie einen sogenannten  *Pull Request* (PR) von Ihrem Branch, den Sie zum Review abgeben wollen.

Die Lösungen müssen in einer bestimmten Ordnerstruktur liegen, die wie folgt
aussieht:

```text
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

## Neue Aufgabenstellung

Neue Übungsaufgaben werden in einem [separaten Homework Repository][1] veröffentlicht. Es kann vorkommen, dass mit der Aufgabenstellung schon ein paar Quelldateien vorgegeben werden; diese müssen Sie dann von dem genannten Repository in Ihren Fork kopieren. Beachten Sie bitte, dass die Ordnerstruktur genau den Vorgaben entsprechen muss.

Auch wird für jede Homework eine Datei unter `files/` bereit gestellt. Diese wird bei den Tests benutzt, um zu überprüfen, ob Sie alle geforderten Dateien erstellt haben und hilft Ihnen bei der Überprüfung der Vollständigkeit Ihrer Lösung.

Dies geht am besten, indem Sie beide Repositories (Ihren Fork des
Gruppenrepositories *und* [das Aufgaben Repository][1]) in den selben Ordner klonen (sodass sie nebeneinander liegen). Das ganze sieht dann in etwa im Ordner von Susi Sorglos (aus `grp0`) so aus:

```text
$ git clone git@github.com:SusiSorglos/bsys-ws17-grp0.git
$ git clone https://github.com/htwg-syslab-bsys/bsys-ws17-homework.git
$ ls
bsys-ws17-grp0/         bsys-ws17-homework/
```

Wenn eine neue Aufgabe veröffentlicht wurde, muss zunächst
`bsys-ws17-homework` aktualisiert werden:

```text
$ cd bsys-ws17-homework
$ git pull
$ cd ..
```

Nun kann die Aufgabenstellung (mit Quelldateien) der `N`ten Homework
in Ihren Fork kopiert werden:

```text
$ cp -r bsys-ws17-homework/hwN bsys-ws17-grp0/
```

## Abgabe allgemein

### Travis-CI

Um das Arbeiten zu erleichtern, ist für alle Lösungsrepositories ein Continuous
Integration Service aufgesetzt worden. Jedes mal, wenn ein Pull Request (PR) geöffnet oder aktualisiert wird, laufen eine Reihe von Tests für Ihre Programmieraufgaben durch, die den Codestil
prüfen, alle Rust Dateien kompilieren und alle Unit-Tests ausführen.

Jeder PR hat also einen Status: *passed*, *failed* oder *pending*. Ihre PR zum
Einreichen (Deadline) der Aufgaben muss den Status *passed* erreicht
haben, also planen Sie genug Zeit zum Verbessern von kleinen Fehlern ein und öffnen den PR nicht erst kurz vor Schluss.

### Schritte zum Pull-Request
1. Überprüfen Sie den Inhalt Ihres Repository. Achten Sie darauf, dass alle Dateien und die jeweiligen Versionen der Dateien die Sie abgeben wollen nicht nur lokal in Ihrem Home Verzeichnis liegen sondern auch in Ihrem Repository auf github.
1. Überprüfen Sie die Ausgabe Ihrer Programme. Stimmen die Ausgaben mit den geforderten Ausgaben überein?
1. Lassen Sie alle Tests laufen indem Sie im Wurzelverzeichnes
des Repositories folgenden Befehl ausführen (nur Linux und Verwandte!):

```text
./ci/run-all.sh
```

1. Sobald Sie alle Aufgaben bearbeitet haben, und zur Bewertung die Aufgabe abgeben wollen, erstellen Sie einen Branch für den Pull-Request.
1. Wählen Sie dann auf github diesen Branch aus und öffnen Sie einen Pull-Request auf diesen Branch. Vergessen Sie nicht einen Tutor (soweit möglich) als Reviewer einzutragen.
1. Bei Ihrem Pull-Request laufen automatische Tests durch, die Ihr Programm testen. Dies sind nicht alle Tests von `ci/run-all.sh`, daher MÜSSEN Sie unbedingt selbst im lokalen Verzeichnis auf Ihrer Workstation den `ci/run-all.sh` Test ausführen!
1. Falls Ihnen ein Fehler unterlaufen ist, so können Sie auch nach dem Pull-Request noch Änderungen am Code vornehmen. Das sollte jedoch der Ausnahmefall bleiben. Überprüfen Sie daher VOR Ihrem Pull-Request, ob die nötigen Aufgaben bearbeitet wurden und ob die Tests alle durchlaufen.
1. Wenn Sie die für die Homework relevante `files\` Datei in Ihr Repository kopiert haben, so wird beim Durchlaufen aller Tests auch die Existenz der geforderten Dateien in Ihrer Lösung überprüft

>Achtung: Damit das Testen in github nicht zu lange dauert, sind einige sehr lang laufende CI Tests deaktiviert. Bitte aktivieren Sie diese Tests NICHT für travis sondern führen Sie die Tests nur lokal aus. Github Classroom erlaubt nur immer eine laufende Instanz der Travis Tests. Erstellen Sie somit Ihren Pull-Request rechtzeitig, da ansonsten die Deadline aufgrund anderer laufender CI Tests von Ihnen u.U. nicht eingehalten werden kann.

## gitignore

Ebenfalls in Ihrem Repository ist bereits eine `.gitignore` Datei im Root Ordner. Damit werden von git gewisse Dateitypen und Directories in Ihren `hwN/` Verzeichnisse ignoriert, so dass Sie diese nicht Ihrem Repository hinzufügen. Achten Sie dennoch drauf, welche Dateien Sie in Ihr Repository hinzufügen, denn in `.gitignore` sind nicht alle Möglichkeiten abgefangen. Fügen Sie mit **git add** immer nur selektiv Dateien hinzu.

[1]: https://github.com/htwg-syslab-bsys/bsys-ws17-homework.git
[git-ssh-key-gen]: https://help.github.com/articles/generating-a-new-ssh-key-and-adding-it-to-the-ssh-agent/#generating-a-new-ssh-key
[git-ssh-key-copy]: https://help.github.com/articles/adding-a-new-ssh-key-to-your-github-account/

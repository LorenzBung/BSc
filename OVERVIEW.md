# Übersicht zum Ablauf der Übungsaufgaben im HTWG Modul BSYS

## Vorbereitung



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


[1]: https://github.com/htwg-syslab-bsys/bsys-ws17-homework.git
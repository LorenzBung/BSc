# Aufgaben zum BSYS Labor

In diesem Ordner werden die Aufgaben (Homework,`hw`) veröffentlicht, die bearbeitet werden müssen, um den Schein in BSYS zu bekommen. In der Datei `OVERVIEW.md` in diesem Ordner sind weitere Informationen enthalten.

Eine Homework besteht aus einer oder mehreren Tasks. Sie finden somit die zu einer Homework gehörenden Aufgaben in den `README.md`, Dateien der zughörigen `hwN/taskN/` Ordner. `N` steht als Platzhalter für die entsprechende Homework bzw. Tasknummer.

## Vorbereitung
Die folgenden Befehle demonstrieren den prinzipiellen technischen Ablauf um die Aufgaben vorzubereiten.
Nach der Vorbereitung haben beide Gruppenmitglieder eine lokale Kopie des Git-Repositories.

> Die Variable `N` wird im Folgenden verwendet um die Gruppennummer anzugeben.
> In den Beispielbefehlen wird hierfür 99 verwendet, diese ist aber bei jeder Gruppe unterschiedlich!
>
> UserA und UserB beziehen sich jeweils auf die Gruppenmitglieder.
> Wer UserA und UserB ist, ist nicht wichtig, darf aber während des gesamten Ablaufs nicht verändert werden!

### User A and User B @ Github:
* Visit invitation link and join _grp$N_

## Git and GitHub Preparations

### User A @ GitHub
* *htwg-syslab-bsys-ws17/bsys-ws17-grp$N* -> fork -> *UserA/bsys-ws17-grp$N*
* Add _UserB_ as collaborator to *UserA/bsys-ws17-grp$N*

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

Zuletzt müssen die Repositories *upstream* und *template* noch als zusätzliches Remote registriert werden. Ein Remote ist ein Repository auf einem anderen Computer, von dem Commits heruntergeladen bzw. auf das Commits hochgeladen werden können. [Weitere Informationen zu remotes](https://git-scm.com/book/en/v2/Git-Basics-Working-with-Remotes)
```bash
git remote add template git@github.com:htwg-syslab-bsys-ws17/bsys-ws17-template.git
git remote add upstream git@github.com:htwg-syslab-bsys-ws17/bsys-ws17-grpN.git
```

### User B @ Container:

Selbes Vorgehen wie User A im vorherigen Abschnitt

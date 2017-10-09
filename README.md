# Aufgaben zum BSYS Labor

In diesem Ordner werden die Aufgaben (Homework,`hw`) veröffentlicht, die bearbeitet werden müssen, um den Schein in BSYS zu bekommen. In der Datei `OVERVIEW.md` in diesem Ordner sind weitere Informationen enthalten.

Eine Homework besteht aus einer oder mehreren Tasks. Sie finden somit die zu einer Homework gehörenden Aufgaben in den `README.md`, Dateien der zughörigen `hwN/taskN/` Ordner. `N` steht als Platzhalter für die entsprechende Homework bzw. Tasknummer.

## Vorbereitung
Die folgenden Befehle demonstrieren den prinzipiellen technischen Ablauf um die Aufgaben vorzubereiten.
Nach der Vorbereitung haben beide Gruppenmitglieder eine lokale Kopie des Git-Repositories.

### User A and User B @ Github:
* Visit invitation link and join grp$N

## Git and GitHub Preparations

### User A @ Github
* *htwg-syslab-bsys-ws17/bsys-ws17-grp$N* -> fork -> *UserA/bsys-ws17-grp$N*
* Add UserB as collaborator to *UserA/bsys-ws17-grp$N*

### User A @ Container:
```bash
N=99
mkdir -p ~/src/htwg-syslab-bsys-ws17/
cd ~/src/htwg-syslab-bsys-ws17/
git clone git@github.com:UserA/bsys-ws17-grp${N} 
git remote add template git@github.com:htwg-syslab-bsys-ws17/bsys-ws17-template.git
git remote add upstream git@github.com:htwg-syslab-bsys-ws17/bsys-ws17-grp${N}.git
```

### User B @ Container:
```bash
N=99
mkdir -p ~/src/htwg-syslab-bsys-ws17/
cd ~/src/htwg-syslab-bsys-ws17/
git clone git@github.com:UserA/bsys-ws17-grp${N} 
git remote add template git@github.com:htwg-syslab-bsys-ws17/bsys-ws17-template.git
git remote add upstream git@github.com:htwg-syslab-bsys-ws17/bsys-ws17-grp${N}.git
```

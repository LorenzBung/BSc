# HW0 - Git Übung
In dieser Übung wird der Umgang mit `git` auf der Kommandozeile gelernt.

## Vorbereitung
Updates des Templates herunterladen und neue Feature-Branches für jeden Benutzer erstellen.

### User A @ Container:
```bash
N=99
cd ~/src/htwg-syslab-bsys-ws17/bsys-ws17-grp${N}
git fetch --all
git checkout hw0
git push origin hw0
git checkout -b hw0-UserA
git push origin hw0-UserA
```

### User B @ Container:
```bash
N=99
cd ~/src/htwg-syslab-bsys-ws17/bsys-ws17-grp${N}
git fetch --all
git checkout hw0
git checkout -b hw0-UserB
git push origin hw0-UserB
```

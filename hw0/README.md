# HW0 - Git Übung
In dieser Übung wird der Umgang mit `git` auf der Kommandozeile gelernt.

## Vorbereitung
Updates des Templates herunterladen und neue Feature-Branches für jeden Benutzer erstellen.

### User A @ Container:
```bash
cd ~/src/htwg-syslab-bsys-ws17/bsys-ws17-grpN
git fetch --all
git checkout hw0
git push origin hw0
git checkout -b hw0-UserA
git push origin hw0-UserA
```

### User B @ Container:
```bash
cd ~/src/htwg-syslab-bsys-ws17/bsys-ws17-grpN
```

Hole die neusten Informationen aller Remotes
```bash
git fetch --all
```

Verwende *origin/hw0* als Basis für die neue lokale *hw0* Branch
```bash
git checkout origin/hw0
git checkout -b hw0
```

Zweige von *hw0* ab nach *hw0-UserB* und push nach auf den Fork von UserA (origin)
```bash
git checkout -b hw0-UserB
git push origin hw0-UserB
```

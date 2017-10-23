### Antworten zu Simulation 1
#### Aufgabe 1
Die CPU sollte 100% der Zeit in Nutzung sein, da wir beim Aufruf
> ./process-run.py -l 5:100,5:100.
angeben, dass 100% der ausgeführten Befehle CPU-Befehle sein sollen.

#### Aufgabe 2
Zuerst werden vier CPU-Befehle in Prozess 0 ausgeführt. Danach ist Prozess 0 beendet, Prozess 1 bereitet sich auf I/O vor und braucht dafür eine CPU-Instruktion. Prozess 1 muss dann auf die Abwicklung des I/Os warten und tut deswegen vier Instruktionen lang nichts. Anschließend läuft eine leere Instruktion, in der weder I/O noch CPU arbeiten, weil die I/O-Instruktion beendet wurde. Wir gehen daher davon aus, dass die Ausführung der beiden Prozesse 10 Instruktionen braucht.

#### Aufgabe 3
Die Änderung der Reihenfolge der auszuführenden Prozesse hat zur Folge, dass die CPU auch rechnen kann, solange sie auf I/O wartet. Daher wird nur eine Instruktion zum Einleiten des I/Os benötigt, dann laufen vier Instruktionen, in denen sowohl CPU (für Prozess 1) und I/O (für Prozess 0) arbeiten, und abschließend vergeht noch eine leere Instruktion aufgrund des I/Os. Daher braucht die Ausführung in dieser Reihenfolge nur 6 Instruktionen.

#### Aufgabe 4
Mit dieser Flag wird die CPU nicht parallel zum I/O arbeiten können. Deswegen wird die Ausführung wie bei Aufgabe 2 ablaufen, jedoch werden die Prozesse vertauscht abgearbeitet, also ein Takt Einleitung des I/Os, vier Takte I/O und dann 4 Takte CPU.

#### Aufgabe 5
Der Ablauf wird genau der gleiche wie bei Aufgabe 3 sein, da per Standard beim Warten auf I/O zum anderen Prozess gewechselt wird.

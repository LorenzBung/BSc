
# Simulation 2 - Antworten

Diese Simulation wurde auf einem Laptop mit 8GB RAM, einer 8GB swapfile und 4 Prozessorkernen ausgeführt.


1. Wenn man *mem* mit auch nur mit 1 MB Speicher aufruft, dann geht die CPU Auslastung eines Kerns auf 100%.

   Im Idle Zustand war die user-time bei ~1%. Bei nur einem laufenden mem-Programm betrug die user-time ~25%. Bei 3 laufenden Instanzen waren es ~80% und bei 4 ~99% user-time.

   Das klingt absolut logisch, da der Prozessor des Systems 4 Kerne hat. Denn wenn auf jedem Kern ein mem-Programm läuft, dann hat das Bertriessystem keinen eigenen Kern und die prozentuale sys-time wird winzig.

2. Direkt nach dem Ausführen von ./mem 1024 hat sich der freie Speicher um ~1.050.000 Byte verringert und die Spalte swpd hat sich nicht verändert, sondern blieb auf 0. Sobald wir das mem-Programm (mit Ctrl-C) beendet haben, ist der freie Speicher (free) fast wieder auf den Urprungswert gesprungen. Es fehlten ~250 Byte. Der freie Speicher hat sich danach also verkleinert.

   Auf der Workstation wurde bereits bei 1024 Byte *geswapped* und der freie Speicher war nach dem Beenden des Programms größer.

3. Bei ./mem 4000 gab es auf dem Rechner (mit 8GB RAM) absolut kein swap-in / swap-out. Es sei denn, der freie Speicher von den gesamten 8GB war kleiner als die ~4GB.

   Unter normaler Last war erst bei ./mem 7000 ein deutlicher swap-in und -out sichtbar. Bei jedem der Werte war jedoch der erste *loop* immer langsamer. Durschnittlich war er oft ~50% langsamer.

   Im ersten Loop

4. Die CPU Auslastung durch das mem-Programm ist immens. Wenn man es auf dem Laptop ausführt, drehen direkt die Lüfter hoch. Doch wie zu erwarten und im Quellcode zu sehen, ist das C-Programm nicht *multi-threaded*. In einem beliebigen System Monitor (z.B. htop) sieht man, dass nur ein Kern ausgelastet ist.

   Die BlockIOs

5. Bei 4000 Byte braucht der erste loop 1070 ms und alle restlichen 745ms

   Da bereits beim Aufruf von ./mem 8192 ein Segmentation Fault auftritt, war es nicht möglich über die Grenze des physikalischen 8GB Speichers (8192 kByte) zu gehen. Der Rest dieser Aufgabe ist also nicht wirklich beantwortbar.

   Bei ./mem 8191 muss allerdings auch schon massiv *geswapped* werden. Der erste Loop braucht 6 Sekunden und die folgenden mehr als 60 Sekunden. Die Bandbreite in Loop 0 war noch bei 1317 MB/s in Loop 1 schon 128 MB/s und in den folgenden Loops ~110 MB/s. Man merkt dass nun die Festplatte der *bottleneck* ist, da das Programm kaum noch CPU Leistung benötigt.

6. Bei 14584 schlägt die Speicheralloziierung fehl

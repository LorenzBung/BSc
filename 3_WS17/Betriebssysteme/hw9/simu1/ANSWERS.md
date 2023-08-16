# hw9 - Simulation 1 - Antworten

1. Der Output des Programms verändert sich: Manchmal sind die beiden Operationen des zweiten Threads zwischen den Operationen des ersten.

2. Je höher die Zahl der Loops, desto höher ist auch die Wahrscheinlichkeit, dass die Operaionen des zweiten Threads zwischen denen des ersten ausgeführt werden (bzw. andersherum). Dies ist jedoch nicht immer der Fall; besonders bei niedriger Anzahl von Loops ist es selten.

3. Mit einer höher werdenden Zahl von Threads steigt auch die Zahl der möglichen Deadlocks. Das Verhindern von diesen ist nur mithilfe der Flag `-n 1` möglich, wodurch nur ein Thread läuft.

4. Hier wird ein Deadlock vermieden, da eine Fallunterscheidung gemacht wird, welche die Reihenfolge der Locks angibt. Im Fall, dass `src` und `dst` dieselben sind, muss nur ein Lock gesetzt werden. Deswegen gibt es hier einen Spezialfall.

5. Mit den Flags `-t -n 2 -l 100000 -d` braucht der Code etwa `0.05 s` für die Ausführung. Eine Verdopplung der Zahl der Loops hat auch eine Verdopplung der Ausführungszeit zur Folge, sodass wir eine Zeit von ca. `0.9 s` bekommen. Bei einer Verdoppelung der Threadzahl nimmt die Ausführungsdauer weiter zu, sodass wir mit den Flags `-t -n 4 -l 100000 -d` eine Zeit von `~ 0.10 s` und mit den Flags `-t -n 4 -l 200000 -d` von `~ 0.22 s` bekommen.

6. Mit der Parallelisierungsflag `-p` läuft das Programm deutlich schneller: Unsere obigen Tests benötigen nun noch `0.04`, `0.06`, `0.06` bzw. `0.09 s`. Zu erwarten ist ein Geschwindigkeitszuwachs um den Faktor der Threads, jedoch muss berücksichtigt werden, dass diese am Ende auch wieder zurückgeführt werden müssen, was die Ausführungszeit deutlich verlangsamt.

7. Der erste Aufruf von `pthread_mutex_trylock()` wird benötigt, da in `dst->values` sonst eine race condition auftreten könnte.
    Der Code läuft deutlich langsamer als `vector-global-order.c`. Die schon oben getesteten Flags liefern hier (ohne Parallelisierung) die Laufzeiten `0.19`, `0.28`, `1.09` und `2.57 s`. Die Retries werden deutlich mehr, wenn die Zahl der Threads ansteigt.

8. In unseren Tests braucht `vector-avoid-hold-and-wait.c` für die Ausführung `0.09`, `0.13`, `0.15` und `0.17 s` bzw. mit der Flag `-p` braucht es `0.07`, `0.10`, `0.13` und `0.21 s`.

9. Aufgrund der Verwendung von Assemblercode werden hier keine Locks benötigt. Diese Version tut also genau dasselbe wie die vorherigen Versionen.

10. Die Version `vector-nolock.c` ist sowohl in der nichtparallelen als auch der parallel laufenden (`-p`) Variante eher langsam im Vergleich mit den anderen Versionen.

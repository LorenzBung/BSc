# hw5 - Simulation 1 - Antworten

1. In diesem Fall sind die durchschnittliche Response- und Turnaroundtime sowohl bei FIFO als auch bei SJF gleich, da die drei Jobs gleich lange dauern. Deshalb werden sie bei SJF in der selben Reihenfolge wie bei FIFO ausgeführt.

2. Nun ist SJF sowohl in Betrachtung der durchschnittlichen Response- als auch der Turnaroundtime FIFO deutlich voraus. Das liegt daran, dass bei SJF der kürzeste Job zuerst bearbeitet wird.

3. Die Responsetime wird im Vergleich zu SJF und FIFO logischerweise deutlich kürzer, da bereits nach 1 Sekunde ein Kontextwechsel stattfindet. Dafür erhöht sich bei gleicher Jobdauer jedoch die (durchschnittliche) Turnaroundtime immens, da die einzelnen Jobs immer wieder unterbrochen werden und somit erst (fast) gleichzeitig fertig werden.
Bei unterschiedlicher Jobdauer ist die dadurch entstehende Verzögerung in der Turnaroundtime nicht ganz so groß (da die kürzeren Jobs auch schneller beendet werden), jedoch immer noch *deutlich* länger als bei SJF oder FIFO.

4. SJF liefert dieselben Turnaroundtimes wie FIFO, sobald die Jobdauer der einzelnen Jobs **gleich lange** ist. Dann spielt es nämlich keine Rolle, welcher Job zuerst bearbeitet wird (da sowieso alle gleich lange brauchen).

5. SJF liefert dieselbe Responsetime wie RR, wenn alle Jobs dieselbe Länge haben und die Quantumlänge bei RR gleich oder höher ist als die Länge der Jobs.

6. Die Responsetime erhöht sich mit der Joblänge. Das liegt daran, dass sich die durchschnittliche Responsetime mit `R = (l_1 + l_2 + ... + l_n) / n` berechnen lässt (`l_i` die jeweiligen Joblängen, `n` die Zahl der Jobs). Die Responsetime ist damit abhängig von der Dauer jedes Jobs. Demonstrieren lässt sich dies mit `./scheduler.py -p SJF -s 1 -l 10,10,10` und z.B. `./scheduler.py -p SJF -s 1 -l 100,100,100`, oder auch `./scheduler.py -p SJF -s 1 -l 10,20,30`.

7. Die Responsetime erhöht sich mit der Quantumlänge. Dies ist logisch, da ein Prozess mehr Rechenzeit hat und der zweite Prozess somit länger warten muss. Nimmt man eine sehr lange Quantumlänge, z.B. mit `-q 10000`, so laufen alle Jobs nacheinander ab (wie bei FIFO). Daraus folgt, dass die Responsetime bei RR durch die Quantumszeit und die Länge der Jobs nach oben limitiert ist. Deshalb gilt: `R = min((l_1 + l_2 + ... + l_n)/n, q)` (`l_i` die jeweiligen Joblängen, `n` die Zahl der Jobs, `q` die Quantumlänge).

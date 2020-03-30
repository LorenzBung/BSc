# hw6 - Simulation 1 - Antworten

1.
    * Da Job 1 so viel mehr Tickets hat, ist es bei der Ausführung durch den Lottery Scheduler viel wahrscheinlicher, dass dieser Job ausgeführt wird.
    * Job 0 wird (zumindest mit `-s 0`) nie ausgeführt, bevor Job 1 fertig ist. Theoretisch könnte es vorkommen, ist jedoch sehr unwahrscheinlich (1/101-tel pro Tick).
    * Eine solch unfaire Ticketverteilung führt zu einem Verhalten des Schedulers, das nicht der Idee der "Lottery" entspricht. Es führt dazu, das ein Job zunächst vollständig ausgeführt wird und anschließend der andere, bzw. zu einer Priorisierung des Jobs mit vielen Tickets. Das Verhalten wird dadurch ähnlich zu Schedulern wie z.B. FIFO.

2. Der Scheduler ist sehr fair bei einer Joblänge von 100. Mit `-s 1` ist die Fairness `196/200 = 0,98`, mit `-s 2` liegt sie bei `190/200 = 0,95`, mit `-s 3` bei `196/200 = 0,98`, mit `-s 4` bei `199/200 = 0,995` und mit `-s 5` bei `181/200 = 0,905`. Im Durchschnitt (unter Verwendung dieser 5 Seeds) ist das eine Fairness von `0,962`, was sehr nahe an der angestrebten Fairness von `1,0` liegt.

3. Mit `-q 10` und den Seeds 1-5 ergeben sich folgende Fairness-Werte: `-s 1` und `-s 5`: `160/200 = 0,8`, `-s 2`, `-s 3` und `-s 4`: `190/200 = 0,95`. Die durchschnittliche Fairness liegt damit bei `0,92`, was deutlich schlechter als mit kürzerer Quantumzeit ist. Je größer die Quantumzeit dabei wird, desto schlechter ist auch die Fairness. Beispielsweise ergibt sich mit den selben Seeds, jedoch mit `-q 100` nur noch eine durchschnittliche Fairness von `0,5`.

4. Interessant zu untersuchen wäre der Zusammenhang zwischen den Joblängen und der Quantumzeit, beispielsweise ob sich eine Abhängigkeit feststellen lässt.

   Mit einem Stride Scheduler wäre die Fairness immer `1,0`, da beide Jobs die selbe Länge haben und somit immer abwechselnd ausgeführt würden. Der Graph wäre somit konstant auf `1.0` bzw. 100% Fairness.

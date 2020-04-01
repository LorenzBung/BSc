# Proof of Work 2.0

###  Performance-Gewinn *Single-Threaded vs Multi-Threaded*

In der folgenden Tabelle sieht man sehr deutlich den Geschwindigkeitsgewinn durch das Multithreading. Je länger die Difficulty ist, desto besser sieht man, dass durch Verdopplung der Threads die reale Zeit ungefähr halbiert wird.

| Difficulty | Single Threaded*           | 2 Threads*                | 4 Threads*               |
| ---------- | -------------------------- | ------------------------- | ------------------------ |
| "1234"     | **0.314s** \| 0.288s       | **0.284s** \| 0.280s      | **0.254s** \| 0.252s     |
| "12345"    | **0.693s** \| 0.644s       | **0.462s** \| 0.660s      | **0.336s** \| 0.552s     |
| "123456"   | **12.936s** \| 12.904s     | **6.899s** \| 13.464s     | **5.755s** \| 15.720s    |
| "1234567"  | **1m31.361s** \| 1m30.740s | **45.961s** \| 1m31.192s  | **21.554s** \| 1m2.240s  |
| "12345678" | **2m2.810s** \| 2m2.684s   | **1m1.332s** \| 1m46.931s | **34.746s** \| 1m38.376s |

*time format: **real** | user



### Antworten

#### Arten der Synchronisierung und Kommunikation

Neben den MPSC-channels für Rückgabe der Solution und der Zeitstatistik, verwenden wir für mehrere Variablen den Typ [Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html). Jeder Thread bekommt über einen *clone()*-Vorgang eine Referenz auf die selbe, gemeinsame Variable im Heap. Das sync-Flag haben wir über einen [AtomicBool](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicBool.html), welcher von einem *Arc* gehalten wird, gelöst. Über store(..) und load(..) lässt sich dann auf diesen 'boolean' zugreifen.

#### Was ist der Unterschied zwischen einer logischen und einer physikalischen CPU?

Logische CPU-Kerne sind nur virtuell vorhanden und werden auf die physikalische CPU 'umgeleitet'. Sie werden vom Betriebssystem, insbesondere vom Scheduler, gesehen und verwendet. Deshalb spielt es beim Multithreading bzw. auch beim Scheduling ein Rolle wie das Verhältnis von logischen und physikalischen CPUs ist.

#### Wie ist die Ausgabe von time zu interpretieren, wenn mehrere Threads laufen?

Im Gegensatz zu einem Thread ist bei mehreren Threads die user-time immer deutlich höher als die real-time (siehe Tabelle).
Das heißt, dass die Threads nicht nebenläufig, sondern echt parallel laufen. Wenn man mit `-p` die Prozentangaben einschaltet, dann sieht man auch, dass die user-time deutlich über 100% liegt.

#### Welche unterschiedlichen Ergebnisse erhalten Sie bei der Option timings? Wie stehen diese im Zusammenhang mit den Ergebnissen von time?

Die Option `timings` variiert unterschiedlich stark von den Ergebnissen von **time**. Das unix-Kommando time bezieht die Ausführungszeit des gesamten Codes mit ein. wenn man das Programm über `time cargo run --release...` aufruft, statt das erstellte binary, dann wird sogar die Compilezeit miteinberechnet.
Also auch Aufrufe von *sys-info* oder zum Beispiel das Parsen der Argumente, welches *clap* durchführt.

#### Welche quantitativen Auswirkungen hat die Synchronisierung (Warum)? Welche Auswirkungen haben Ihre zusätzlichen Parameter?

Mit dem Synchronisierungsflag muss nur ein Producer eine Lösung finden, danach brechen die anderen Threads kontrolliert ihre Suche ab. Mit dem Wert hinter dem Parameter `--special` kann man die Häufigkeit dieser Abfrage (ob ein anderer Thread eine Lösung gefunden hat) steuern.

#### Wie variieren -s und -w die Messungen?

Bei der Suche nach Hash mit base = 42 und difficulty = "123456" und 10 threads:

Mit `-s` und `-w`: 

Der Thread, der die erste gültige *number* findet, setzt den gennanten AtomicBool auf true und beendet somit **indirekt** die anderen Threads.
  ```text
  Sum Loops in Producers:       19019908
  Sum Duration in Producers:    55s / 55585ms / 55585423us
  ```


Ohne `-s` und `-w`:

Wartet der Consumer bis jeder Thread eine *Solution* gefunden hat. Dies ist sehr deutlich an Loopanzahl und Zeit erkennbar.
  ```text
  Sum Loops in Producers:       60340756
  Sum Duration in Producers:    114s / 114074ms / 114074341us
  ```

#### Welche sonstigen Arten der Synchronisierung bieten sich für die Problemstellung "Lösung gefunden (-s)" an? Was sind deren Vor- und Nachteile gegenüber Ihrer gewählten Lösung?

Man könnte anstatt dem Arc und dem AtomicBool im Consumer aktiv alle Threads töten, sobald eine Lösung gefunden wurde. Das ist zwar schneller, aber es ist nicht mehr möglich auf die Threadlaufzeiten und Threadschleifendurchläufe zuzugreifen.

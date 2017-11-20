# Antworten

1. 
   **Figure 8.1: A Single Long-Running Job**

   `./mlfq.py -l 0,200,0`

   Dieser Aufruf erzeugt einen Job der bei Zeit t=0 startet, 200ms läuft und keine IO-Time hat.

   **Figure 8.2: Along Came a Short Job**

   `./mlfq.py -l 0,200,0:120,20,0`

   Job 0 startet bei t=0 mit 200ms Runtime und Job 1 springt bei t=120 für 20ms ein (10 ms Priority 2 + 10ms Prority 1). Wenn man nun das "alte" t=20 als neuen Zeitpunkt 0 sieht, dann hat man das Beispiel aus der Vorlesung.
   
   **Figure 8.3: What About I/O?**
   
   `./mlfq.py --jlist 0,180,0:100,20,0 -Q 10,10,10`
   

2. Indem man den Scheduler mit `-m 1` auf nur eine Queue einstellt. Dann werden neue Jobs nicht in eine höhere Queue gestuft und die Priorität auch nicht erhöht.

3. Mit dem Aufruf `./mlfq.py -S -l 0,200,0:0,800,99 -c -q 100 -i 1` wird der erste Job bei t=100 von dem 2.ten zu 99% unterbrochen, welcher Regel 4b ausnutzt und auf Priorität 2 bleibt. Man sieht dass **JOB 1** 99 Ticks abarbeitet, dann darf **JOB 0** ein Tick abarbeiten und der Kreislauf wiederholt sich:

```
[ time 198 ] Run JOB 1 at PRIORITY 2 [ TICKSLEFT 1 RUNTIME 800 TIMELEFT 701 ]
[ time 199 ] IO_START by JOB 1
IO DONE
[ time 199 ] Run JOB 0 at PRIORITY 1 [ TICKSLEFT 99 RUNTIME 200 TIMELEFT 99 ]
[ time 200 ] IO_DONE by JOB 1
[ time 200 ] Run JOB 1 at PRIORITY 2 [ TICKSLEFT 99 RUNTIME 800 TIMELEFT 700 ]
[ time 201 ] Run JOB 1 at PRIORITY 2 [ TICKSLEFT 98 RUNTIME 800 TIMELEFT 699 ]
[ time 202 ] Run JOB 1 at PRIORITY 2 [ TICKSLEFT 97 RUNTIME 800 TIMELEFT 698 ]
```


4.

5. Bei Workloads bei denen mehr als ein Job auf einer Queue läuft hat diese Option ein Effekt.

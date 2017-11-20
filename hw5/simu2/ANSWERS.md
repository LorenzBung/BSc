# Antworten

1. 
   Figure 8.1: A Single Long-Running Job

   `./mlfq.py -l 0,200,0`

   Dieser Aufruf erzeugt einen Job der bei Zeit t=0 startet, 200ms l�uft und keine IO-Time hat.

   Figure 8.2: 

   `./mlfq.py -l 0,200,0:120,20,0`

   Job 0 startet bei t=0 mit 200ms Runtime und Job 1 springt bei t=120 f�r 20ms ein (10 ms Priority 2 + 10ms Prority 1). Wenn man nun das "alte" t=20 als neuen Zeitpunkt 0 sieht, dann hat man das Beispiel aus der Vorlesung.

2. Indem man den Scheduler mit `-m 1` auf nur eine Queue einstellt. Dann werden neue Jobs nicht in eine h�here Queue gestuft und die Priorit�t auch nicht erh�ht.

3.
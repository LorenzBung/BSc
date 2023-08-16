# hw4 - Simulation 3 - Antworten

1.
    1. FIFO: Da immer das älteste Element aus dem Cache geworfen wird, verursacht `1,2,3,4,5,6,1,2,...` ausschließlich Cache-Misses.
    2. LRU: Nun wird immer das am längsten nicht getroffene Element aus dem Cache geworfen. Die Strategie bei FIFO funktioniert auch hier.
    3. MRU: Hier funktioniert die bisher verwendete Folge nicht mehr. Da immer das zuletzt verwendete Element aus dem Cache entfernt wird, ist `1,2,3,4,5,6,5,6,...` eine Möglichkeit, um ausschließlich Cache-Misses zu provozieren. Die Folge `1-6` dient dabei dazu, den Cache zuerst aufzufüllen.
    4. Für unseren Fall (eine Abfolge von 6 verschiedenen Zugriffen) braucht der Cache eine minimale Größe von 6, um sehr viel mehr Cache-Hits zu erreichen. Um genau zu sein, sind dann immer alle Einträge im Cache vorhanden, und nach der anfänglichen Auffüllphase (in der nur Misses auftreten) ist jeder Zugriff ein Hit.

2. Für unser Programm, siehe `randomtrace.c`.
    1. Wir erwarten, dass sich die verschiedenen Policies entsprechend dem, wie wir es in der Vorlesung besprochen hatten, verhalten.

3. *Traces* mit *locality*:

    i. Um räumliche Lokalität (spatial locality) herzustellen darf man die TLB Abdeckung nicht überschreiten bzw. müssen Zahlen gewählt werden, die in der Nähe von einander liegen. Zusätzlich kann man zeitliche Lokalität erzeugen, in dem sich Zahlen wiederholen (temporal locality). Das bedeutet 1,2,2,3,2,3,2,3,2,3,2,2,2,2 ist von räumlicher und zeitlicher Lokalität.

    ii. LRU hatte beim Auruf mit solchen traces eine Hitrate von ca. 80%

    iii. Mit der Policy RAND gab es trotzdem immer die selbe Hitrate wie mit LRU...

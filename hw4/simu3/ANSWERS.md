# hw4 - Simulation 3 - Antworten

1.
    1. FIFO: Da immer das älteste Element aus dem Cache geworfen wird, verursacht `1,2,3,4,5,6,1,2,...` ausschließlich Cache-Misses.
    2. LRU: Nun wird immer das am längsten nicht getroffene Element aus dem Cache geworfen. Die Strategie bei FIFO funktioniert auch hier.
    3. MRU: Hier funktioniert die bisher verwendete Folge nicht mehr. Da immer das zuletzt verwendete Element aus dem Cache entfernt wird, ist `1,2,3,4,5,6,5,6,...` eine Möglichkeit, um ausschließlich Cache-Misses zu provozieren. Die Folge `1-6` dient dabei dazu, den Cache zuerst aufzufüllen.
    Für unseren Fall (eine Abfolge von 6 verschiedenen Zugriffen) braucht der Cache eine minimale Größe von 6, um sehr viel mehr Cache-Hits zu erreichen. Um genau zu sein, sind dann immer alle Einträge im Cache vorhanden, und nach der anfänglichen Auffüllphase (in der nur Misses auftreten) ist jeder Zugriff ein Hit.

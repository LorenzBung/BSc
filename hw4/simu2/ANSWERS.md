# Warmup


# Antworten


1. ​
2. Direkt nach dem Ausführen von ./mem 1024 ging der freie Speicher ständig *gegen 0* und der virtuelle Speicher hat sich zu Begin um ~100000 erhöht. Sobald wir das mem-Programm (mit Ctrl-C) beendet haben, ist der Freie Speicher (free) von ~36 auf ~946812 gesprungen. Anfangs waren aber "nur" ~800000 bei free zu sehen. Der freie Speicher hat sich also vergrößert.
3. Bei ./mem 4000 waren auf dem Rechner (mit 8GB RAM) gab es absolut kein swap-in / swap-out. Es sei denn, der freie Speicher von den gesamten 8GB war kleiner als die ~4GB. Unter normaler Last war erst bei ./mem 700 eine deutlicher Swap sichtbar. Bei jedem der Werte war jedoch der erste *loop* immer langsamer. Bei 4000 war er etwa 40% länger, bei 6000 war er etwa 65% länger und bei 
4. Die CPU Auslastung durch das mem-Programm ist immens. Wenn man es auf dem Laptop ausführt, drehen direkt die Lüfter hoch. Doch wie zu erwarten und im Quellcode zu sehen, ist das C-Programm nicht *multi-threaded*. In einem beliebigen System Monitor (z.B. htop) sieht man, dass nur ein Kern ausgelastet ist.

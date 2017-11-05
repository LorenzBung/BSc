#Simulation 4 - Antworten

1.1. Die Tabellengröße kann mit folgender Gleichung beschrieben werden: `T = a/P` mit `T` = Tabellengröße, `a` = Adressraumgröße und `P` = Seitengröße.

1.2. Aus obiger Formel lässt sich schließen: Mit wachsendem Adressraum sollte die Tabellengröße mit gleichem Faktor wachsen, mit wachsender Seitengröße sollte die Tabellengröße mit gleichem Faktor sinken.

1.3. Bei einem Kontextwechsel muss bei großer Seitengröße mehr geladen werden. Das benötigt Zeit.

2. Je größer der Anteil der zum Adressraum zugeordneten Seiten wird, desto weniger Segmentation-Fehler treten auf.

3. Der Parameter `-P 1m -a 256m -p 512m -v -s 3` ist unrealistisch, da eine Seitengröße von 1MB Größe viel zu groß ist, um praktikabel zu sein.

4. Wenn der Adressraum größer als der physikalische Speicher ist, gibt das Programm eine Fehlermeldung aus.
Weitere Fehler können manuell provoziert werden, wenn zum Beispiel eine negative Seitengröße (Domain Error) oder die Seitengröße mit 0 angegeben werden (Divide by 0). Auch beim angeben eines leeren Adressraums tritt ein Fehler auf (address space must be bigger than 0), genauso auch bei der physikalischen Speichergröße.

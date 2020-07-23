## Antworten zur Simulation 1 (hw2)

1. Beim Ausführen stürzt das Programm mit der Fehlermeldung `Segmentation Fault` ab.
2. GDB gibt den Hinweis `Program received signal SIGSEGV, Segmentation fault.` aus.
3. Valgrind zeigt den Fehler `Invalid Read` und sowohl die Adresse im Speicher als auch die Zeile im Code, in der der Fehler auftritt. Das bedeutet, dass auf eine undefinierte Adresse zugegriffen wird. Da die Zeile im Code auch ausgegeben wird, lässt sich der Ursprung des Fehlers leicht eingrenzen.
4. Das Programm `malloc` erzeugt keine Ausgabe und crasht nicht. GDB bestätigt das: `Inferior 1 (process 2982) exited normally`. Mithilfe von Valgrind sehen wir, dass 10 Bytes nicht freigegeben wurden: `LEAK SUMMARY: definitely lost: 10 bytes in 1 blocks`.
5.
6. Das Programm (`intArray2`) gibt den Wert `0` aus und läuft ohne Fehler. Valgrind weist uns auf einen `invalid read` an der betreffenden Stelle hin.


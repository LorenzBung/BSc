# Warmup

Die Entwicklung des Speichers nach ausführen von `./malloc.py -n 10 -H 0 -p BEST -s` 

```
 --------------- 1000
 |             |
 |             |
 |-------------| 1100
```

```
 --------------- 1000
 |   alloc'd   |
 |-------------| 1003
 |             |
 |             |
 |    FREE     |
 |             |
 |             |
 |-------------| 1100
```

```
 --------------- 1000
 |    FREE     |
 |-------------| 1003
 |             |
 |             |
 |    FREE     |
 |             |
 |             |
 |-------------| 1100
```

```
 --------------- 1000
 |    FREE     |
 |-------------| 1003
 |   alloc'd   |
 |-------------| 1008
 |             |
 |    FREE     |
 |             |
 |-------------| 1100
```
```
 --------------- 1000
 |    FREE     |
 |-------------| 1003
 |    FREE     |
 |-------------| 1008
 |             |
 |    FREE     |
 |             |
 |-------------| 1100
```
```
 --------------- 1000
 |    FREE     |
 |-------------| 1003
 |    FREE     |
 |-------------| 1008
 |   alloc'd   |
 |-------------| 1016
 |    FREE     |
 |-------------| 1100
```
```
 --------------- 1000
 |    FREE     |
 |-------------| 1002
 |   alloc'd   |
 |-------------| 1003
 |    FREE     |
 |-------------| 1008
 |    FREE     |
 |-------------| 1016
 |    FREE     |
 |-------------| 1100
```
```
 --------------- 1000
 |    FREE     |
 |-------------| 1002
 |   alloc'd   |
 |-------------| 1003
 |    FREE     |
 |-------------| 1008
 |   alloc'd   |
 |-------------| 1015
 |    FREE     |
 |-------------| 1016
 |    FREE     |
 |-------------| 1100
```

Wie man sieht hat die Fragmentierung des Speichers bzw. der Free-Liste zugenommen.

# Antworten

1. Wenn zwei mal ein gleich großes Speicherstück alloziiert wird, dann wird nicht das wieder freigewordene Fragment genutzt, sondern ein neues reserviert. Auch Alloziierungen von kleineren Stücken bekommen keine Teile von wieder freigegebenem Platz. Also ensteht im allgemeinen eine höhere Fragmentierung als mit **BEST**.
2. Die Struktur der Free-Liste ändert sich nicht, aber es werden weniger Elemente (Speicherblöcke) gesucht, bevor die Adresse zurückgeliefert wird. Je größer die Free-Liste, desto länger dauert es, diese zu durchsuchen. Das Flag **FIRST** verkürzt also die Suchzeit und somit auch die insgesamte alloc-Zeit.
3. Die verschiedenen Sortiermechanismen:
   - **ADDRSORT** sortiert die Liste mit dem freien Speicher nach der höhe der Adresse. Es verändert sich nichts zu vorher mit anderen Policies.
   - Bei **SIZESORT+** sind die kleinsten *Stücke* vorne in der Liste. Bei **BEST** ist dann oft der Verschnitt, also 1-Byte-Blöcke am Anfang. 
   - Bei **SIZESORT-** wird die Speicherliste absteigend nach der Größe der Blöcke sortiert. Diese Sortierung ist vorteilhaft für die Policy **FIRST**, da große Blöcke direkt am Anfang gefunden werden die sehr häufig für den angeforderten Speicher ausreichen. 
4. Bei größeren Speicheranforderungen schlägt malloc fehl, mit dem Rückgabewert `-1` und die Free-Liste ist sehr groß. Wenn man nun die Verschmelzung von freiem Speicher mit `-C`  aktiviert, dann schlägt keine Allozierung mehr fehl und die Liste des freien Speichers ist kleiner. Die Sortierung der Liste spielt keine Rolle, da keine Fragmente über andere Adressen hinweg miteinander verschmolzen werden können.
5. Wenn ein Faktor von über 50% erlaubt wird, dann bleibt die Liste klein, da der Speicher nicht wieder freigegeben wird. Bei Werten bis zu 100% kommt es definitiv zu einem Fehler bei der Alloziierung, da nicht genug freier Speicher vorhanden ist. Bei Werten, die gegen 0 gehen wird nach jedem **alloc** ein **free** aufgerufen und es ist immer wieder freier Speicher vorhanden.
6. ​In dem man bei jedem **alloc** ein Byte mehr Speicher als davor anfordert, muss immer ein neuer "Block" angefangen werden. Dabei hilft die Policy BEST auch nichts mehr, da die neuen Allozierungen nie in die wieder frei gewordene kleinere Speicherstücke passt.


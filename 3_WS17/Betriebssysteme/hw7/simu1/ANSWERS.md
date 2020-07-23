# hw7 - Simulation 1 - Antworten

### `flag.s`
1. 1.1. Der Assembler-Code versucht, mithilfe einer Variable `flag` einen Lock zu implementieren.

    1.2. Mit den Standardeinstellungen (`./x86.py -p flag.s -M flag,count -R ax,bx`) funktioniert `flag.s` wie erwartet.

    1.3. Es wird ein korrekter Output erzeugt, da `count` pro Thread einmal inkrementiert wird und deswegen am Ende den Wert `2` haben muss.

    1.4. Am Ende der einzelnen Threads wird der Wert `0` in `flag` stehen, da jeder Thread diesen Wert vor Beendigung auf `0` setzt.

2. Beispielsweise mit der Option `-a bx=40,bx=40` sollte das Programm eigentlich am Ende den Wert `80` in `count` stehen haben. Dies ist jedoch nicht der Fall, denn es steht `70` in `count`. Der Lock kann also nicht wie erwartet funktionieren.

3. Wenn wir die Interrupt-Rate auf z.B. `-i 5` setzen (d.h. alle 5 Zyklen ein Interrupt), steht am Ende nur der Wert `40` in `count` (erwartet war `80`). Mit `-i 90` bekommen wir am Ende den Wert `71`. Dies lässt darauf schließen, dass häufige Interrupts eine stärkere Verfälschung des erwarteten Ergebnisses zur Folge haben, während besonders hohe Werte nahezu bis gar keine Auswirkung darauf haben. Je häufiger ein Interrupt auftritt, desto höher ist auch die Wahrscheinlichkeit, einen veralteten Wert zu lesen bzw. zu schreiben. Bei einer sehr hohen Interruptrate (z.B. `-i 1`) bekommen wir daher nur noch die Hälfte des eigentlich erwarteten Ergebnisses, in unserem Fall `40`.

### `test-and-set.s`

Hier wird der Lock durch das Vertauschen des Wertes in `mutex` mit dem Register `ax` gesetzt. Aufgehoben wird der Lock, in dem eine `0` in die Mutex-Variable (`mutex`) geschrieben wird.

1. Das Programm liefert nun immer den erwarteten Wert. Sowohl mit `./x86.py -p test-and-set.s -M mutex,count -R ax,bx -i 1 -a bx=40,bx=40` als auch mit nur `-i 1000` steht am Ende der Wert `80` in `count`.

### `peterson.s`

1. Trotz unterschiedlicher Interruptfrequenzen (`-i 1`, `-i 5`, `-i 10`, `-i 20`...) funktioniert der Code fehlerfrei.

2. Mit beispielsweise `./x86.py -p peterson.s -M count,flag,turn -R ax,bx,cx,fx -a ax=5:bx=0,ax=5:bx=1 -P 11111111000000001000111111111111111` lassen sich mögliche Race Conditions ausschließen, da zwischen dem Testen auf einen Wert und dem zugehörigen Sprung jeweils ein Interrupt stattfindet. Das Endergebnis bleibt davon jedoch unbeeinflusst, was auf fehlerfreien Code schließen lässt.

### `ticket.s`

1. Die Wartezeit der Threads ist abhängig von der Interrupt-Häufigkeit. Wenn nur selten ein Interrupt stattfindet, beispielsweise mit `-i 10000`, hängen die einzelnen Threads sehr lange in der `.aquire`-Schleife fest.

2. Eine größere Threadzahl macht im Gegensatz zur Interrupthäufigkeit keinen Unterschied. Die einzelnen Threads warten zwar dennoch länger, das liegt jedoch daran, dass mehr Tickets vergeben wurden und somit die Wartezeit auch höher ist.

### `yield.s`

1. Beim Aufruf von `./x86.py -p test-and-set.s -M mutex,count -R ax,bx -a ax=5:bx=5,ax=5:bx=5` bzw. `./x86.py -p yield.s -M mutex,count -R ax,bx -a ax=5:bx=5,ax=5:bx=5` ist der Unterschied deutlich zu merken. Der zweite Thread hängt in `test-and-set.s` in einer Schleife beim holen des Locks, und loopt bis zum nächsten Interrupt. `yield.s` tut hier was es soll, und spart somit ca. 50 Zyklen (bzw. etwas weniger als die Interruptrate). Die Einsparung von Zyklen tritt immer dann auf, wenn Thread 1 den Lock hat, dann ein Interrupt kommt und Thread 2 auf den Lock warten muss. Mit `test-and-set.s` wird in diesem Fall ewig geloopt, mit `yield.s` wird die CPU direkt wieder für den ersten Thread freigegeben.

### `test-and-test-and-set.s`

1. Dieser Lock tut dasselbe wie in `test-and-set.s`, nur wird beim holen des Locks ein zweiter Test durchgeführt.

2. Im Vergleich mit `test-and-set.s` muss mit dieser Konfiguration kein atomarer Tausch der Werte in `%ax` und `mutex` ausgeführt werden, wenn der mutex nicht sowieso frei ist. Nur wenn der mutex auch verfügbar ist, wird dieser Tausch durchgeführt.

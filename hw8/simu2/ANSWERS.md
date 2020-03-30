# hw8 - Simulation 2 - Antworten

0. Beim Erstellen wird ein Pointer übergeben, welcher dann in einen `int` gecastet werden soll. Dies funktioniert jedoch nicht, da die Größe unterschiedlich ist. Um dies zu beheben, castet man den Pointer einfach zu einem `long int`, dann stimmt die Größe.

### `main-two-cvs-while.c`

1. Wenn der Buffer größer wird, ändert sich nichts am Code. Die Zahl der produzierten Werte ist jedoch linear zu den Codezeilen. Das Programm ist also nicht abhängig von der Buffergröße.

2. Der Producer wird (sofern die Zahl der produzierten Werte größer ist als die Größe des Buffers) den Buffer komplett füllen und der Consumer dann immer ein Item konsumieren. Im Gegensatz dazu hat der Consumer im ersten Test immer sofort konsumiert, d.h. der Buffer läuft nie voll.

3. Auf der Labshell ist das Verhalten unterschiedlich zu dem auf beispielsweise einem Laptop: Der Consumer braucht auf der Labshell nicht so lange wie auf einem Laptop, da die Laufzeit offensichtlich systemabhängig ist.

4. Da der Producer 10 Werte produziert, wird 10 mal für je eine Sekunde gewartet. Die zu erwartende Laufzeit liegt also über 10 Sekunden.

5. Da der Producer immer noch 10 Werte produziert und dies nicht von der Größe des Buffers abhängig ist, ist die Laufzeit mit einer Buffergröße von 3 also gleich groß wie bei **4**.

6. In diesem Fall werden die produzierten Werte abwechselnd von den einzelnen Consumern konsumiert. Diese warten anschließend für je eine Sekunde. Der Producer produziert 10 Werte, die auf drei Consumer aufgeteilt werden, also `C0: 4, C1: 3, C2: 3`. `C0` wartet 4 mal, also muss die Laufzeit länger als 4 Sekunden dauern.

7. Wie bei **5** ist die Laufzeit hier nicht von der Größe des Buffers abhängig, da die Werte direkt aus dem Buffer konsumiert werden und dieser somit nie voll wird. Die zu erwartende Laufzeit ist also dieselbe wie bei **6**.

### `main-one-cv-while.c`

1. Nein, denn mit einer Condition-Variable und einer while-Schleife können nur Fehler auftreten, wenn ein zweiter Consumer geweckt wird. Da dieser hier jedoch nicht existiert, treten keine Fehler im Programm auf.

2. 

### `main-two-cvs-if.c`

1.

### `main-two-cvs-while-extra-unlock.c`

1.

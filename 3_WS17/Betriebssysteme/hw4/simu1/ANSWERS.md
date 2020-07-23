# hw4 - Simulation 1 - Lösungen

### Warmup

`0x611c` =>

`0110 0001 0001 1100` =>

| Valid | PDE | PTE | Offset |
|-------|-----|-----|--------|
|0      |11000|01000|11100   |
| False | 24  | 8   | 28     |

PDE at index 24:

> page 108: 83 [...] e9 **a1** e8 [...] ff



`0xa1` => `1010 0001` => `1 | 0100001` => Valid | 33

PTE at index 8 in page 33 (found in PDE):

> page 33: 7f [...] 7f **b5** 7f [...] 7f


`0xb5` => `1011 0101` => `1 | 0110101` => Valid | 53

Final Value is in page `0xb5` (53) with offset 28 => 0x08 (8).



### Aufgaben

1. Für eine zweistufige Seitentabelle benötigt man ein Register, für eine dreistufige Seitentabelle bräuchte man ein zweites PDBR.

2. Da es sich bei dieser Simulation um eine zweistufige Seitentabelle handelt, gibt es eine Referenz für den PDE, eine für den PTE und schlussendlich noch eine dritte Referenz, in der das Endergebnis steht.

3. Da der Cache nicht im Voraus weiß, auf welche Adresse nach der Übersetzung des PDE bzw. PTE zugegriffen wird, müssen danach erst wieder Daten in den Cache geladen werden. Dies führt zu einer Reihe von Cache Misses.

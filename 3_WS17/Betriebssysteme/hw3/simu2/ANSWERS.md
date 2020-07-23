## Warmup

| VA Number | Virtual Adress     | Physical Adress     | Valid |
| --------- | :----------------- | ------------------- | ----- |
| VA  0     | 0x11  --> 0010001  | 000010001 --> 0x011 | YES   |
| VA  1     | 0x6c  --> 1101100  | 111101100 --> 0x1ec | YES   |
| VA  2     | 0x61  --> 1100001  | 111100001 --> 0x1e1 | NO    |
| VA  3     | 0x20  --> 0100000  | 00100000 --> 0x020  | NO    |
| VA  4     | 0x3f   --> 0111111 | 00111111 --> 0x03f  | NO    |



```
 --------------- 0x00000000
 |  Segment 0  |
 |-------------| 0x00000014
 |             |
 |             |
 |             |
 |             |
 |-------------| 0x000001ec
 |  Segment 1  |
 |-------------| 0x00000200
```



## Antworten

1.  Die höchste erlaubte Adresse in Segment 0 ist 0x00000013. Die niedrigste valide Addresse des Segments 1 ist 0x000001ec. Die niedrigste illegale Adresse ist 0x0000014 und die höchste illegale Adresse ist 0x000001eb im gesamten Adressraum. Um zu testen ob das stimmt kann mann alle validen  virtuellen Adressen der Segmente UND jeweils (mindestens) eine über dem Limit mit `-A` angeben.

     `-A 0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,`

     `106,107,108,109,110,111,112,113,114,115,116,117,118,119,120,121,122,123,124,125,126,127`


2.   Der Aufruf muss wie folgt aussehen:

    > ./segmentation.py -a 16 -p 128 -A 0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15 **--b0 0 --l0 2 --b1 16 --l1 2** -c

    Die Flags `-l0` und `-l1` setzten jeweils das Limit für das Segment auf 2:

    ​	Segment 0 nimmt also die virtuellen Adressen 0 und 1 an und

    ​	Segment 1 nimmt die Adressen 14 und 15 an

3.   Man sollte das Limit `-l` so wählen, dass es 90% der Adressen des Adressraums `-a` abdeckt. Dann werden die restlichen 10% der virtuellen Adressen über das gesetzte Limit gehen und einen Segmentation Fault auslösen.

4.  Man setzt die Limits der Segmente mit `-l 0` auf 0, dann ist keine virtuelle Adresse valide. 

5.  Im folgenden wird VA für virtuelle Adresse und PA für physikalische Adresse benutzt.

    `Segment 0: 0x0000 bis 0x0040`

    `Segment 1: 0x0380 bis 0x0400`

    Höchste VA **0x16C** -> (mappt auf) höchste PA **0x400**

    1024 - 364 = 660 (Segment 1 Offset)

    | VA Nr. | VA HEX     | VA BIN          | Physical Address HEX   | PA DEC |
    | ------ | ---------- | --------------- | ---------------------- | :----: |
    | 0      | 0x0000005c | 00\|1011100     | Segmentation Violation |   -    |
    | 1      | 0x00000011 | 00\|0010001     | **0x00000011**         |   17   |
    | 2      | 0x00000043 | 00\|1000011     | Segmentation Violation |   -    |
    | 3      | 0x00000021 | 00\|0100001     | **0x00000021**         |   33   |
    | 4      | 0x0000006c | 00\|1101100     | Segmentation Violation |   -    |
    | 5      | 0x0000007a | 00\|1111010     | Segmentation Violation |   -    |
    | 6      | 0x00000050 | 00\|1010000     | Segmentation Violation |   -    |
    | 7      | 0x00000037 | 00\|0110111     | **0x00000037**         |   55   |
    | 8      | 0x000000ff | **01**\|1111111 | **0x00000393**         |  915   |
    | 9      | 0x000000e9 | **01**\|1101001 | Segmentation Violation |   -    |
    | 10     | 0x00000001 | 00\|0000001     | **0x00000001**         |   1    |
    | 11     | 0x0000014c | **10**\|1001100 | **0x000003e0**         |  992   |
    | 12     | 0x000000b4 | **01**\|0110100 | Segmentation Violation |   -    |
    | 13     | 0x000000cf | **01**\|1001111 | Segmentation Violation |   -    |
    | 14     | 0x0000012b | **10**\|0101011 | **0x000003bf**         |  959   |
    | 15     | 0x00000084 | **01**\|0000100 | Segmentation Violation |   -    |

    Zuerst überprüft man in welchem Segment die VA liegt, indem man auf das höchste bit der binären VA schaut. Wenn sie im Segment 0 liegt dann ist VA => PA und man kann prüfen, ob die Adresse im oben berechneten physikalischen Adressbereich von Segment 0 liegt. Wenn nicht => SEG FAULT.

    Wenn die VA in Segment 1 liegt, dann muss man vorher den berechneten Offset vonn dezimal 660 oder hexadezimal 0x258 aufaddieren. Dann hat man die PA und kann dann schauen ob sie im physikalischen Adressbereich von Segment 1 liegt. Wenn nicht => SEG FAULT.

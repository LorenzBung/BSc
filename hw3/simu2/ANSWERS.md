# Antworten

### Warmup

| VA Number | Virtual Adress     | Physical Adress         | Valid |
| --------- | :----------------- | ----------------------- | ----- |
| VA  0     | 0x11  --> 0010001  | **00**0010001 --> 0x11  | YES   |
| VA  1     | 0x6c  --> 1101100  | **11**1101100 --> 0x1ec | YES   |
| VA  2     | 0x61  --> 1100001  | **11**1100001 --> 0x1e1 | NO    |
| VA  3     | 0x20  --> 0100000  | **00**100000 --> 0x1e1  | NO    |
| VA  4     | 0x3f   --> 0111111 | **00**111111 --> 0x1e1  | NO    |



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



## Questions

1.  Die höchste erlaubte Adresse in Segment 0 ist 0x00000020. Die niedrigste valide Addresse des Segments 1 ist 0x000001ec. Um zu testen ob das stimmt kann mann alle validen  Adressen des ersten Segments UND Adressen über dem Limit 

     `-A 0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,106,107,108,109,110,111,112,113,114,115,116,117,118,119,120,121,122,123,124,125,126,127`


2.   Der Aufruf muss wie folgt aussehen:

    > ./segmentation.py -a 16 -p 128 -A 0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15 **--b0 0 --l0 2 --b1 16 --l1 2** -c

    Die Flags `-l0` und `-l1` setzten jeweils das Limit für das Segment auf 2:

    ​	Segment 0 nimmt also die virtuellen Adressen 0 und 1 an und

    ​	Segment 1 nimmt die Adressen 14 und 15 an

3.   Man sollte das Limit `-l` so wählen, dass es 90% der Adressen des Adressraums `-a` abdeckt. Dann werden die restliche 10% über das gesetzte Limit gehen und einen Segmentation Fault auslösen.

4.  Man setzt die Limits der Segmente auf 0, dann ist keine virtuelle Adresse valide.

5.  `Segment 0: 0x0000 bis 0x0040`

    `Segment 1: 0x0380 bis 0x0400`

    | VA Nr. | Virtual Address | Physical Address HEX | PA DEC |
    | ------ | --------------- | -------------------- | ------ |
    | 0      | 0x0000005c      |                      |        |
    | 1      | 0x00000011      |                      |        |
    | 2      | 0x00000043      |                      |        |
    | 3      | 0x00000021      |                      |        |
    | 4      | 0x0000006c      |                      |        |
    | 5      | 0x0000007a      |                      |        |
    | 6      | 0x00000050      |                      |        |
    | 7      | 0x00000037      |                      |        |
    | 8      | 0x000000ff      |                      |        |
    | 9      | 0x000000e9      |                      |        |
    | 10     | 0x00000001      |                      |        |
    | 11     | 0x0000014c      |                      |        |
    | 12     | 0x000000b4      |                      |        |
    | 13     | 0x000000cf      |                      |        |
    | 14     | 0x0000012b      |                      |        |
    | 15     | 0x00000084      |                      |        |

    ​

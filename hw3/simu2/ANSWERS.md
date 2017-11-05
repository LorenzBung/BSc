# Antworten

### Warmup

| VA Number | Virtual Adress | Binary Address | Segment Number |
| --------- | -------------- | -------------- | -------------- |
| VA  0     | 0x00000011     | 0**0**010001   | 0              |
| VA  1     | 0x0000006c     | 0**1**101100   | 1              |
| VA  2     | 0x00000061     | 0**1**100001   | 1              |
| VA  3     | 0x00000020     | 0**0**100000   | 0              |
| VA  4     | 0x0000003f     | 0**0**111111   | 0              |



```
 --------------- 0x00000000
 |  Segment 0  |
 |-------------| 0x00000020
 |             |
 |             |
 |             |
 |             |
 |-------------| 0x000001ec
 |  Segment 1  |
 |-------------| 0x00000200
```



## Questions

1.  Die höchste erlaubte Adresse in Segment 0 ist 0x00000020. Die niedrigste valide Addresse des Segments 1 ist 0x000001ec.
2.  ​
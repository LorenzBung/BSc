# Simulation 1 - Antworten

1. Das letzte Segment liegt auf der dezimalen (virtuellen) Adresse 929. Damit dieses Segment in den Adressraum passt, muss dieser mindestens ein Limit von **930** haben.

2. Der physikalische Adressraum ist 16kB groß und das Limit beträgt 100. 16384 - 100 = 16284

3. Bei einer Vergrößerung des **Adressraums** (z.B. `-a 2k`) nimmt die Zahl der Segmentation-Fehler zu, da das Programm die Adressen aus einer **größeren Spanne** auswählt, das **Limit** der validen Adressen sich jedoch nicht ändert. Vergrößern wir jedoch die Größe des **physikalischen Speichers**, ändert sich nichts am Ergebnis - die Größe des verfügbaren Speichers hat nämlich *keinen Einfluss* darauf, aus welcher **Reichweite** das Programm Adressen auswählt bzw. bis zu welchem Limit Adressen **valide** sind.

4.
Virtual Adress | Segmentation Violation?
---|---
0|no
1|yes
2|no
3|no
4|yes
5|yes
6|yes
7|no
8|yes
9|no

# hw5-task2 Römische Ziffern

Schreiben Sie ein `from` Trait in einer Bibliothek, um arabische Ziffern in
römische Ziffern zu wandeln. Benutzen Sie dafür einen eigenen Datentypen in
Ihrer Bibliothek.

Römische Ziffern benutzen die Zeichen I,V,X,L,C,D,M.

```text
 1  => I
10  => X
 7  => VII
 ...
```

 Die Römer waren sehr erfinderisch. Als Baumeister waren Sie berühmt. Neben
 befestigte Strassen und Brücken haben Sie eine Reihe von imposanten Bauwerken
 erstellt.

 Die `Null` hatten Sie aber scheinbar noch nicht entdeckt, denn es gibt keine 0
 in den römischen Ziffern.

 Auch sehr große Zahlen waren damals nicht wirklich gefordert. Die größte
 arabische Zahl, die Sie in Ihrem `From` Trait wandeln müssen ist die 3000.

 Schauen wir uns dazu als Beispiel die Zahl 1900 an:

-   Die römsche Zahl dafür ist die MCMXC
-   1000=M 900=CM 90=XC

Weitere Informationen und einen Konverter finden Sie z.B. unter [Novaroma].

[novaroma]: http://www.novaroma.org/via_romana/numbers.html

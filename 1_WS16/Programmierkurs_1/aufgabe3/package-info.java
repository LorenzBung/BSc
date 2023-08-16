/**

Paket mit der dritten &Uuml;bungsaufgabe zu Programmiertechnik 1 f&uuml;r AIN/1.

<p>
Innerhalb der Methode
{@link aufgabe3.Notenstatistik#main aufgabe3.Notenstatistik.main}
wird der Benutzer aufgefordert, Pr&uuml;fungsnoten einzugeben.
Ausgabe ist eine Statistik zum Pr&uuml;fungsergebnis.
</p>
<p>
Beispiel (Eingaben des Benutzers sind fett gedruckt):</p>
<pre>
   Noten im Format Ganze,Zehntel oder Ganze.Zehntel eingeben (Ende mit Strg-D):
   <b>1.7 2,3 2-3 3,0 3,1 2,7 5.0 6.0 1 1,75 gut x,y</b>
   Note 2-3 wird wegen Formatfehler ignoriert!
   Note 3,1 wird wegen Nachkommastelle 1 ignoriert!
   Note 6.0 wird wegen Vorkommastelle 6 ignoriert!
   Note 1 wird wegen Formatfehler ignoriert!
   Note 1,75 wird wegen Formatfehler ignoriert!
   Note gut wird wegen Formatfehler ignoriert!
   Note x,y wird wegen Formatfehler ignoriert!
   <b>Strg-D</b>
   Anzahl beruecksichtigter Noten: 5
   Beste Note: 1,7
   Schlechteste Note: 5,0
   Durchschnitt Bestandene: 2,4
   Durchfallquote: 20,0%
</pre>
<p>
Zur Spezifikation der g&uuml;ltigen und ung&uuml;ltigen Eingaben
sowie der Statistik siehe die Dokumentation der Klasse
{@link aufgabe3.Notenstatistik aufgabe3.Notenstatistik}.
Bei ung&uuml;ltigen Eingaben gibt das Programm die im obigen Beispiel
gezeigten Fehlermeldungen aus und ignoriert die Note.
</p>

@see <a href="http://www-home.htwg-konstanz.de/~drachen/prog1/Uebungen.pdf">
     Anleitungen zu den &Uuml;bungen</a>

@author  H.Drachenfels
@version 9.8.2016

*/

package aufgabe3;


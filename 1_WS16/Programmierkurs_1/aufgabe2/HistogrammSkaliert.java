// HistogrammSkaliert.java
package aufgabe2;

import java.util.Scanner;

/**
 * HistogrammSkaliert liest ganze Zahlen zwischen 1 und 6 ein und
 * gibt deren H&auml;ufigkeitsverteilung als Histogramm aus, wobei
 * die Histogrammbalken auf eine Zeilenlänge von 50 skaliert werden.
 * @author Lorenz Bung
 * @version 03.11.16
 */
public final class HistogrammSkaliert {
    private HistogrammSkaliert() { }

    private static final Scanner EINGABE = new Scanner(System.in);

    /**
     * main ist der Startpunkt des Programms.
     * @param args wird nicht verwendet.
     */
    public static void main(final String[] args) {

        /* Definition von Konstanten, damit Checkstyle nicht
           wegen "MagicNumber" jammert */
        final int arraySize = 6;
        final int counterOfPosition = 5;
        final int lineSize = 50;

        /* (1) hier ein Feld von Zaehlern definieren */
        int[] counter = new int[arraySize];


        //-------------------------------------------------- Zahlen einlesen
        System.out.println("Ganze Zahlen zwischen 1 und 6 eingeben "
                    + "(Ende mit Strg-D):");

        while (EINGABE.hasNext()) {
            if (EINGABE.hasNextInt()) {    // Falls die Eingabe eine Zahl ist
                int number = EINGABE.nextInt();

                /* (2) hier Anweisungen fuer das
                             Pruefen und Zaehlen der Eingabe schreiben */
                if (number > 0 && number <= arraySize) {
                    counter[number - 1]++;
                } else {    //Falls die Eingabe nicht zwischen 1 und 6 liegt
                    System.out.println("\nFalsche Eingabe wird ignoriert: "
                        + number);
                }
            } else {    //Falls die Eingabe keine Zahl ist
                System.out.println("Falsche Eingabe wird ignoriert: "
                    + EINGABE.next());
            }
        }

        //---------------------------------------------- Histogramm ausgeben
        System.out.println("\n+------------+\n| Histogramm "
            + "|\n+------------+\n");

        // Ermittle die längste Zeichenkette des Arrays
        int maxArrayEntry = 0;
        for (int i = 0; i < arraySize; i++) {
            if (maxArrayEntry < counter[i]) {
                maxArrayEntry = counter[i];
            }
        }
        // Definiere den Skalierungsfaktor für das Histogramm
        int faktor = maxArrayEntry / lineSize + 1;
        for (int i = 0; i < arraySize; i++) {
            /*
             * Die Zahl der tatsächlich auszugebenden Zeichen wird als
             * Wert timesPrinted abgespeichert.
             */
            int timesPrinted = counter[i] / faktor;
            for (int j = 1; j <= timesPrinted; j++) {
                /*
                 * Wenn das aktuelle Zeichen an der 5. Stelle ist, soll es
                 * durch ein "$" ersetzt werden.
                 */
                if (j % counterOfPosition == 0 && j != 0) {
                    System.out.print("$");
                } else {
                    System.out.print(i + 1);
                }
                /*
                 * Nun soll die Zahl der Zeichen ausgegeben werden, aber nur,
                 * wenn das letzte Zeichen des Histogrammbalkens bereits
                 * ausgegeben wurde.
                 */
                if (j == timesPrinted) {
                    System.out.println(" (" + counter[i] + ")");
                }
            }
        }
        // Eine leere Zeile macht die Darstellung des Histogramms schöner
        System.out.println();
    }
}


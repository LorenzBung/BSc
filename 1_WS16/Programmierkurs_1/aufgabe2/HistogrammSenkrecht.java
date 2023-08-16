// HistogrammSenkrecht.java
package aufgabe2;

import java.util.Scanner;

/**
 * HistogrammSenkrecht liest ganze Zahlen zwischen 1 und 6 ein und
 * gibt deren H&auml;ufigkeitsverteilung als senkrechtes Histogramm aus.
 * @author Lorenz Bung
 * @version 15.11.16
 */
public final class HistogrammSenkrecht {
    private HistogrammSenkrecht() { }

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

        /* (1) hier ein Feld von Zaehlern definieren */
        int[] counter = new int[arraySize];


        //-------------------------------------------------- Zahlen einlesen
        System.out.println("Ganze Zahlen zwischen 1 und 6 eingeben "
                    + "(Ende mit Strg-D):");

        while (EINGABE.hasNext()) {
            if (EINGABE.hasNextInt()) {
                int number = EINGABE.nextInt();

                /* (2) hier Anweisungen fuer das
                             Pruefen und Zaehlen der Eingabe schreiben */
                if (number > 0 && number <= arraySize) {
                    counter[number - 1]++;
                } else {
                    System.out.println("\nFalsche Eingabe wird ignoriert: "
                        + number);
                }
            } else {
                System.out.println("Falsche Eingabe wird ignoriert: "
                    + EINGABE.next());
            }
        }

        //---------------------------------------------- Histogramm ausgeben
        System.out.println("\n +------------+\n | Histogramm "
            + "|\n +------------+\n");

        /* (3) hier Anweisungen fuer die Histogrammausgabe schreiben */

        // Berechne den maximalen Wert im Array
        int maxArrayEntry = 0;
        for (int i = 0; i < arraySize; i++) {
            if (maxArrayEntry < counter[i]) {
                maxArrayEntry = counter[i];
            }
        }

        /*
         * Es muessen so lange Ausgaben erzeugt werden, bis maxArrayEntry
         * erreicht ist.
         */
        for (int i = 0; i < maxArrayEntry; i++) {

            // Gehe durch das Array und gebe die entsprechende Ausgabe aus.
            for (int j = 0; j < arraySize; j++) {
                if (counter[j] != 0) {
                    System.out.print((j + 1) + " ");
                } else {
                    System.out.print("  ");
                }
            }

            // Dekrementiert jeden Wert im Array, falls er über 0 ist.
            for (int j = 0; j < arraySize; j++) {
                if (counter[j] > 0) {
                    counter[j]--;
                }
            }

            // Hier wird die Zeilennummer angehängt.
            System.out.println(" (" + (i + 1) + ")");
        }

        // Eine leere Zeile macht die Ausgabe schoener
        System.out.println();
    }
}

//EOF

// Histogramm.java
package aufgabe2;

import java.util.Scanner;

/**
 * Histogramm liest ganze Zahlen zwischen 1 und 6 ein und
 * gibt deren H&auml;ufigkeitsverteilung als Histogramm aus.
 * @author Lorenz Bung
 * @version 03.11.16
 */
public final class Histogramm {
    private Histogramm() { }

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

        /* (3) hier Anweisungen fuer die Histogrammausgabe schreiben */
        for (int i = 0; i < arraySize; i++) {
            for (int j = 1; j <= counter[i]; j++) {
                if (j % counterOfPosition == 0 && j != 0) {
                    System.out.print("$");
                } else {
                    System.out.print(i + 1);
                }
            }
            System.out.println(" (" + counter[i] + ")");
        }
    }
}


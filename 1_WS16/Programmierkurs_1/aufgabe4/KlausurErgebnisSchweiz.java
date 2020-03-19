// KlausurErgebnisSchweiz.java
package aufgabe4;

import java.util.Locale;
import java.util.Scanner;

/**
 * erstellt eine Notenstatistik.
 * <p>
 * Das Programm erwartet Pr&uuml;fungsnoten im Format
 * <tt>Ganze,Zehntel</tt> oder <tt>Ganze.Zehntel</tt>,
 * wobei <tt>Ganze</tt> und <tt>Zehntel</tt> nur aus
 * je einer Dezimalziffer bestehen d&uuml;rfen.
 * Andere Eingaben werden wegen Formatfehler ignoriert.
 * </p>
 * <p>
 * Das Programm gibt die folgende Statistik aus:
 * </p>
 * <ul>
 * <li>die Anzahl der ber&uuml;cksichtigten Noten</li>
 * <li>die beste Note</li>
 * <li>die schlechteste Note</li>
 * <li>den Durchschnitt der Bestandenen</li>
 * <li>die Durchfallquote in Prozent</li>
 * </ul>
 * <p>
 * Es werden in der Statistik nur die nach HTWG-Pr&uuml;fungsordnung
 * zul&auml;ssigen Noten (1,0 1,3 1,7 2,0 2,3 2,7 3,0 3,3 3,7 4,0 5,0)
 * ber&uuml;cksichtigt.
 * Andere Eingaben werden wegen falscher Vorkommastelle oder
 * falscher Nachkommastelle ignoriert.
 * Alle Noten bis 4,0 gelten als bestanden, nur die 5,0 als durchgefallen.
 * </p>
 *
 * @author Lorenz Bung
 * @version 04.12.16
 */
public final class KlausurErgebnisSchweiz {
    private KlausurErgebnisSchweiz() { }

    private static final Scanner EINGABE = new Scanner(System.in);

    /**
     * main ist der Startpunkt des Programms.
     * @param args wird nicht verwendet.
     */
    public static void main(final String[] args) {
        Locale.setDefault(Locale.GERMANY);

        int passed = 0;
        int failed = 0;
        double best = 1.0;
        double worst = 6.0;
        double average = 0.0;

        //--------------------------------------------------- Noten einlesen
        System.out.println("Noten im Format Ganze,Zehntel "
                           + "eingeben (Ende mit Strg-D):");


        while (EINGABE.hasNext()) {
            String note = EINGABE.next();
            // Temporaerer Wert fuer die Note.
            double noteValue = 0.0;

            //---------------------------------------------- Eingabe pruefen

            boolean isValid = true;

            try {
                noteValue = Noten.doubleNote(note);
            } catch (Exception e) {
                isValid = false;
            }
            //------------------------------------------------ Note erfassen

            /* (2) Notensumme Bestandene, Anzahl Bestandene,
                         Anzahl Durchgefallene sowie
                         beste und schlechteste Note aktualisieren ... */

            // Aktualisiere bestandene und durchgefallene
            if (isValid) { // Falls die Note gueltig ist, werte sie aus
                if (Noten.istBestanden(noteValue)) {
                    passed++;
                    // Aktualisiere den Schnitt der Bestandenen
                    if (average != 0.0) {
                        average = ((average * (passed - 1)) / passed)
                                  + (noteValue / passed);
                    } else {
                        average = noteValue;
                    }
                } else if (noteValue != 0.0) {
                    failed++;
                }

                // Aktualisiere beste und schlechteste Note
                best = Noten.bessere(best, noteValue);
                worst = Noten.schlechtere(worst, noteValue);

            } else { // Gib eine Fehlermeldung aus
                System.out.println("Unzulaessige Note " + note
                                   + " wird ignoriert!");
            }

        } // while

        //------------------------------------------ Notenstatistik ausgeben
        System.out.println();
        System.out.println("Anzahl beruecksichtigter Noten: "
                           + (passed + failed));
        if ((passed + failed) != 0) {
            System.out.printf("Beste Note: %.1f%n", best);
            System.out.printf("Schlechteste Note: %.1f%n", worst);

            /* (3) Durchschnitt und Durchfallquote berechnen
             und dann die gesamte Statistik ausgeben ... */
            if (passed != 0) {
                System.out.printf("Durchschnitt Bestandene: %.1f%n", average);
            }
            double failedQuote = failed / (failed + passed + 0.0);
            final int percent = 100;
            System.out.printf("Durchfallquote: %.1f", (failedQuote * percent));
            System.out.println("%");
        }

    } // main
}


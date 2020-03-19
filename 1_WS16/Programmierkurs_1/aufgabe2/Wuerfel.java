// Wuerfel.java

package aufgabe2;

import java.util.Random;

/**
 * Wuerfel gibt Zufallszahlen zwischen 1 und 6 aus.
 * @author H.Drachenfels
 * @version 20.2.2013
 */
public final class Wuerfel {
    private Wuerfel() { }

    /**
     * main ist der Startpunkt des Programms.
     * @param args Anzahl der Zufallszahlen (Default 1)
     */
    public static void main(final String[] args) {
        int versuche = 1;

        if (args.length > 0) {
            try {
                versuche = Integer.parseInt(args[0]);
            } catch (NumberFormatException x) {
                versuche = 0;
            }

            if (versuche == 0 || args.length > 1) {
                System.err.printf("Zu viele oder falsche Parameter!%n"
                                  + "Aufruf: Wuerfel [Anzahl]%n");
                return;
            }
        }

        Random wuerfel = new Random();
        final int anzahlAugen = 6;
        for (int i = 0; i < versuche; ++i) {
            System.out.println(wuerfel.nextInt(anzahlAugen) + 1);
        }
    }
}


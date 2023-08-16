// Notenspiegel.java

package aufgabe5;

import aufgabe4.Noten;
import java.util.NoSuchElementException;
import java.util.Scanner;

/**
 * NotenspiegelBonus liest die Namen von F&auml;chern mit den zugeh&ouml;rigen Noten
 * in eine verkettete Liste ein und gibt dann einen nach Notenwert sortierten
 * Notenspiegel aus.
 * @author Lorenz Bung
 * @version 20.12.16
 */
public final class NotenspiegelBonus {
    private NotenspiegelBonus() { }

    private static final Scanner EINGABE = new Scanner(System.in);

    /**
     * main ist der Startpunkt des Programms.
     * @param args wird nicht verwendet.
     */
    public static void main(final String[] args) {
        FachNote listenAnfang = null; // leere Liste

        //--------------------------------------------- Notenspiegel einlesen
        System.err.println("Fach und Note eingeben (Ende mit Strg-D):");

        while (EINGABE.hasNext()) {
            try {
                // Fach und Note einlesen:

                /* (1) erst Fach und dann Note mit next() einlesen */
                String fach = EINGABE.next();
                String tempNote = EINGABE.next();
                double noteValue = Noten.doubleNote(tempNote.replace(",", "."));



                // neue Fachnote in Notenliste eintragen:

                /* (2) ein neues FachNote-Objekt erzeugen
                             und am Listenanfang einfuegen */

                FachNote nachfolger = null;
                for (FachNote listenIterator = listenAnfang; listenIterator != null
                    && listenIterator.note < noteValue;
                    listenIterator = listenIterator.naechste) {
                    nachfolger = listenIterator;
                }
                if (nachfolger == null) {
                    listenAnfang = new FachNote(fach, noteValue, listenAnfang);
                } else {
                    nachfolger.naechste = new FachNote(fach, noteValue, nachfolger.naechste);
                }




                /*
                if (listenAnfang == null) {
                    listenAnfang = newNote;
                } else if (listenAnfang.naechste == null) {
                    listenIterator.naechste = newNote;
                } else {
                    FachNote listenIterator = listenAnfang;
                    while (listenIterator != null && listenIterator.naechste != null) {
                        if (listenIterator.note < newNote.note
                            && listenIterator.naechste.note > newNote.note) {
                            // Neue Note schlechter: Hinter sich einfuegen
                            newNote.naechste = listenIterator.naechste;
                            listenIterator.naechste = newNote;
                            System.out.println(listenIterator.naechste);
                            break;
                        } else if (listenIterator.naechste == null) {
                            listenIterator.naechste = newNote;
                            break;
                        } else if (listenAnfang.note > newNote.note) {
                            newNote.naechste = listenIterator;
                            listenAnfang = newNote;
                            break;
                        }
                        listenIterator = listenIterator.naechste;
                    }
                }
                */

            } catch (IllegalArgumentException x) {
                System.err.printf("Eingabefehler: %s%n", x.getMessage());
                continue;
            } catch (NoSuchElementException x) {
                System.err.println("Fach ohne Note ignoriert!");
                break;
            }
        }

        //--------------------------------------------- Notenspiegel ausgeben

        /* (3) tabellarischen Notenspiegel
                     mit der Ueberschrift NOTENSPIEGEL ausgeben */
        System.out.println("\nNOTENSPIEGEL");
        // Variablen, um durch die Liste zu iterieren und
        // um die laenge des laengsten Strings zu errechnen
        FachNote tempListenAnfang = listenAnfang;
        int laenge = 0;

        // errechne den laengsten String in der Liste
        while (tempListenAnfang != null) {
            if (tempListenAnfang.fach.length() > laenge) {
                laenge = tempListenAnfang.fach.length();
            }
            tempListenAnfang = tempListenAnfang.naechste;
        }

        // Ausgabe von Fach, entsprechend vielen Leerzeichen und Note
        while (listenAnfang != null) {
            int leerzeichenZahl = 0;
            if (listenAnfang.fach.length() < laenge) {
                leerzeichenZahl = laenge - listenAnfang.fach.length();
            }
            System.out.print(listenAnfang.fach);
            for (int i = 0; i < leerzeichenZahl; i++) {
                System.out.print(" ");
            }
            System.out.println("    " + listenAnfang.note);
            listenAnfang = listenAnfang.naechste;
        }


    } // main
}


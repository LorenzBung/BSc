// ByteVar.java

package aufgabe1;

import java.util.Scanner;

/**
 * ByteVar zeigt den Umgang mit Variablen vom Typ byte.
 * &Uuml;bungsaufgabe 1 zur Programmiertechnik 1.
 * @author Lorenz Bung
 * @version 20.10.16
 */
public final class ByteVar {
    private ByteVar() { }

    private static final Scanner EINGABE = new Scanner(System.in);

    /**
     * main ist der Startpunkt des Programms.
     * @param args wird nicht verwendet.
     */
    public static void main(final String[] args) {
        /* Konstanten min und max definieren */
        final byte min = -128;
        final byte max = 127;


        /* Eingabeaufforderung ausgeben */
        System.out.println("Erste ganze Zahl zwischen "
            + min + " und " + max + " eingeben:");


        /* Erste ganze Zahl einlesen */
        while (!EINGABE.hasNextByte()) {
            System.out.println("Bitte gültigen Wert eingeben!");
            EINGABE.next();
        }
        byte first = EINGABE.nextByte();

        /* Eingabeaufforderung ausgeben */
        System.out.println("Zweite ganze Zahl zwischen "
            + min + " und " + max + " eingeben:");

        /* Zweite ganze Zahl einlesen */
        while (!EINGABE.hasNextByte()) {
            System.out.println("Bitte gültigen Wert eingeben!");
            EINGABE.next();
        }
        byte second = EINGABE.nextByte();


        /* Die beiden Zahlen dezimal, okatal und hexadezimal ausgeben */
        System.out.printf("%d ist oktal %o und hexadezimal %h%n",
            first, first, first);
        System.out.printf("%d ist oktal %o und hexadezimal %h%n",
            second, second, second);


        /* Alle zweistelligen arithmetischen Operatoren ausprobieren */
       // byte sum = first + second;
       // byte sub = first - second;
       // byte pro = first * second;
       // byte div = first / second;
       // byte mod = first % second;

        System.out.println(first + " + " + second + " ist "
            + (first + second));
        System.out.println(first + " - " + second + " ist "
            + (byte) (first - second));
        System.out.println(first + " * " + second + " ist "
            + (byte) (first * second));
        System.out.println(first + " / " + second + " ist "
            + (byte) (first / second));
        System.out.println(first + " % " + second + " ist "
            + (byte) (first % second));


        /* Alle Vergleichsoperatoren ausprobieren */
       // boolean eq = (first == second);
       // boolean ne = !eq;
       // boolean lt = (first < second);
       // boolean le = (first <= second);
       // boolean gt = (first > second);
       // boolean ge = (first >= second);

        System.out.println(first + " == " + second + " ist "
            + (first == second));
        System.out.println(first + " != " + second + " ist "
            + (first != second));
        System.out.println(first + " < " + second + " ist "
            + (first < second));
        System.out.println(first + " <= " + second + " ist "
            + (first <= second));
        System.out.println(first + " > " + second + " ist "
            + (first > second));
        System.out.println(first + " >= " + second + " ist "
            + (first >= second));
    }
}


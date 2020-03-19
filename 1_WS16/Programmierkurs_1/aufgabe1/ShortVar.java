// ShortVar.java

package aufgabe1;

import java.util.Scanner;

/**
 * ShortVar zeigt den Umgang mit Variablen vom Typ short.
 * &Uuml;bungsaufgabe 1 zur Programmiertechnik 1.
 * @author Lorenz Bung
 * @version 20.10.16
 */
public final class ShortVar {
    private ShortVar() { }

    private static final Scanner EINGABE = new Scanner(System.in);

    /**
     * main ist der Startpunkt des Programms.
     * @param args wird nicht verwendet.
     */
    public static void main(final String[] args) {
        /* Konstanten min und max definieren */
        final short min = -32768;
        final short max = 32767;


        /* Eingabeaufforderung ausgeben */
        System.out.println("Erste ganze Zahl zwischen "
            + min + " und " + max + " eingeben:");


        /* Erste ganze Zahl einlesen */
        while (!EINGABE.hasNextShort()) {
            System.out.println("Bitte gültigen Wert eingeben!");
            EINGABE.next();
        }
        short first = EINGABE.nextShort();

        /* Eingabeaufforderung ausgeben */
        System.out.println("Zweite ganze Zahl zwischen "
            + min + " und " + max + " eingeben:");

        /* Zweite ganze Zahl einlesen */
        while (!EINGABE.hasNextShort()) {
            System.out.println("Bitte gültigen Wert eingeben!");
            EINGABE.next();
        }
        short second = EINGABE.nextShort();


        /* Die beiden Zahlen dezimal, okatal und hexadezimal ausgeben */
        System.out.printf("%d ist oktal %o und hexadezimal %h%n",
            first, first, first);
        System.out.printf("%d ist oktal %o und hexadezimal %h%n",
            second, second, second);


        /* Alle zweistelligen arithmetischen Operatoren ausprobieren */
        //short sum = first + second;
        //short sub = first - second;
        //short pro = first * second;
        //short div = first / second;
        //short mod = first % second;

        System.out.println(first + " + " + second + " ist "
            + (short) (first + second));
        System.out.println(first + " - " + second + " ist "
            + (short) (first - second));
        System.out.println(first + " * " + second + " ist "
            + (short) (first * second));
        System.out.println(first + " / " + second + " ist "
            + (short) (first / second));
        System.out.println(first + " % " + second + " ist "
            + (short) (first % second));


        /* Alle Vergleichsoperatoren ausprobieren */
        //boolean eq = (first == second);
        //boolean ne = !eq;
        //boolean lt = (first < second);
        //boolean le = (first <= second);
        //boolean gt = (first > second);
        //boolean ge = (first >= second);

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


// IntVar.java

package aufgabe1;

import java.util.Scanner;

/**
 * IntVar zeigt den Umgang mit Variablen vom Typ int.
 * &Uuml;bungsaufgabe 1 zur Programmiertechnik 1.
 * @author Lorenz Bung
 * @version 20.10.16
 */
public final class IntVar {
    private IntVar() { }

    private static final Scanner EINGABE = new Scanner(System.in);

    /**
     * main ist der Startpunkt des Programms.
     * @param args wird nicht verwendet.
     */
    public static void main(final String[] args) {
        /* Konstanten min und max definieren */
        final int min = -2147483648;
        final int max = 2147483647;


        /* Eingabeaufforderung ausgeben */
        System.out.println("Erste ganze Zahl zwischen "
            + min + " und " + max + " eingeben:");


        /* Erste ganze Zahl einlesen */
        while (!EINGABE.hasNextInt() && !EINGABE.hasNext()) {
            System.out.println("Bitte gültigen Wert eingeben!");
        }
        while (!EINGABE.hasNextInt() && EINGABE.hasNext()) {
            System.out.println("Bitte gültigen Wert eingeben!");
            EINGABE.next();
        }
        int first = EINGABE.nextInt();

        /* Eingabeaufforderung ausgeben */
        System.out.println("Zweite ganze Zahl zwischen "
            + min + " und " + max + " eingeben:");

        /* Zweite ganze Zahl einlesen */
        while (!EINGABE.hasNextInt() && !EINGABE.hasNext()) {
            System.out.println("Bitte gülitgen Wert eingeben!");
        }
        while (!EINGABE.hasNextInt() && EINGABE.hasNext()) {
            System.out.println("Bitte gültigen Wert eingeben!");
            EINGABE.next();
        }
        int second = EINGABE.nextInt();


        /* Die beiden Zahlen dezimal, okatal und hexadezimal ausgeben */
        System.out.printf("%d ist oktal %o und hexadezimal %h%n",
            first, first, first);
        System.out.printf("%d ist oktal %o und hexadezimal %h%n",
            second, second, second);


        /* Alle zweistelligen arithmetischen Operatoren ausprobieren */
        int sum = first + second;
        int sub = first - second;
        int pro = first * second;
        int div = first / second;
        int mod = first % second;

        System.out.println(first + " + " + second + " ist " + sum);
        System.out.println(first + " - " + second + " ist " + sub);
        System.out.println(first + " * " + second + " ist " + pro);
        System.out.println(first + " / " + second + " ist " + div);
        System.out.println(first + " % " + second + " ist " + mod);


        /* Alle Vergleichsoperatoren ausprobieren */
        boolean eq = (first == second);
        boolean ne = !eq;
        boolean lt = (first < second);
        boolean le = (first <= second);
        boolean gt = (first > second);
        boolean ge = (first >= second);

        System.out.println(first + " == " + second + " ist " + eq);
        System.out.println(first + " != " + second + " ist " + ne);
        System.out.println(first + " < " + second + " ist " + lt);
        System.out.println(first + " <= " + second + " ist " + le);
        System.out.println(first + " > " + second + " ist " + gt);
        System.out.println(first + " >= " + second + " ist " + ge);
    }
}


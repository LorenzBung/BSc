// IntVarLong.java

package aufgabe1;

import java.util.Scanner;

/**
 * IntVarLong zeigt den Umgang mit Variablen vom Typ int.
 * &Uuml;bungsaufgabe 1 zur Programmiertechnik 1.
 * @author Lorenz Bung
 * @version 20.10.16
 */
public final class IntVarLong {
    private IntVarLong() { }

    private static final Scanner EINGABE = new Scanner(System.in);


    /**
     * Konstante MIN definieren.
     */
    static final int MIN = -2147483648;
    /**
     * Konstante MAX definieren.
     */
    static final int MAX = 2147483647;

    /**
     * main ist der Startpunkt des Programms.
     * @param args wird nicht verwendet.
     */
    public static void main(final String[] args) {
        String[] operators = {"+", "-", "*", "/", "%", "==",
            "!=", "<", "<=", ">", ">="};


        /* Eingabeaufforderung ausgeben */
        System.out.println("Erste ganze Zahl zwischen "
            + MIN + " und " + MAX + " eingeben:");


        /* Erste ganze Zahl einlesen */
        while (!EINGABE.hasNextInt()) {
            System.out.println("Bitte gültigen Wert eingeben!");
            continue;
        }
        long first = EINGABE.nextInt();

        /* Eingabeaufforderung ausgeben */
        System.out.println("Zweite ganze Zahl zwischen "
            + MIN + " und " + MAX + " eingeben:");

        /* Zweite ganze Zahl einlesen */
        while (!EINGABE.hasNextInt()) {
            System.out.println("Bitte gültigen Wert eingeben!");
            continue;
        }
        long second = EINGABE.nextInt();

        /* Eingabeaufforderung ausgeben */
        System.out.println("Operator eingeben:");

        /* Operator einlesen */
        boolean isOperator = false;
        String operatorString = "";
        while (EINGABE.hasNext()) {
            String tempOperatorString = EINGABE.next();
            for (int i = 0; i < operators.length; i++) {
                if (tempOperatorString.equals(operators[i])) {
                    isOperator = true;
                }
            }
            if (isOperator) {
                operatorString = tempOperatorString;
                break;
            } else {
                System.out.println("Bitte gültigen Operator eingeben!");
            }
        }


        /* Die beiden Zahlen dezimal, okatal und hexadezimal ausgeben */
        System.out.printf("%d ist oktal %o und hexadezimal %h%n",
            first, first, first);
        System.out.printf("%d ist oktal %o und hexadezimal %h%n",
            second, second, second);
        switch (operatorString) {
        default:
            break;
        case "+":
            System.out.println(first + " + " + second + " ist "
                + isInteger(first + second));
            break;
        case "-":
            System.out.println(first + " - " + second + " ist "
                + isInteger(first - second));
            break;
        case "*":
            System.out.println(first + " * " + second + " ist "
                + isInteger(first * second));
            break;
        case "/":
            System.out.println(first + " / " + second + " ist "
                + isInteger(first / second));
            break;
        case "%":
            System.out.println(first + " % " + second + " ist "
                + isInteger(first % second));
            break;
        case "==":
            System.out.println(first + " == " + second + " ist "
                + (first == second));
            break;
        case "!=":
            System.out.println(first + " != " + second + " ist "
                + (first != second));
            break;
        case "<":
            System.out.println(first + " < " + second + " ist "
                + (first < second));
            break;
        case "<=":
            System.out.println(first + " <= " + second + " ist "
                + (first <= second));
            break;
        case ">":
            System.out.println(first + " > " + second + " ist "
                + (first > second));
            break;
        case ">=":
            System.out.println(first + " >= " + second + " ist "
                + (first >= second));
            break;
        }
    }
    /**
     * isInteger testet, ob sich der übergebene Wert long im
     * Wertebereich für Integer liegt und gibt einen ent-
     * sprechenden String zurück.
     * @param testValue Der Wert, dessen Größe überprüft wird.
     * @return Der String, welcher zurückgegeben wird.
     */
    public static String isInteger(final long testValue) {
        if (testValue <= MAX && testValue >= MIN) {
            return Long.toString(testValue);
        } else {
            return "nicht im Wertebereich für Integer.";
        }
    }
}


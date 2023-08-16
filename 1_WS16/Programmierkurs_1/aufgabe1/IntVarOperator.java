// IntVarOperator.java

package aufgabe1;

import java.util.Scanner;

/**
 * IntVarOperator zeigt den Umgang mit Variablen vom Typ int.
 * &Uuml;bungsaufgabe 1 zur Programmiertechnik 1.
 * @author Lorenz Bung
 * @version 20.10.16
 */
public final class IntVarOperator {
    private IntVarOperator() { }

    private static final Scanner EINGABE = new Scanner(System.in);

    /**
     * main ist der Startpunkt des Programms.
     * @param args wird nicht verwendet.
     */
    public static void main(final String[] args) {
        /* Konstanten min und max definieren */
        final int min = -2147483648;
        final int max = 2147483647;
        String[] operators = {"+", "-", "*", "/", "%", "==",
            "!=", "<", "<=", ">", ">="};


        /* Eingabeaufforderung ausgeben */
        System.out.println("Erste ganze Zahl zwischen "
            + min + " und " + max + " eingeben:");


        /* Erste ganze Zahl einlesen */
        while (!EINGABE.hasNextInt()) {
            System.out.println("Bitte gültigen Wert eingeben!");
            EINGABE.next();
        }
        int first = EINGABE.nextInt();

        /* Eingabeaufforderung ausgeben */
        System.out.println("Zweite ganze Zahl zwischen "
            + min + " und " + max + " eingeben:");

        /* Zweite ganze Zahl einlesen */
        while (!EINGABE.hasNextInt()) {
            System.out.println("Bitte gültigen Wert eingeben!");
            EINGABE.next();
        }
        int second = EINGABE.nextInt();

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
                + (first + second));
            break;
        case "-":
            System.out.println(first + " - " + second + " ist "
                + (first - second));
            break;
        case "*":
            System.out.println(first + " * " + second + " ist "
                + (first * second));
            break;
        case "/":
            System.out.println(first + " / " + second + " ist "
                + (first / second));
            break;
        case "%":
            System.out.println(first + " % " + second + " ist "
                + (first % second));
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
}


package aufgabe6;

/**
 * Hilfsklasse Faecher, die die Inhalte des AIN-Studienplans beschreibt.
 * @author Lorenz Bung
 * @version 10.01.16
 */
public final class Faecher {

    private Faecher() { }

    /**
     * Klassenvariable, welche alle zulaessigen Fachnamen des AIN-Studienplans
     * im ersten Semester enthaelt.
     */
    private static final String[] FAECHER = {"Konsolidierung", "Mathematik 1",
        "Programmiertechnik 1", "Studienmethodik", "Digitaltechnik",
        "Systemmodellierung"};

    /**
     * istZulaessig ueberprueft, ob der uebergebene String ein zulaessiger
     * Fachname ist.
     * @param s Der String, welcher ueberprueft wird.
     * @return Der Wert, welcher angibt, ob der ueberpruefte String
     * zulaessig ist.
     */
    public static boolean istZulaessig(final String s) {
        for (String f:FAECHER) {
            if (f.equals(s)) {
                return true;
            }
        }
        return false;
    }
}

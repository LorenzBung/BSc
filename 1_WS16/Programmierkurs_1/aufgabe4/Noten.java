package aufgabe4;

/**
 * Die Klasse Noten ist eine Hilfsklasse, die das Notensystem der
 * HTWG Konstanz beschreibt.
 */
public final class Noten {
    /**
     * Definiere Konstante fuer die beste moegliche Note.
     */
    public static final double BESTE = 1.0;
    /**
     * Definiere Konstante fuer die schlechteste moegliche Note.
     */
    public static final double SCHLECHTESTE = 5.0;

    /**
     * Die Methode istZulaessig errechnet, ob die uebergebene Note zulaessig
     * ist oder nicht.
     * @param note Die Note, welche ueberprueft wird
     * @return Der boolean, welcher angibt, ob die Note zulaessig ist
     */
    public static boolean istZulaessig(final String note) {
        if (note.length() == 3) {
            if (Character.isDigit(note.charAt(0))
                    && Character.isDigit(note.charAt(2))
                    && (note.charAt(1) == ',' || note.charAt(1) == '.')) {
                switch (note.charAt(0)) {
                case '1':
                case '2':
                case '3':
                    return (note.charAt(2) == '0'
                            || note.charAt(2) == '3'
                            || note.charAt(2) == '7');
                case '4':
                case '5':
                    return (note.charAt(2) == '0');
                default:
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    /**
     * Die Methode doubleNote nimmt eine Note vom Typ String und liefert sie
     * als double zurueck.
     * @param noteString Der String der Note, welche ueberprueft wird.
     * @return Die zu einem double konvertierte Note.
     */
    public static double doubleNote(final String noteString) {
        if (istZulaessig(noteString)) {
            return Double.parseDouble(noteString);
        } else {
            throw new IllegalArgumentException("Unzulaessige Note " + noteString
                                               + " wird ignoriert!");
        }
    }

    /**
     * Die Methode istBestanden ueberprueft, eine Pruefung mit der
     * uebergebenen Note bestanden wurde oder nicht.
     * @param note Die Note, welche ueberprueft wird.
     * @return Der boolean, welcher angibt, ob die Pruefung bestanden wurde.
     */
    public static boolean istBestanden(final double note) {
        return (note <= 4.0);
    }

    /**
     * Die Methode bessere nimmt zwei Noten vom Typ double und gibt die
     * bessere der beiden Noten zurueck.
     * @param note1 Die erste zu testende Note
     * @param note2 Die zweite zu testende Note
     * @return Die bessere der beiden Noten
     */
    public static double bessere(final double note1, final double note2) {
        if (note1 > note2) {
            return note2;
        } else {
            return note1;
        }
    }

    /**
     * Die Methode schlechtere nimmt zwei Noten vom Typ double und gibt die
     * schlechtere der beiden Noten zurueck.
     * @param note1 Die erste zu testende Note
     * @param note2 Die zweite zu testende Note
     * @return Die schlechtere der beiden Noten
     */
    public static double schlechtere(final double note1, final double note2) {
        if (note1 > note2) {
            return note1;
        } else {
            return note2;
        }
    }
}

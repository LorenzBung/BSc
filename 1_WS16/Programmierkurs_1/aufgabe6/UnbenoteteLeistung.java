package aufgabe6;

/**
 * Die Klasse UnbenoteteLeistung repraesentiert eine unbenotete Leistung.
 * @author Lorenz Bung
 * @version 10.01.16
 */
public final class UnbenoteteLeistung extends FachNote {

    /**
     * Gibt an, ob die Leistung bestanden ist oder nicht.
     */
    private boolean istBestanden;

    /**
     * Konstruktor fuer Objekte vom Typ UnbenoteteLeistung.
     * @param name Der Name des Fachs.
     * @param bestanden Gibt an, ob die Leistung bestanden wurde oder nicht.
     */
    public UnbenoteteLeistung(final String name, final boolean bestanden) {
        super(name);

        istBestanden = bestanden;
    }

    /* Überschreibe geerbte Methoden */

    /**
     * Gibt an, ob das Fach benotet ist.
     * @return Ob das Fach benotet ist.
     */
    @Override
    public boolean istBenotet() {
        return false;
    }

    /**
     * Gibt an, ob das Fach bestanden ist.
     * @return Ob das Fach bestanden ist.
     */
    @Override
    public boolean istBestanden() {
        return istBestanden;
    }
}

package aufgabe6;
import aufgabe4.Noten;

/**
 * Die Klasse BenoteteLeistung repraesentiert eine benotete Leistung.
 * @author Lorenz Bung
 * @version 10.01.16
 */
public final class BenoteteLeistung extends FachNote {

    /**
     * note speichert den Notenwert der Leistung.
     */
    private double note;

    /**
     * Konstruktor fuer Objekte vom Typ BenoteteLeistung.
     * @param name Der Name des Fachs.
     * @param notenString Die Note der Leistung.
     */
    public BenoteteLeistung(final String name, final String notenString) {
        super(name); // Aufruf des Konstruktors der Superklasse.
        if (Noten.istZulaessig(notenString)) {
            note = Noten.doubleNote(notenString);
        } else {
            throw new IllegalArgumentException("Note ist nicht zulaessig!");
        }
    }

    /* Ueberschreibe geerbte Methoden */

    /**
     * Gibt an, ob das Fach benotet ist.
     * @return Ob das Fach benotet ist.
     */
    @Override
    public boolean istBenotet() {
        return true;
    }

    /**
     * Gibt an, ob das Fach bestanden ist.
     * @return Ob das Fach bestanden ist.
     */
    @Override
    public boolean istBestanden() {
        return Noten.istBestanden(note);
    }

    /**
     * Gibt die Note der Leistung in numerischer Schreibweise zurueck.
     * @return Die Note der Leistung in numerischer Schreibweise.
     */
    @Override
    public String getNote() {
        return Double.toString(note);
    }

    /**
     * Gibt die Note der Leistung in Worten zurueck.
     * @return Die Note der Leistung in Worten.
     */
    @Override
    public String getNoteInWorten() {
        if (note < 1.5) {
            return "sehr gut";
        } else if (note < 2.5) {
            return "gut";
        } else if (note < 3.5) {
            return "befriedigend";
        } else if (note <= 4.0) {
            return "ausreichend";
        } else {
            return "nicht ausreichend";
        }
    }
}

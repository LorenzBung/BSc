//FachNote.java

package aufgabe5;
/**
 * FachNote eine Klasse zum beschreiben von Objekten in einer
 * verlinkten Liste, die Elemente darin beschreiben eine
 * Notenstatistik.
 */
public class FachNote {

    /**
     * Angabe des naechsten Elements in der Liste.
     */
    public /*final*/ FachNote naechste;
    /**
     * Angabe des Fachs des aktuellen Elements.
     */
    public final String fach;
    /**
     * Angabe der Note des aktuellen Elements.
     */
    public final double note;

    /**
     * Konstruktor fuer Objekte vom Typ FachNote.
     * @param fach Das Fach des aktuellen Elements.
     * @param note Die Note des aktuellen Elements.
     * @param naechste Das naechste Element in der Liste.
     */
    public FachNote(final String fach, final double note,
        final FachNote naechste) {
        this.fach = fach;
        this.note = note;
        this.naechste = naechste;
    }
}

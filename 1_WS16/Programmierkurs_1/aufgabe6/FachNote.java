// FachNote.java
package aufgabe6;

/**
 * FachNote kapselt ein Fach mit Note als Entit&auml;t.
 * @author H.Drachenfels
 * @version 10.8.2016
 */
public abstract class FachNote {
    private final String fach;

    /**
     * initialisiert die Fachnote mit einem Namen.
     * <p>
     * Wirft eine Ausnahme <tt>IllegalArgumentException</tt>, wenn das Fach
     * laut {@link Faecher#istZulaessig} nicht zul&auml;ssig ist.</p>
     * @param f der Fachname
     */
    protected FachNote(final String f) {
        if (!Faecher.istZulaessig(f)) {
            throw new IllegalArgumentException("unzulaessiges Fach " + f);
        }

        this.fach = f;
    }

    /**
     * liefert den Namen des Fachs.
     * @return den Fachnamen
     */
    public final String getFach() {
        return this.fach;
    }

    /**
     * liefert die Note des Fachs in numerischer Schreibweise.
     * <p>
     * Wenn {@link #istBenotet} <tt>true</tt> zur&uuml;ckgibt,
     * muss die Methode in der Unterklasse so &uuml;berschrieben werden,
     * dass sie die Note in numerischer Schreibweise liefert,
     * also Strings "1,0", "1,3" usw.
     * @return leerer String &quot;&quot;
     */
    public String getNote() {
        assert !istBenotet() : this.getClass().getName()
                               + " muss getNote() ueberschreiben";
        return "";
    }

    /**
     * liefert die Note des Fachs in Worten.
     * <p>
     * Wenn {@link #istBenotet} <tt>true</tt> zur&uuml;ckgibt,
     * muss die Methode in der Unterklasse &uuml;berschrieben werden.
     * Der geliefert String h&auml;ngt dann davon ab,
     * welches Notensystem die Unterklasse verwendet.
     * @return "bestanden", wenn {@link #istBestanden} true liefert,
     *         sonst "nicht bestanden".
     */
    public String getNoteInWorten() {
        assert !istBenotet() : this.getClass().getName()
                               + " muss getNoteInWorten() ueberschreiben";
        if (istBestanden()) {
            return "bestanden";
        }

        return "nicht bestanden";
    }

    /**
     * fragt ab, ob das Fach bestanden ist.
     * Wenn {@link #istBenotet} <tt>true</tt> zur&uuml;ckgibt,
     * h&auml;ngt es vom Notensystem der Unterklasse ab,
     * welche Noten als bestanden und welche als nicht bestanden gelten.
     * @return <tt>true</tt>, wenn bestanden, sonst <tt>false</tt>
     */
    public abstract boolean istBestanden();

    /**
     * fragt ab, ob das Fach benotet oder unbenotet ist.
     * <p>
     * Darf nur <tt>true</tt> liefern, wenn die Methoden
     * {@link #getNote} und {@link #getNoteInWorten}
     * so &uuml;berschreiben sind, dass sie eine Note liefern.</p>
     * @return <tt>true</tt>, wenn benotet, sonst <tt>false</tt>
     */
    public abstract boolean istBenotet();
}


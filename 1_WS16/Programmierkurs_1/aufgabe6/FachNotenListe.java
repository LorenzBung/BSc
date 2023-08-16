package aufgabe6;

import java.util.Iterator;
import java.util.NoSuchElementException;

/**
 * Klasse FachNotenListe, eine verkettete Liste fuer FachNoten.
 */
public final class FachNotenListe implements Iterable<FachNote> {

    private Element head;


    /**
     * Konstruktor fuer Objekte vom Typ FachNotenListe.
     */
    public FachNotenListe() {
        head = null;
    }

    /**
     * Ueberschreibe Methoden aus Iterator.
     */
    @Override
    public Iterator<FachNote> iterator() {
        return new Iterator<FachNote>() {
            private Element current = head;
            @Override
            public boolean hasNext() {
                return this.current != null;
            }
            @Override
            public FachNote next() {
                if (this.current == null) {
                    throw new NoSuchElementException();
                }
                Element e = this.current;
                this.current = this.current.next;
                return e.note;
            }
            @Override
            public void remove() {
                throw new UnsupportedOperationException();
            }
        };
    }

    /**
     * Klasse Element, welche Fachnoten referenziert.
     */
    private static final class Element {
        // Referenz auf das naechste Element
        private final Element next;
        // Die Fachnote selbst
        private final FachNote note;
        // Konstruktor fuer Elemente
        private Element(final FachNote notenWert, final Element e) {
            next = e;
            note = notenWert;
        }
    }

    /**
     * Eine Methode, um Elemente zur FachNotenListe hinzuzufuegen.
     * @param fachnote Die Note des Fachs.
     */
    public void einfuegen(final FachNote fachnote) {
        head = new Element(fachnote, head);
    }
}

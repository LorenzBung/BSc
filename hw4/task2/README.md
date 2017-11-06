# Homework hw4 task2

Die folgenden Informationen sollen helfen, sich schneller in die Materie des
'Timings' von Programmen einzuarbeiten:

- Das externe Crate time][] bietet mehr und vor allem einfachere Funktionalität
  als die Rust Standard-Bibliothek.
- Alle Betriebssysteme stellen eine Time API zu Verfügung. Es kann hilfreich
  sein zu verstehen, wie diese funktioniert. Daher beschäftigt sich das nächste
  Kapitel ausführlich mit dieser API.
- Das Modul [std::mem] aus der Standardbibliothek ist zur Lösung dieser Aufgabe
  sehr hilfreich.

[time]: https://docs.rs/time
[std::mem]: https://doc.rust-lang.org/std/mem/

- [Zeiten lesen in C](#zeiten-lesen-in-c)
    - [Datenstrukturen](#datenstrukturen)
        - [Zeit lesen](#zeit-lesen)
        - [Zeitvergleich: Differenzzeitmessung](#zeitvergleich-differenzzeitmessung)
          Differenzzeitmessung](#zeitvergleich-differenzzeitmessung)

## Zeiten lesen in C

Das folgende Kapitel muss zur Lösung von task2 nicht komplett verstanden werden.
Vielmehr soll es weitere Informationen liefern, wenn Ihnen gewisse
Funktionalitäten der Thematik 'Timing' unklar sind.

### Datenstrukturen

Betriebssysteme stellen Anwendungen Zeitgeber mit unterschiedlichen
Eigenschaften zur Verfügung, über die das Zeitverhalten kontrolliert wird. Diese
sind gekennzeichnet durch ihre

- Genauigkeit,
- die Zuverlässigkeit,
- den Bezugspunkt,
- die Darstellung und
- den maximalen Zeitbereich.

Das Betriebssystem repräsentiert Zeiten unter anderem mit den folgenden
Datentypen (Darstellung):

- clock_t: Timerticks.
- struct timeval: Zeit in Mikrosekunden-Auflösung.
- struct timespec: Zeit in Nanosekunden-Auflösung.
- struct tm: absolute Zeitangabe.

```c
struct timeval {

    time_t      tv_sec;     /* seconds */
    suseconds_t tv_usec;    /* microseconds */
};

struct timespec {
    time_t tv_sec; /* seconds */
    long tv_nsec; /* nanoseconds */
};

struct tm
{
    int tm_sec; /* seconds */
    int tm_min; /* minutes */
    int tm_hour; /* hours */
    int tm_mday; /* day of the month */
    int tm_mon; /* month */
    int tm_year; /* year */
    int tm_wday; /* day of the week */
    int tm_yday; /* day in the year */
    int tm_isdst; /* daylight saving time */
};
```

Die Strukturen struct timeval und struct timespec bestehen aus jeweils zwei
Variablen, die einmal den Sekundenanteil und einmal den Mikro- beziehungsweise
den Nanosekundenanteil repräsentieren. Die Darstellung erfolgt jeweils normiert.
Das bedeutet, dass der Mikro- oder Nanosekundenanteil immer kleiner als eine
volle Sekunde bleibt. Ein Zeitstempel von beispielsweise 1 Sekunde, 123456
Mikrosekunden ist gültig, 1 Sekunde, 1234567 Mikrosekunden ist ungültig. In
normierter Darstellung ergäbe sich 2 Sekunden, 234567 Mikrosekunden.

Die Darstellungs- beziehungsweise Repräsentationsform reflektiert auch den
darstellbaren Wertebereich. Da bei den dargestellten Datenstrukturen für den Typ
time\_t ein long eingesetzt wird, lassen sich auf einer 32-Bit Maschine rund 4
Milliarden Sekunden zählen, auf einer 64-Bit Maschine 2\^64 (mehr als 500
Milliarden Jahre).

Als Bezugspunkte haben sich die folgenden eingebürgert: *Start des Systems,
*Start eines Jobs und \*Start einer Epoche, beispielsweise "Christi Geburt" oder
der 1.1.1970 (Unix-Epoche). Dieser Bezugspunkt weist zudem noch eine örtliche
Komponente auf: Der Zeitpunkt 19:00 Uhr in Europa entspricht beispielsweise
einem anderen Zeitpunkt in den USA (minus sechs Stunden zur Ostküste).

Die Genauigkeit wird beeinflußt durch, die Taktung des Zeitgebers, deren
Schwankungen und durch Zeitsprünge.

Das Attribut Zuverlässigkeit eines Zeitgebers beschreibt dessen Verhalten bei
(bewußten) Schwankungen der Taktung und bei Zeitsprüngen: Ein Zeitgeber kann
beispielsweise der Systemuhr folgen (CLOCK\_REALTIME) oder unabhängig von
jeglicher Modifikation an der Systemzeit einfach weiterzählen
(CLOCK\_MONOTONIC). Die Posix-Realzeiterweiterung beziehungsweise Linux-Systeme
definieren hierzu folgende Clocks [Man-Page zu
clock\_gettime()](http://linux.die.net/man/3/clock_gettime):

- **CLOCK\_REALTIME**: Dieser Zeitgeber repräsentiert die systemweite, aktuelle
  Zeit. Er reagiert auf Zeitsprünge, sowohl vorwärts als auch rückwärts, die
  beispielsweise beim Aufwachen (Resume) nach einem Suspend (Schlafzustand des
  gesamten Systems) ausgelöst werden. Er reagiert ebenfalls auf unterschiedliche
  Taktungen, die beispielsweise durch NTP erfolgen. Dieser Zeitgeber liefert die
  Sekunden und Nanosekunden seit dem 1.1. 1970 UTC (Unixzeit) zurück.
- **CLOCK\_MONOTONIC**: Dieser Zeitgeber läuft entsprechend seiner Auflösung
  stets vorwärts, ohne dabei Zeitsprünge zu vollziehen. Er ist also unabhängig
  von der mit Superuserprivilegien zu verändernden Systemuhr. Allerdings
  reagiert dieser Zeitgeber auf Modifikationen der Taktung, die beispielsweise
  durch NTP erfolgen.
- **CLOCK\_MONOTONIC_RAW**: Dieser Zeitgeber ist linuxspezifisch. Er reagiert
  weder auf Zeitsprünge noch auf in Betrieb geänderte Taktungen (NTP).
- **CLOCK\_PROCESS\_CPUTIME_ID**: Dieser Zeitgeber erfasst die Verarbeitungszeit
  (Execution Time) des zugehörigen Prozesses. Das funktioniert aber nur
  zuverlässig auf Single-Core-Systemen beziehungsweise wenn sichergestellt
  werden kann, dass keine Prozessmigration stattfindet.
- **CLOCK\_THREAD\_CPUTIME\_ID**: Dieser Zeitgeber erfasst die Verarbeitungszeit
  (Execution Time) des zugehörigen Threads. Das funktioniert aber nur
  zuverlässig auf Single-Core-Systemen beziehungsweise wenn sichergestellt
  werden kann, dass keine Prozessmigration stattfindet.

Der maximale Zeitbereich schließlich ergibt sich durch die Auflösung des
Zeitgebers und die Bitbreite der Variablen:

```text
zeitbereich = auflösung * 2^bitbreite
```

#### Zeit lesen

Es gibt unterschiedliche Systemfunktionen, mit denen die aktuelle Zeit gelesen
werden kann. Favorisiert ist die Funktion *int clock\_gettime(clockid\_t
clk\_id, struct timespec * tp)*, die die Zeit seit dem 1.1.1970 (Unixzeit) als
Universal Time (Zeitzone UTC) zurückliefert (struct timespec). Konnte die
aktuelle Zeit gelesen werden, gibt die Funktion Null, ansonsten einen Fehlercode
zurück. Allerdings ist das Auslesen auf 32-Bit Systemen in so fern
problematisch, da der 32-Bit Zähler am 19. Januar 2038 überläuft.

```c
struct timespec {
        time_t   tv_sec;        /* seconds */
        long     tv_nsec;       /* nanoseconds */
};

```

```c
struct timespec timestamp;
    ...
    if (clock_gettime(CLOCK_MONOTONIC,&timestamp))
    {
        perror("timestamp");
        return -1;
    }
    printf("seconds: %ld, nanoseconds: %ld\n",
        timestamp.tv\_sec, timestamp.tv\_nsec);
```

Durch die Wahl der Clock CLOCK\_PROCESS\_CPUTIME\_ID beziehungsweise
CLOCK\_THREAD\_CPUTIME\_ID kann auch die Verarbeitungszeit ausgemessen werden
(Profiling).

Die Genauigkeit der zurückgelieferten Zeit kann mit Hilfe der Funktion
*clock\_getres(clockid\_t clk\_id, struct timespec * res)* ausgelesen werden.

Die Funktion *clock\_gettime()* ist nicht in der Standard-C-Bibliothek zu
finden, sondern in der Realzeit-Bibliothek librt. Daher ist bei der
Programmgenerierung diese Bibliothek hinzuzulinken (Parameter -lrt). Steht nur
die Standard-C-Bibliothek zur Verfügung, kann

- time\_t time(time\_t *t) oder auch
- int gettimeofday(struct timeval * tv, struct timezone * tz)

eingesetzt werden.

*time()* gibt die Sekunden zurück, die seit dem 1.1.1970 (UTC) vergangen sind.

```c
time_t now;
...
now = time(NULL);
```

*gettimeofday()* schreibt an die per tv übergebene Speicheradresse die Sekunden
und Mikrosekunden seit dem 1.1.1970. Das Argument tz wird typischerweise mit
NULL angegeben.

Liegen die Sekunden seit dem 1.1.1970 vor (timestamp.tv\_sec), können diese mit
Hilfe der Funktionen

- struct tm *localtime\_r(const time\_t *timep, struct tm *result) oder
- struct tm *gmtime\_r(const time\_t * timep, struct tm *result)

in die Struktur *struct tm* konvertiert werden.

```c
struct tm absolute_time;
if (localtime_r( timestamp.tv_sec, &absolute_time )==NULL)
{
    perror("localtime_r" );
    return -1;
}

printf("year: %d\n", absolute_time.tm_year);
```

Die Funktion *time\_t mktime(struct tm * tm)* konvertiert eine über die Struktur
struct tm gegebene Zeit in Sekunden seit dem 1.1.1970 (time\_t).

Mit Hilfe der Funktion *clock\_t times(struct tms \ buf)* lässt sich sowohl die
aktuelle Zeit zu einem letztlich nicht genau definierten Bezugspunkt, als auch
die Verarbeitungszeit (Execution-Time) des aufrufenden Prozesses bestimmen. Die
Zeiten werden als Timerticks (clock\_t) zurückgeliefert. Die zurückgelieferte
Verarbeitungszeit ist aufgeschlüsselt in die Anteile, die im Userland und die
Anteile, die im Kernel verbraucht wurden. Außerdem werden diese Anteile auch für
Kindprozesse gelistet.

```c
#include <stdio.h>
#include <sys/times.h>
#include <unistd.h>
#include <time.h>

int main( int argc, char **argv, char **envp )
{
    struct tms exec_time;
    clock_t act_time;
    long ticks_per_second;
    long tickduration_in_ms;

    ticks_per_second = sysconf(_SC_CLK_TCK);
    tickduration_in_ms = 1000/ticks_per_second;

    act_time = times( &exec_time );
    printf("actual time (in ms): %ld\n", act_time*tickduration_in_ms);
    printf("execution time (in ms): %ld\n",
            (exec_time.tms_utime+exec_time.tms_stime)*tickduration_in_ms);

    return 0;

}
```

Sehr genaue Zeiten lassen sich erfassen, falls der eingesetzte Prozessor einen
Zähler besitzt, der mit der Taktfrequenz des Systems getaktet wird. Bei einer
x86-Architektur (PC) heißt dieser Zähler Time Stamp Counter (TSC). Der TSC kann
auch von einer normalen Applikation ausgelesen werden, allerdings muss
sichergestellt sein, dass sich die Taktfrequenz zwischen zwei Messungen ändert.
Alternativ kann man sich vom Betriebssystem über die Taktänderung informieren
lassen.

#### Zeitvergleich: Differenzzeitmessung

Zwei Absolutzeiten (struct tm) werden am einfachsten über deren Repräsentation
in Sekunden verglichen. Die Umwandlung erfolgt über die Funktion (time\_t
mktime(struct tm \*tm)). Allerdings ist dabei zu beachten, dass es auf einem
32-Bit System am 19. Januar 2038 zu einem Überlauf kommt. Wird einer der beiden
Zeitstempel vor dem 19. Januar 2038 genommen, der andere danach, kommt es zu
einem falschen Ergebnis, wenn nur die beiden Werte per "\<" beziehungsweise "\>"
verglichen werden.

Das ist ein generelles Problem und kann dann gelöst werden, wenn sichergestellt
ist, dass die zu vergleichenden Zeiten nicht weiter als die Hälfte des gesamten
Zeitbereiches auseinanderliegen. In diesem Fall lassen sich die Makros
einsetzen, die im Linux-Kernel für den Vergleich zweier Zeiten eingesetzt
werden. Das Makro time\_after(a,b) liefert true zurück, falls es sich bei a um
eine spätere Zeit als b handelt. Das Makro time\_after\_eq(a,b) liefert true
zurück, falls es sich bei a um eine spätere Zeit oder um die gleiche Zeit
handelt, wie b handelt. Die Zeitstempel a und b müssen beide vom Typ unsigned
long sein. Natürlich können die Makros auch auf andere Datentypen angepasst
werden

[HEADERDATEI linux/jiffies.h].

```c
#define time_after(a,b)         \
        (typecheck(unsigned long, a) && \
         typecheck(unsigned long, b) && \
         ((long)(b) - (long)(a) < 0))

#define time_before(a,b)        time_after(b,a)

#define time_after_eq(a,b)      \
        (typecheck(unsigned long, a) && \
         typecheck(unsigned long, b) && \
         ((long)(a) - (long)(b) \>= 0))

#define time_before_eq(a,b)     time_after_eq(b,a)
```

Beim Benchmarking ist es häufig notwendig eine Zeitdauer zu messen, Zeitpunkte
zu erfassen oder eine definierte Zeit verstreichen zu lassen. Dabei sind
folgende Aspekte zu beachten:

- Die Genauigkeit der eingesetzten Zeitgeber,
- die maximalen Zeitdifferenzen,
- Schwankungen der Zeitbasis, beispielsweise durch Schlafzustände,
- Modifikationen an der Zeitbasis des eingesetzten Rechnersystems (Zeitsprünge)
  und
- die Ortsabhängigkeit absoluter Zeitpunkte.

Zur Bestimmung einer Zeitdauer verwendet man häufig eine Differenzzeitmessung.
Dabei wird vor und nach der auszumessenden Aktion jeweils ein Zeitstempel
genommen. Die Zeitdauer ergibt sich aus der Differenz dieser beiden Zeitstempel.

Dies ist allerdings nicht immer ganz einfach. Liegt der Zeitstempel
beispielsweise in Form der Datenstruktur struct timeval (als Ergebnis der
Funktion *gettimeofday()*) vor, müssen die Sekunden zunächst getrennt von den
Mikrosekunden subtrahiert werden. Ist der Mikrosekundenanteil negativ, muss der
Sekundenanteil um eins erniedrigt und der Mikrosekundenanteil korrigiert werden.
Dazu wird auf die Anzahl der Mikrosekunden pro Sekunde (also eine Million) der
negative Mikrosekundenanteil addiert.

```c
#define MICROSECONDS\_PER\_SECOND 1000000

struct timespec * diff_time( struct timeval before, struct timeval after,
    struct timeval *result )
{
    if (result==NULL)
        return NULL;

    result->tv_sec = after.tv_sec - before.tv_sec;
    result->tv_usec= after.tv_usec- before.tv_usec;

    if (result->tv_usec<0) {
        result->tv_sec--;
        /* result->tv_usec is negative, therefore we use "+" */
        result->tv_usec = MICROSECONDS_PER_SECOND+result->tv_usec;
    }
    return result;

}
```

Für Zeitstempel vom Typ struct timeval gilt entsprechendes. Anstelle der
MICROSECONDS\_PER\_SECOND sind NANOSECONDS\_PER\_SECOND einzusetzen. Sind die
Zeitstempel vorzeichenlos, sieht die Rechnung für den Sekundenanteil etwas
komplizierter aus. Das soll hier aber nicht weiter vertieft werden.

Etwas einfacher ist die Differenzbildung, wenn aus der Datenstruktur eine
einzelne Variable mit der gewünschten Auflösung, beispielsweise Mikrosekunden,
generiert wird. Im Fall von struct timeval wird dazu der Sekundenanteil mit
einer Million multipliziert und der Mikrosekundenanteil aufaddiert. Bei der
Multiplikation können natürlich Informationen verloren gehen, allerdings geht
der gleiche Informationsgehalt auch beim zweiten Zeitstempel verloren. Für die
Differenzbildung ist das damit nicht relevant, solange der zu messende zeitliche
Abstand kleiner als 1000 Sekunden ist und es während der Messung keinen Überlauf
beim Sekundenanteil gibt.

```c
time_in_usec=((nachher.tv_sec*1000000)+nachher.tv_usec)-
    ((vorher.tv_sec*1000000)+vorher.tv_usec);
```

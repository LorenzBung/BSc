# Task 1 - Laufzeitanalyse

Bei unseren Tests au der Labshell bekamen wir folgende Werte:

```text
cargo run --release -- 42 12 timings
[...]
(Duration 0s / 0ms / 239us)
```

```text
cargo run --release -- 42 123 timings
[...]
(Duration 0s / 8ms / 8027us)
```

```text
cargo run --release -- 42 1234 timings
[...]
(Duration 0s / 95ms / 95245us)
```

```text
cargo run --release -- 43 12345 timings
[...]
(Duration 0s / 484ms / 484376us)
```

```text
cargo run --release -- 43 123456 timings
[...]
(Duration 13s / 13711ms / 13711554us)
```

Diese Messwerte lassen auf ein exponentielles Wachstum der Laufzeit
pro hinzukommender Stelle schließen.
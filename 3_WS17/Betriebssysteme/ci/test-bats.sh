#!/usr/bin/env bash

# This file needs to be run from the git root directory!

# Exit script on the first error
set -o errexit -o nounset

current_hw=$(find . -type f | grep "hw./task./Cargo\.toml" | sort | tail -n1 | cut -d / -f -2)

while IFS= read -r -d '' taskdir; do

  echo ""
  echo "=== Suche bats File in $taskdir"
  for batsfile in $taskdir/tests/*.bats; do
    if [ -e "$batsfile" ]; then
      echo "=== bats File gefunden"
      echo ""
      echo "=== Führe Lösung von Aufgabe in $taskdir aus"
      manifest="$taskdir/Cargo.toml"
      if [ -e "$manifest" ]; then
        echo "=== Cargo-Manifest gefunden in '$manifest' -> Cargo-Modus"
        cargo run --manifest-path "$manifest" || true
      elif [ "$(find "$taskdir" -maxdepth 1 -type f -name '*.rs' | wc -l)" -ne 0 ]; then
        echo "=== Sourcedatei(en) gefunden -> rustc-Modus"
        for srcfile in $taskdir/*.rs; do
          echo "=== Kompiliere '$srcfile'..."
          rustc "$srcfile"
        done
      else
        echo ""
        echo "!!! Falsch konfigurierter Aufgabenordner oder ungelöste Aufgabe"
        echo "!!! Bitte .rs-Dateien in '$taskdir' ablegen"
        echo "!!! Oder ein Cargo-Projekt mit 'cargo init' darin erzeugen"
        echo "!!! Alternativ den Ordner löschen"
        exit 1
      fi
      echo "=== bats File gefunden, teste output"
      bats "$batsfile"
    else
      echo "=== keine output Tests via bats ausgeführt"
    fi
  done

done < <(find "$current_hw" -mindepth 1 -maxdepth 1 -type d -name 'task*' -print0)

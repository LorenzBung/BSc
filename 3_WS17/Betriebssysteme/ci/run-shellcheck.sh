#!/usr/bin/env bash

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"

# run shellcheck on all shell scripts
find "$ROOT_DIR" -type f -name '*.sh' -print0 | xargs -n 1 -0 shellcheck --color

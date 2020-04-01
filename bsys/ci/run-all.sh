#!/usr/bin/env bash

# Exit script on the first error
set -o errexit -o nounset

MY_PATH="$(dirname "$0")"

# basic style check
"$MY_PATH/check-basic-style.py"

# rustfmt style check
"$MY_PATH/rustfmt.sh"

# check that everything compiles and all tests pass
"$MY_PATH/test-rust.sh"

# after compiles run bats tests
"$MY_PATH/test-bats.sh"

# file existence
echo "=== Checking for Missing Files ======================================="
"$MY_PATH/check-files.py"

echo "++++++++++++++++++++++++++++++++++++++++++++++++++++"
echo "+              Everything is fine!                 +"
echo "++++++++++++++++++++++++++++++++++++++++++++++++++++"

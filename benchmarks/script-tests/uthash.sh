#!/bin/bash

set -e

script_dir="$(dirname "$0")"
. "$script_dir/prepare"

for n in {1..96}; do
  cp "$2/$name/target/$dir/tests_test${n}_main" "$1/$name/tests/test${n}"
done

cd "$1/$name/tests"
./do_tests

#!/bin/bash

set -e

script_dir="$(dirname "$0")"
. "$script_dir/prepare"

cp "$2/$name/target/$dir/src_test_torture_main" "$1/$name/src/test/torture_test"

cd "$1/$name/src/test"
./tst ./res -c

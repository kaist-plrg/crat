#!/bin/bash

set -e

script_dir="$(dirname "$0")"
. "$script_dir/prepare"

cp "$2/$name/target/$dir/src_main_main" "$1/$name/make"

cd "$1/$name/tests"
ulimit -n 512
perl ./run_make_tests.pl -srcdir .. -make ../make

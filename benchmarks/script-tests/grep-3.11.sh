#!/bin/bash

set -e

script_dir="$(dirname "$0")"
. "$script_dir/prepare"

mkdir "$1/$name/src"
cp "$2/$name/target/$dir/src_grep_main" "$1/$name/src/grep"

cd "$1/$name/tests"
make check-TESTS

#!/bin/bash

set -e

script_dir="$(dirname "$0")"
. "$script_dir/prepare"

mkdir "$1/$name/src"
cp "$2/$name/target/$dir/src_cmp_main" "$1/$name/src/cmp"
cp "$2/$name/target/$dir/src_diff3_main" "$1/$name/src/diff3"
cp "$2/$name/target/$dir/src_diff_main" "$1/$name/src/diff"
cp "$2/$name/target/$dir/src_sdiff_main" "$1/$name/src/sdiff"

cd "$1/$name/tests"
make check-TESTS

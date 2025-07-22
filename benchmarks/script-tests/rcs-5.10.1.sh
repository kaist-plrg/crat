#!/bin/bash

set -e

script_dir="$(dirname "$0")"
. "$script_dir/prepare"

cp "$2/$name/target/$dir/src_supr_main" "$1/$name/src/rcs"
cp "$2/$name/target/$dir/src_ident_main" "$1/$name/src/ident"
cp "$2/$name/target/$dir/src_merge_main" "$1/$name/src/merge"

cd "$1/$name/tests"
make check-TESTS

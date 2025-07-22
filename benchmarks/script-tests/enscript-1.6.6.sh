#!/bin/bash

set -e

script_dir="$(dirname "$0")"
. "$script_dir/prepare"

cp "$2/$name/target/$dir/src_main_main" "$1/$name/src/enscript"
cp "$2/$name/target/$dir/states_main_main" "$1/$name/states/states"

cd "$1/$name"
make -C src/tests check-TESTS
make -C states/tests check-TESTS

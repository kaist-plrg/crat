#!/bin/bash

set -e

script_dir="$(dirname "$0")"
. "$script_dir/prepare"

mkdir "$1/$name/src"
cp "$2/$name/target/$dir/src_main_main" "$1/$name/src/wget"

cd "$1/$name"
make -C fuzz check-TESTS
make -C tests check-TESTS
make -C testenv check-TESTS

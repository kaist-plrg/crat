#!/bin/bash

set -e

script_dir="$(dirname "$0")"
. "$script_dir/prepare"

mkdir "$1/$name/src"
cp "$2/$name/target/$dir/src_test_all_main" "$1/$name/src/test_all"

cd "$1/$name/src"
./test_all

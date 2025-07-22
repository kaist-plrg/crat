#!/bin/bash

set -e

script_dir="$(dirname "$0")"
. "$script_dir/prepare"

cp "$2/$name/target/$dir/cava_main" "$1/$name/cava"

cd "$1/$name"
./run_all_tests.sh

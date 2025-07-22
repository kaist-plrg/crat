#!/bin/bash

set -e

script_dir="$(dirname "$0")"
. "$script_dir/prepare"

mkdir -p "$1/$name/build/Release/bin"
cp "$2/$name/target/$dir/cli_main_main" "$1/$name/build/Release/bin/pocket"

cd "$1/$name/scripts"
python3 run_tests.py

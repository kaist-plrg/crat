#!/bin/bash

set -e

script_dir="$(dirname "$0")"
. "$script_dir/prepare"

cp "$2/$name/target/$dir/main_main" "$1/$name/ed"

cd "$1/$name"
./testsuite/check.sh ./testsuite 1.19

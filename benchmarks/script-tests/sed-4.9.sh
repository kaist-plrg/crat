#!/bin/bash

set -e

script_dir="$(dirname "$0")"
. "$script_dir/prepare"

mkdir "$1/$name/sed"
cp "$2/$name/target/$dir/sed_sed_main" "$1/$name/sed/sed"

cd "$1/$name"
make check-TESTS

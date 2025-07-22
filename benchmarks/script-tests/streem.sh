#!/bin/bash

set -e

script_dir="$(dirname "$0")"
. "$script_dir/prepare"

mkdir "$1/$name/bin"
cp "$2/$name/target/$dir/src_main_main" "$1/$name/bin/streem"

cd "$1/$name"
./test.sh

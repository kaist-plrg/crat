#!/bin/bash

set -e

script_dir="$(dirname "$0")"
. "$script_dir/prepare"

mkdir "$1/$name/src"
cp "$2/$name/target/$dir/src_indent_main" "$1/$name/src/indent"

cd "$1/$name/regression"
./TEST

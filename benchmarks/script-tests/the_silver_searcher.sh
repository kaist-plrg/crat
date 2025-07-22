#!/bin/bash

set -e

script_dir="$(dirname "$0")"
. "$script_dir/prepare"

cp "$2/$name/target/$dir/src_main_main" "$1/$name/ag"

cd "$1/$name"
cram -v tests/*.t

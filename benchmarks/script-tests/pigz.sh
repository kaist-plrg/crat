#!/bin/bash

set -e

script_dir="$(dirname "$0")"
. "$script_dir/prepare"

cp "$2/$name/target/$dir/pigz_main" "$1/$name/pigz"
cp "$2/$name/target/$dir/pigz_main" "$1/$name/unpigz"

cd "$1/$name"
./test.sh

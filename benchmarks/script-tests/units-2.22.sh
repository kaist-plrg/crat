#!/bin/bash

set -e

script_dir="$(dirname "$0")"
. "$script_dir/prepare"

cp "$2/$name/target/$dir/units_main" "$1/$name/units"

cd "$1/$name"
./test.sh

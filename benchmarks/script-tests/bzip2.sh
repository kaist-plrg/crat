#!/bin/bash

set -e

script_dir="$(dirname "$0")"
. "$script_dir/prepare"

cp "$2/$name/target/$dir/bzip2_main" "$1/$name/bzip2"

cd "$1/$name"
./test.sh

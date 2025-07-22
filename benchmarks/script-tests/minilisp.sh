#!/bin/bash

set -e

script_dir="$(dirname "$0")"
. "$script_dir/prepare"

cp "$2/$name/target/$dir/minilisp_main" "$1/$name/minilisp"

cd "$1/$name"
./test.sh

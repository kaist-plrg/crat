#!/bin/bash

set -e

script_dir="$(dirname "$0")"
. "$script_dir/prepare"

cp "$2/$name/target/$dir/dump1090_main" "$1/$name/dump1090"

cd "$1/$name"
diff -s result <(./dump1090 --ifile testfiles/modes1.bin)

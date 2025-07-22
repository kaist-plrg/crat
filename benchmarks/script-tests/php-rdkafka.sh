#!/bin/bash

set -e

script_dir="$(dirname "$0")"
. "$script_dir/prepare"

mkdir "$1/$name/modules"
cp "$2/$name/target/$dir/libc2rust_out.so" "$1/$name/modules/rdkafka.so"

cd "$1/$name"
./test.sh

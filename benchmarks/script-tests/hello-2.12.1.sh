#!/bin/bash

set -e

script_dir="$(dirname "$0")"
. "$script_dir/prepare"

cp "$2/$name/target/$dir/src_hello_main" "$1/$name/hello"

cd "$1/$name"
make check-TESTS

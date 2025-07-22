#!/bin/bash

set -e

script_dir="$(dirname "$0")"
. "$script_dir/prepare"

cp "$2/$name/target/$dir/gzip_main" "$1/$name/gzip"

cd "$1/$name/tests"
make check-TESTS

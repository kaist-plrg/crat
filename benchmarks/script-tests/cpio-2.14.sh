#!/bin/bash

set -e

script_dir="$(dirname "$0")"
. "$script_dir/prepare"

mkdir "$1/$name/src"
cp "$2/$name/target/$dir/src_main_main" "$1/$name/src/cpio"

cd "$1/$name/tests"
PATH="`pwd`/../src:`pwd`:$PATH" ./testsuite

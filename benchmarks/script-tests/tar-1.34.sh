#!/bin/bash

set -e

RUSTFLAGS='-Awarnings -Zon-broken-pipe=inherit'

script_dir="$(dirname "$0")"
. "$script_dir/prepare"

mkdir "$1/$name/src"
cp "$2/$name/target/$dir/src_tar_main" "$1/$name/src/tar"

cd "$1/$name/tests"
PATH="`pwd`/../src:$PATH" ./testsuite

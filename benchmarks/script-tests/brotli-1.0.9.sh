#!/bin/bash

set -e

script_dir="$(dirname "$0")"
. "$script_dir/prepare"

mkdir -p "$1/$name/bin/tmp"
cp "$2/$name/target/$dir/c_tools_brotli_main" "$1/$name/bin/brotli"

cd "$1/$name"
tests/roundtrip_test.sh

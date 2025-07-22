#!/bin/bash

set -e

script_dir="$(dirname "$0")"
. "$script_dir/prepare"

mkdir "$1/$name/find"
mkdir "$1/$name/xargs"
cp "$2/$name/target/$dir/find_ftsfind_main" "$1/$name/find/find"
cp "$2/$name/target/$dir/locate_frcode_main" "$1/$name/locate/frcode"
cp "$2/$name/target/$dir/locate_locate_main" "$1/$name/locate/locate"
cp "$2/$name/target/$dir/xargs_xargs_main" "$1/$name/xargs/xargs"

cd "$1/$name"
PATH="`pwd`/find:`pwd`/locate:`pwd`/xargs:$PATH" make check-TESTS

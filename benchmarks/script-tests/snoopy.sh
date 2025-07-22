#!/bin/bash

set -e

script_dir="$(dirname "$0")"
. "$script_dir/prepare"

mkdir -p "$1/$name/src/cli"
cp "$2/$name/target/$dir/src_cli_snoopy_main" "$1/$name/src/cli/snoopy"
cp "$2/$name/target/$dir/tests_bin_snoopy_test_main" "$1/$name/tests/bin/snoopy-test"
cp "$2/$name/target/$dir/tests_bin_spaceparent_main" "$1/$name/tests/bin/spaceparent"
cp "$2/$name/target/$dir/tests_bin_spaceparent_main" "$1/$name/tests/bin/space parent"

make -C $1/$name/tests/cli check-TESTS
make -C $1/$name/tests/configfile check-TESTS
make -C $1/$name/tests/datasource check-TESTS
make -C $1/$name/tests/filter check-TESTS
make -C $1/$name/tests/general check-TESTS
make -C $1/$name/tests/message check-TESTS
make -C $1/$name/tests/output check-TESTS
make -C $1/$name/tests/threads check-TESTS
make -C $1/$name/tests/unit check-TESTS
make -C $1/$name/tests/combined check-TESTS

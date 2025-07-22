#!/bin/bash

set -e

script_dir="$(dirname "$0")"
. "$script_dir/prepare"

cp "$2/$name/target/$dir/tests_src_api_main" "$1/$name/tests/test_api"
cp "$2/$name/target/$dir/tests_src_conc_increment_main" "$1/$name/tests/test_conc_increment"
cp "$2/$name/target/$dir/tests_src_nonzero_heap_stack_main" "$1/$name/tests/test_nonzero_heap_stack"
cp "$2/$name/target/$dir/tests_src_pause_resume_main" "$1/$name/tests/test_pause_resume"
cp "$2/$name/target/$dir/tests_src_wait_main" "$1/$name/tests/test_wait"

cd "$1/$name/tests"
./api.sh
./heap_stack_garbage.sh
./pause_resume.sh
./threadpool.sh
./wait.sh

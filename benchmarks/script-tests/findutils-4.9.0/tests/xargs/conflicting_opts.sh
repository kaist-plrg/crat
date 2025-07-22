#!/bin/sh
# Test mutually exclusive options --max-args, --max-lines, --replace, and
# their short-option aliases (-L, -l -I -i -n).

# Copyright (C) 2021-2022 Free Software Foundation, Inc.

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.

# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.

# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

. "${srcdir=.}/tests/init.sh"; fu_path_prepend_
print_ver_ xargs

# The idea behind: when using the mutually exclusive options, the last one
# specified overrides the values of the previous ones.  Or to put it in
# other words: the resulting output (and exit code btw.) is the same as if
# the earlier, conflicting options had not been passed - apart from the
# error diagnostic for every overridden option.

# Create some input file.
cat <<\EOF > in || framework_failure_
a b c
d e f
g h i
EOF

WARNFMT='xargs: warning: options %s and %s are mutually exclusive, ignoring previous %s value'

str4opt() {
  case "$1" in
  -n*|--max-args*) echo '--max-args/-n';;
  -l*|--max-lines*) echo '--max-lines/-l';;
  -i|-I*|--replace*) echo '--replace/-I/-i';;
  esac
}
lopt4opt() {
  case "$1" in
  -n*|--max-args*) echo '--max-args';;
  -l*|--max-lines*) echo '--max-lines';;
  -i|-I*|--replace*) echo '--replace';;
  esac
}

gen_err() {  # see warn_mutually_exclusive in xargs.c.
  s="$(str4opt "$1")" || framework_failure_
  l="$(lopt4opt "$2")" || framework_failure_

  printf "$WARNFMT\n" "$l" "$s" "$l"
}

# test_xargs a test case, using $1 as the last option which should finally override
# any other of the mutually exclusive options.
# Example: test_xargs -n2 OTHER_OPTION
# This will invoke 'xargs' with that option as the last one:
#   xargs OTHER_OPTION -n2
test_xargs() {
  rm -f out err err2 expout experr || framework_failure_

  # Generate the expected output file for the option in "$1".
  # test_xargs xargs with that option only; stderr should be empty
  # and stdout of this test_xargs is the reference for the actual test.
  last_opt="$1" \
    && shift 1 \
    && xargs "$last_opt" < in > expout 2>err \
    && compare /dev/null err \
    || framework_failure_

  # Generate the expected error diagnostic for the mutually exclusive option
  # passed earlier.  No diagnostic if the previous mutually-exclusive option
  # is the same as the current one.
  prev_excl_option=  # variable for previous mutually-exclusive option.
  for o in "$@" "$last_opt"; do
    lopt="$(lopt4opt "$o")" || framework_failure_
    case $o in -n*|--max-args*|-l*|--max-lines*|-i|-I*|--replace*)
      test "$prev_excl_option" != '' \
        && test "$prev_excl_option" != "$lopt" \
        && gen_err "$o" "$prev_excl_option"
      prev_excl_option="$lopt"
      ;;
    esac
  done > experr

  # test_xargs the test.
  xargs "$@" "$last_opt" < in > out 2>err || fail=1
  sed 's/^.*\(xargs:\)/&/' err > err2 || framework_failure_
  compare expout out || fail=1
  compare experr err2 || fail=1
}

# test_xargs the tests for --max-args as last option.
for last_opt in -n2 --max-args=2; do
  for other_opt in --max-lines=2 -l2 --replace='REPL' -I'REPL' -i; do
    test_xargs $last_opt $other_opt
  done
done

# test_xargs the tests for --max-line as last option.
for last_opt in -l2 --max-lines=2; do
  for other_opt in --max-args=2 -n2 --replace='REPL' -I'REPL' -i; do
    test_xargs $last_opt $other_opt
  done
done

# test_xargs the tests for --replace as last option.
for last_opt in --replace='REPL' -I'REPL' -i; do
  for other_opt in --max-args=2 -n2 -l2 --max-lines=2; do
    test_xargs $last_opt $other_opt
  done
done

# Finally do some multi-option tests.
# Test 'xargs -l2 -l3 -l4 -i -n2':
# diagnostic for -l once, then for -i being overridden.
test_xargs -n2   -l2 -l3 -l4 -i

# Test 'xargs -n2 -i -l4 -l5': diagnostic for -n, then once for -i.
test_xargs -l5   -n2 -i -l4

# Test 'xargs -l2 --max-lines=3 --replace=4 -I5 --max-args=6 -n7 -n1':
# Diagnostic for the change from MAX-LINES to REPLACE,
# then from REPLACE to MAX-ARGS.
test_xargs -n1 -l2 --max-lines=3 --replace=4 -I5 --max-args=6 -n7

# Test 'xargs --replace=REPL -n1': verify that it doesn't emit the warning.
# See discussion at: https://sv.gnu.org/patch/?1500
xargs -I_ echo 'a._.b' < in > expout 2>err \
  && compare /dev/null err \
  || framework_failure_

xargs -I_ -n1 echo 'a._.b' < in > out 2>err || fail=1
compare expout out || fail=1
compare /dev/null err || fail=1

Exit $fail

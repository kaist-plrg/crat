#!/bin/sh
# This test verifies that find does not leak a file descriptor for the working
# directory specified by the -execdir option.
# See Savannah bug #34976.

# Copyright (C) 2013-2022 Free Software Foundation, Inc.

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
print_ver_ find

# seq is not required by POSIX, so we have manual lists of number here instead.
three_to_thirty_five="3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35"
three_to_hundred="3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64 65 66 67 68 69 70 71 72 73 74 75 76 77 78 79 80 81 82 83 84 85 86 87 88 89 90 91 92 93 94 95 96 97 98 99 100"

# Test if restricting the number of file descriptors via ulimit -n works.
# We always try to open 33 files (but the ulimit -n value changes).
test_ulimit() {
  l="$1"  # limit to use
  (
    ulimit -n "$l" || exit 1
    for i in ${three_to_thirty_five} ; do
      printf "exec %d> /dev/null || exit 1\n" $i
    done | sh -x ;
  ) 2>/dev/null
}
# Opening 33 files with a limit of 40 should work.
test_ulimit 40 || { echo "SKIP: ulimit does not work (case 1)" >&2; exit 0 ; }
# Opening 33 files with a limit of 20 should fail.
test_ulimit 20 && { echo "SKIP: ulimit does not work (case 2)" >&2; exit 0 ; }

# Create test files, each 98 in the directories ".", "one" and "two".
make_test_data() {
  mkdir one two || return 1
  for i in ${three_to_hundred} ; do
    # We don't quote the RHS here because we actually want to create 3 files.
    touch $(printf './%03d one/%03d two/%03d ' $i $i $i) \
      || return 1
  done
}

# Create some test files.
make_test_data \
  || framework_failure_ "failed to set up the test"

( ulimit -n 30 && \
  find . -type f -execdir cat '{}' \; >/dev/null; \
) \
  || { echo "Option -execdir leaks file descriptors" >&2 ; \
       fail=1 ; }

Exit $fail

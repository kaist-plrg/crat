#!/bin/sh
# This test verifies that find does not have excessive memory consumption
# even for large directories.
# See Savannah bug #34079.

# Copyright (C) 2011-2022 Free Software Foundation, Inc.

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

# Mark as expensive.
expensive_

NFILES=200000

# Require seq(1) for this test - which may not be available
# on some systems, e.g on some *BSDs.
seq -f "_%04g" 0 2 >/dev/null 2>&1 \
  || skip_ "required utility 'seq' missing"

# Get the number of free inodes on the file system of the given file/directory.
get_ifree_() {
  d="$1"
  # Try GNU coreutils' stat.
  stat --format='%d' -f -- "$d" 2>/dev/null \
    && return 0

  # Fall back to parsing 'df -i' output.
  df -i -- "$d" \
  | awk '
      NR == 1 {  # Find ifree column.
        ifree = -1;
        for (i=1; i<=NF; i++) {
          n=tolower($i);
          if(n=="ifree" || n=="iavail") {
            ifree=i;
          }
        };
        if (ifree<=0) {
          print "failed to determine IFREE column in header: ", $0 | "cat 1>&2";
          exit 1;
        }
        next;
      }
    { print $ifree }
  ' \
  | grep .
}

# Skip early if we know that there are too few free inodes.
# Require some slack.
free_inodes=$(get_ifree_ '.') \
  && test 0 -lt $free_inodes \
  && min_free_inodes=$(expr 12 \* ${NFILES} / 10) \
  && { test $min_free_inodes -lt $free_inodes \
         || skip_ "too few free inodes on '.': $free_inodes;" \
                  "this test requires at least $min_free_inodes"; }

# Create directory with many entries.
mkdir dir && cd dir || framework_failure_
seq ${NFILES} | xargs touch || framework_failure_
cd ..

# Create a small directory as reference to determine lower ulimit.
mkdir dir2 && touch dir2/a dir2/b dir2/c || framework_failure_

# Determine memory consumption for the trivial case.
vm="$(get_min_ulimit_v_ find dir2 -fprint dummy)" \
  || skip_ "this shell lacks ulimit support"

# Allow 35MiB more memory than above.
( ulimit -v $(($vm + 35000)) && find dir >/dev/null ) \
  || { echo "memory consumption is too high" >&2; fail=1; }

Exit $fail

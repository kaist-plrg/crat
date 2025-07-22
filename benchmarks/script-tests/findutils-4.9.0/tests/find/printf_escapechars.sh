#!/bin/sh
# Test -printf with octal and letter escapes.

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

# Check for working od(1).
echo test | od -c >/dev/null \
  || skip_ "no working 'od -c' found"

# Create file with expected output interpreted as octal; use:
#   $ printf '%s'   \
#       $'OCTAL1: \1\n' \
#       $'OCTAL2: \02\n' \
#       $'OCTAL3: \003\n' \
#       $'OCTAL4: \0044\n' \
#       $'OCTAL8: \0028\n' \
#       $'BEL: \a\n' \
#       $'CR: \r\n' \
#       $'FF: \f\n' \
#       $'TAB: \t\n' \
#       $'VTAB: \v\n' \
#       $'BS: \b\n' \
#       $'BACKSLASH: \\\n' \
#       $'UNKNOWN: \z\n' \
#       | od -t o1
cat <<_EOD_ > expout || framework_failure_
0000000 117 103 124 101 114 061 072 040 001 012 117 103 124 101 114 062
0000020 072 040 002 012 117 103 124 101 114 063 072 040 003 012 117 103
0000040 124 101 114 064 072 040 004 064 012 117 103 124 101 114 070 072
0000060 040 002 070 012 102 105 114 072 040 007 012 103 122 072 040 015
0000100 012 106 106 072 040 014 012 124 101 102 072 040 011 012 126 124
0000120 101 102 072 040 013 012 102 123 072 040 010 012 102 101 103 113
0000140 123 114 101 123 110 072 040 134 012 125 116 113 116 117 127 116
0000160 072 040 134 172 012
0000165
_EOD_

# Prepare expected stderr.
echo "warning: unrecognized escape" > experr || framework_failure_

find . -maxdepth 0 \
  -printf 'OCTAL1: \1\n' \
  -printf 'OCTAL2: \02\n' \
  -printf 'OCTAL3: \003\n' \
  -printf 'OCTAL4: \0044\n' \
  -printf 'OCTAL8: \0028\n' \
  -printf 'BEL: \a\n' \
  -printf 'CR: \r\n' \
  -printf 'FF: \f\n' \
  -printf 'TAB: \t\n' \
  -printf 'VTAB: \v\n' \
  -printf 'BS: \b\n' \
  -printf 'BACKSLASH: \\\n' \
  -printf 'UNKNOWN: \z\n' \
  > out 2> err || fail=1

# Some 'od' implementations (e.g. on the *BSDs) produce different indentation
# and trailing spaces, therefore squeeze the former and remove the latter.
od -t o1 < out | sed 's/  */ /g; s/ *$//;' > out2 || framework_failure_
compare expout out2 || fail=1

sed 's/^.*\(warning: unrecognized escape\) .*$/\1/' err > err2 \
  || framework_failure_
compare experr err2 || fail=1

Exit $fail

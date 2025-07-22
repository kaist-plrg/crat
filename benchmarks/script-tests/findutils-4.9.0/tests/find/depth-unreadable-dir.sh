#!/bin/sh
# find -depth: ensure to output an unreadable directory.

# Copyright (C) 2020-2022 Free Software Foundation, Inc.

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

# Find run as root would not fail for an unreadable directory.
skip_if_root_

# Prepare an unreadable directory, and the expected stdout/stderr.
mkdir tmp tmp/dir \
  && chmod 0311 tmp/dir \
  && echo 'tmp/dir' > exp \
  && echo "find: 'tmp/dir': Permission denied" > experr \
  || framework_failure_

# Run FTS-based find with -depth; versions < 4.7.0 failed to output
# an unreadable directory (see #54171).
returns_ 1 find tmp -depth -name dir > out 2> err || fail=1

compare exp out || fail=1
compare experr err || fail=1

Exit $fail

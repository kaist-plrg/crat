#!/bin/sh
# This test verifies that find refuses the internal -noop, ---noop option.
# Between findutils-4.3.1 and 4.6, find dumped core ($? = 139).
# See Savannah bug #48180.

# Copyright (C) 2016-2022 Free Software Foundation, Inc.

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

# Exercise both the previous name of the pseudo-option '-noop',
# and the now renamed '---noop' option.
for opt in 'noop' '--noop'; do
  rm -f out err || framework_failure_
  returns_ 1 find "-${opt}" > out 2> err || fail=1
  compare /dev/null out || fail=1
  grep "find: unknown predicate .-${opt}." err \
    || { cat err; fail=1; }
done

Exit $fail

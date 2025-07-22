#!/bin/sh
# Ensure find(1) treats inode number 0 correctly.

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
print_ver_ find

# Skip test unless we find a file with inode number 0.
# GNU/Hurd uses inode 0 for /dev/console.
f='/dev/console'
test -e "${f}" \
  && ino=$( stat -c '%i' "${f}" ) \
  && test "${ino}" = '0' \
  || skip_ "no file with inode number 0 here"

echo "${f}" > exp || framework_failure_

# Ensure -inum works.
# Find by exact inode number 0.
find "${f}" -inum 0 >out 2>err || fail=1
compare exp out || fail=1
compare /dev/null err || fail=1

# Find by inode number <1.
find "${f}" -inum -1 >out 2>err || fail=1
compare exp out || fail=1
compare /dev/null err || fail=1

# No match with unrelated inode number.
find "${f}" -inum 12345 >out 2>err || fail=1
compare /dev/null out || fail=1
compare /dev/null err || fail=1

# Ensure '-printf "%i"' works.
echo 0 > exp || framework_failure_
find "${f}" -printf '%i\n' >out 2>err || fail=1
compare exp out || fail=1
compare /dev/null err || fail=1

Exit $fail

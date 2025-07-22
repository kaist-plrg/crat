#!/bin/sh
# Verify that ls -i and find -printf %i produce the same output.

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

make_canonical() {
  sed -e '
    # Solaris ls outputs with leading padding blanks; strip them.
    s/^ *//g;
    # Squeeze blanks between inode number and name to one underscore.
    s/ /_/g'
}

# Create a file.
> file || framework_failure_

# Let ls(1) create the expected output.
ls -i file | make_canonical > exp || framework_failure_

rm -f out out2
find file -printf '%i_%p\n' > out || fail=1
make_canonical < out > out2 || framework_failure_
compare exp out2 || fail=1

Exit $fail

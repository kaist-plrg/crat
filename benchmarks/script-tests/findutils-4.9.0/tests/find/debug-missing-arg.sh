#!/bin/sh
# Verify that 'find -D' without further argument outputs an error diagnostic.
# Between FINDUTILS_4_3_1-1 and 4.6, find crashed on some platforms.
# See Savannah bug #52220.

# Copyright (C) 2017-2022 Free Software Foundation, Inc.

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

returns_ 1 find -D >/dev/null 2> err || fail=1
grep -F "find: Missing argument after the -D option." err \
  || { cat err; fail=1; }

Exit $fail

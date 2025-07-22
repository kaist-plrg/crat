#!/bin/sh
# This test verifies that find invokes the given command for the
# multiple-argument sytax '-exec CMD {} +'.  Between FINDUTILS-4.2.12
# and v4.6.0, find(1) would have failed to execute CMD another time
# if there was only one last single file argument.
# See Savannah bug #48030.

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

# Require seq(1) for this test - which may not be available
# on some systems, e.g on some *BSDs.
seq 2 >/dev/null 2>&1 \
  || skip_ "required utility 'seq' missing"

DIR='RashuBug'
# Name of the CMD to execute: the file name must be 6 characters long
# (to trigger the bug in combination with the test files).
CMD='tstcmd'

# Create the CMD script and check that it works.
mkdir 'bin' \
  && printf '%s\n' '#!/bin/sh' 'printf "%s\n" "$@"' > "bin/$CMD" \
  && chmod +x "bin/$CMD" \
  && PATH="$PWD/bin:$PATH" \
  && [ "$( find bin -maxdepth 0 -exec "$CMD" '{}' + )" = 'bin' ] \
  || framework_failure_

# Create expected output file - also used for creating the test data.
{ seq -f "${DIR}/abcdefghijklmnopqrstuv%04g" 901 &&
  seq -f "${DIR}/abcdefghijklmnopqrstu%04g" 902 3719
} > exp2 \
  && LC_ALL=C sort exp2 > exp \
  && rm exp2 \
  || framework_failure_

# Create test files, and check if test data has been created correctly.
mkdir "$DIR" \
  && xargs touch < exp \
  && [ -f "${DIR}/abcdefghijklmnopqrstu3719" ] \
  && [ 3719 = $( find "$DIR" -type f | wc -l ) ] \
  || framework_failure_


find "$DIR" -type f -exec "$CMD" '{}' + > out || fail=1
LC_ALL=C sort out > out2 || fail=1
compare exp out2 || fail=1

Exit $fail

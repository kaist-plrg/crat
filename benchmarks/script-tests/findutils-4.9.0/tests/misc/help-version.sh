#!/bin/sh
# Make sure all of these programs work properly
# when invoked with --help or --version.

# Copyright (C) 2019-2022 Free Software Foundation, Inc.

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

# Terminate any background processes
cleanup_() { kill $pid 2>/dev/null && wait $pid; }
echo PATH=$PATH
test "$built_programs" \
  || fail_ "built_programs not specified!?!"

test "$VERSION" \
  || fail_ "set envvar VERSION; it is required for a PATH sanity-check"

# Extract version from --version output of the first program
for i in $built_programs; do
  v=$(set -x; env $i --version | sed -n '1s/.* //p;q')
  break
done

# Ensure that it matches $VERSION.
test "x$v" = "x$VERSION" \
  || fail_ "--version-\$VERSION mismatch"

for i in $built_programs; do
  # Make sure they exit successfully, under normal conditions.
  env $i --help    >/dev/null || fail=1
  env $i --version >/dev/null || fail=1

  # Make sure they fail upon 'disk full' error.
  if test -w /dev/full && test -c /dev/full; then
    prog=$(set -x; echo $i|sed "s/$EXEEXT$//");
    eval "expected=\$expected_failure_status_$prog"
    test x$expected = x && expected=1

    returns_ $expected env $i --help    >/dev/full 2>/dev/null &&
    returns_ $expected env $i --version >/dev/full 2>/dev/null ||
    {
      fail=1
      env $i --help >/dev/full 2>/dev/null
      status=$?
      echo "*** $i: bad exit status '$status' (expected $expected)," 1>&2
      echo "  with --help or --version output redirected to /dev/full" 1>&2
    }
  fi
done

Exit $fail

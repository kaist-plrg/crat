#!/bin/sh
# Exercise -files0-from option.

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

# Option -files0-from requires a file name argument.
returns_ 1 find -files0-from > out 2> err \
  && grep 'missing argument.*files0' err \
  || { grep . out err; fail=1; }

# Option -files0-from must not be combined with passing starting points on
# the command line.
returns_ 1 find OFFENDING -files0-from FILE > out 2> err \
  && grep 'extra operand .*OFFENDING' err \
  && grep 'file operands cannot be combined with -files0-from' err \
  || { grep . out err; fail=1; }

# Process "." as default when starting points neither passed via -files0-from
# nor on the command line.
printf "%s\n" '.' > exp || framework_failure_
find -maxdepth 0 > out 2> err || fail=1
compare exp out || fail=1
compare /dev/null err || fail=1

# Option -files0-from with argument "-" (=stdin) must not be combined with
# the -ok action: getting the user confirmation would mess with stdin.
returns_ 1 find -files0-from - -ok echo '{}' ';' < /dev/null > out 2> err \
  && grep 'files0.* standard input .*cannot be combined with .*ok' err \
  || { grep . out err; fail=1; }

# Option -files0-from with argument "-" (=stdin) must not be combined with
# the -okdir action: getting the user confirmation would mess with stdin.
returns_ 1 find -files0-from - -okdir echo '{}' ';' < /dev/null > out 2> err \
  && grep 'files0.* standard input .*cannot be combined with .*ok' err \
  || { grep . out err; fail=1; }

# File argument of -files0-from option must not refer to the same file as stdin.
printf '.' > in || framework_failure_
returns_ 1 find -files0-from in -ok echo '{}' ';' < in > out 2> err \
    && grep 'files0.* standard input .*same file .*with -ok, -okdir' err \
  || { grep . out err; fail=1; }

# Likewise via a symlink.
if ln -s in inlink; then
  returns_ 1 find -files0-from inlink -ok echo '{}' ';' < in > out 2> err \
    && grep 'files0.* standard input .*same file .*with -ok, -okdir' err \
    || { grep . out err; fail=1; }
  # ... and vice versa.
  returns_ 1 find -files0-from in -ok echo '{}' ';' < inlink > out 2> err \
    && grep 'files0.* standard input .*same file .*with -ok, -okdir' err \
    || { grep . out err; fail=1; }
fi

# Likewise when the system provides the name '/dev/stdin'.
if ls /dev/stdin >/dev/null 2>&1; then
  returns_ 1 find -files0-from /dev/stdin -ok echo '{}' ';' < in > out 2> err \
    && grep 'files0.* standard input .*same file .*with -ok, -okdir' err \
    || { grep . out err; fail=1; }
fi

# Exercise a non-existing file.
returns_ 1 find -files0-from ENOENT > out 2> err \
  && grep 'cannot open .ENOENT. for reading: No such' err \
  || { grep . out err; fail=1; }

# Exercise a file which cannot be opened.
# The shadow(5) file seems to be a good choice for reasonable coverage.
f='/etc/shadow'
if test -e $f && test '!' -r $f; then
  returns_ 1 find -files0-from $f > out 2> err \
  && grep 'cannot open .* for reading: Permission denied' err \
  || { grep . out err; fail=1; }
fi

# Exercise a directory argument.
# On most modern systems like GNU/Linux, read(2)-ing from a directory file
# descriptor will fail with EISDIR.  Skip on other plaforms where that succeeds,
# e.g. on GNU/Hurd and AIX.
if returns_ 1 cat / >/dev/null; then
  returns_ 1 find -files0-from / > out 2> err \
    && grep 'read error' err \
    || { grep . out err; fail=1; }
fi

# Exercise an empty input file.
find -files0-from /dev/null > out 2> err || fail=1
compare /dev/null out || fail=1
compare /dev/null err || fail=1

# Likewise via stdin.
find -files0-from - < /dev/null > out 2> err || fail=1
compare /dev/null out || fail=1
compare /dev/null err || fail=1

# Likewise via a pipe on stdin.
cat /dev/null | find -files0-from - > out 2> err || fail=1
compare /dev/null out || fail=1
compare /dev/null err || fail=1

# With the -files0-from option, now a closing paren could be passed as first
# predicate (without, it is treated as a starting point).  Ensure that find(1)
# handles this invalid invocation.
returns_ 1 find -files0-from - ')' -print < /dev/null > out 2> err || fail=1
compare /dev/null out || fail=1
grep "inv.*: expected expression before closing parentheses" err || fail=1

# Likewise in the case find(1) implicitly adds the default action via the
# artificial '( user-expr ) -print' construct.
returns_ 1 find -files0-from - ')' < /dev/null > out 2> err || fail=1
compare /dev/null out || fail=1
grep "inv.*: expected expression before closing parentheses" err || fail=1

# Now a regular case: 2 files: expect the same output.
touch a b || framework_failure_
printf '%s\0' a b > in || framework_failure_
tr '\0' '\n' < in > exp || framework_failure_
find -files0-from in -print > out 2> err || fail=1
compare exp out || fail=1
compare /dev/null err || fail=1

# Demonstrate that -files0-from accepts file names which would otherwise be
# rejected because they are recognized as test or action.
touch ./-print ./-mtime ./-size || framework_failure_
printf '%s\0' _print _mtime _size | tr '_' '-' > in || framework_failure_
tr '\0' '\n' < in > exp || framework_failure_
find -files0-from in -printf '%p\n' > out 2> err || fail=1
compare exp out || fail=1
compare /dev/null err || fail=1

# Zero-length file name on position 2, once per stdin ...
printf '%s\n' a b > exp || framework_failure_
printf '%s\0' a '' b \
  | tee file \
  | returns_ 1 find -files0-from - -print > out 2> err || fail=1
compare exp out || fail=1
grep '(standard input).:2: invalid zero-length file name' err || fail=1
# ... and once per file.
returns_ 1 find -files0-from file -print > out 2> err || fail=1
compare exp out || fail=1
grep 'file.:2: invalid zero-length file name' err || fail=1

# Non-existing file name.
printf '%s\0' a ENOENT b \
  | returns_ 1 find -files0-from - -print > out 2> err || fail=1
compare exp out || fail=1
grep 'ENOENT' err || fail=1

# Demonstrate (the usual!) recursion ...
mkdir d1 d1/d2 d1/d2/d3 && touch d1/d2/d3/file || framework_failure_
printf '%s\n' d1 d1/d2 d1/d2/d3 d1/d2/d3/file > exp || framework_failure_
printf 'd1' \
  | find -files0-from - > out 2> err || fail=1
compare exp out || fail=1
compare /dev/null err || fail=1
# ... and how to avoid recursion with -maxdepth 0.
find d1 -print0 \
  | find -files0-from - -maxdepth 0 > out 2> err || fail=1
compare exp out || fail=1
compare /dev/null err || fail=1

# Large input with files.
# Create file 'exp' with many times its own name as content, NUL-separated;
# and let find(1) check that it worked (4x in bytes).
yes exp | head -n 100000 | tr '\n' '\0' > exp \
  && find exp -size 400000c | grep . || framework_failure_
# Run the test.
find -files0-from exp > out 2> err || fail=1
# ... and verify.
test $( wc -l < out ) = 100000 || fail=1
find out -size 400000c | grep . || fail=1

Exit $fail

#!/bin/sh
# Verify that find -used works.

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

# Create sample files with the access date D=10,20,30,40 days in the future.
for d in 10 20 30 40; do
  touch -a -d "$(date -d "$d day" '+%Y-%m-%d %H:%M:%S')" t$d \
    || skip_ "creating files with future timestamp failed"

  # Check with stat(1) the access (%X) vs. the status change time (%z).
  exp="$( expr 86400 '*' $d )" \
    && x="$( stat -c "%X" t$d )" \
    && z="$( stat -c "%Z" t$d )" \
    && tdiff="$( expr "$x" - "$z" )" \
    && test "$tdiff" -ge "$exp" \
    || skip_ "cannot verify timestamps of sample files"
done
# Create another sample file with timestamp now.
touch t00 \
  || skip_ "creating sample file failed"

stat -c "Name: %n  Access: %x  Change: %z" t?0 || : # ignore error.

# Verify the output for "find -used $d".  Use even number of days to avoid
# possibly strange effects due to atime/ctime precision etc.
for d in -45 -35 -25 -15 -5 0 5 15 25 35 45 +0 +5 +15 +25 +35 +45; do
  echo "== testing: find -used $d"
  find . -type f -name 't*' -used $d > out2 \
    || fail=1
  LC_ALL=C sort out2 || framework_failure_
done > out

cat <<\EOF > exp || framework_failure_
== testing: find -used -45
./t00
./t10
./t20
./t30
./t40
== testing: find -used -35
./t00
./t10
./t20
./t30
== testing: find -used -25
./t00
./t10
./t20
== testing: find -used -15
./t00
./t10
== testing: find -used -5
./t00
== testing: find -used 0
== testing: find -used 5
== testing: find -used 15
== testing: find -used 25
== testing: find -used 35
== testing: find -used 45
== testing: find -used +0
./t10
./t20
./t30
./t40
== testing: find -used +5
./t10
./t20
./t30
./t40
== testing: find -used +15
./t20
./t30
./t40
== testing: find -used +25
./t30
./t40
== testing: find -used +35
./t40
== testing: find -used +45
EOF

compare exp out || { fail=1; cat out; }

Exit $fail

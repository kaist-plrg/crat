#!/bin/sh
# Verify that 'xargs -t' quotes the command properly when needed.

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
print_ver_ xargs

# Prepare a command with a whitespace in its file name.
printf "%s\n" \
  '#!/bin/sh' \
  'echo "$@"' \
  > 'my command' \
  && chmod +x 'my command' \
  || framework_failure_

# Run xargs with -t for verious commands which require quoting.
printf '%s\0' \
  000 \
  '10 0' \
  '20"0' \
  "30'0" \
  40$'\n'0 \
  | xargs -0t '-I{}' './my command' 'hel lo' '{}' world > out 2> err \
  || fail=1

# Verify stderr.
cat <<\EOF > experr || framework_failure_
'./my command' 'hel lo' 000 world
'./my command' 'hel lo' '10 0' world
'./my command' 'hel lo' '20"0' world
'./my command' 'hel lo' "30'0" world
'./my command' 'hel lo' '40'$'\n''0' world
EOF
compare experr err || fail=1

# Verify stdout.
cat <<\EOF > expout || framework_failure_
hel lo 000 world
hel lo 10 0 world
hel lo 20"0 world
hel lo 30'0 world
hel lo 40
0 world
EOF
compare expout out || fail=1

Exit $fail

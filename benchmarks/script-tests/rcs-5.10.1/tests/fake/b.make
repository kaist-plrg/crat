#!/bin/sh
# b.make --- make b and b.d/*

# Copyright (C) 2010-2022 Thien-Thi Nguyen
#
# This file is part of GNU RCS.
#
# GNU RCS is free software: you can redistribute it and/or modify it
# under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# GNU RCS is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty
# of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
# See the GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <http://www.gnu.org/licenses/>.

test x"$1" = x || cd "$1"

RCSINIT='-q' ; export RCSINIT
set -e
rm -rf b b,v b.d
rcs -i -c' + ' -t-'foo foo@foo@@ foo@' b,v
mkdir b.d

checkin ()
{
    # $1 -- filename stem
    # $2 -- utc time, of the form "MM/DD HH:MM:SS"
    # $3 -- log message
    # $4 -- (optional) symbolic name
    # $5 -- (optional) more args
    f=b.d/$1
    if [ x"$4" = x ]
    then named=
    else named="-n$4"
    fi
    cp b   $f.ci
    ci -f -u -d"2010/$2" -m"$3" $named $5 b
    if [ x"$named" = x ] ; then
        cp b $f.cou
    else
        cp b b.d/$4.cou
        co -p b > $f.cou
    fi
    cp b,v $f,vu
    co -l b
    cp b   $f.col
    cp b,v $f,v
}

bop ()
{
    # $1 -- sed script
    # We don't use "sed -i" because that's a GNU extension.
    # Other restrictions, for the sake of portability:
    # - Avoid \n in the replacement portion of the ‘s’ command.
    sed -e "$1" b > b.NEW
    mv b.NEW b
}

changebranch ()
{
    # $1 -- revision number of branch point
    # $2 -- branch serial number
    bp=$1
    ser=$2
    rcs -u b,v
    rm -f b
    rcs -b${bp}.${ser} b,v
    co -l${bp} b
}

# 1.1
touch b
checkin 11 '03/17 09:00:17' 'The empty base.'

# 1.2
ls='Author Date Header Id Locker Name RCSfile Revision Source State Log'
for k in $ls ; do echo ';; $'$k'$' ; done > b
# echo nonempty > b
checkin 12 '03/30 09:45:02' 'Add empty headers.'

# 1.3
bop '$s/$/\
greetings/'
checkin 13 '03/30 09:45:42' 'Add "greetings".' '' '-wzurg'

# 1.4
bop '$s/$/\
earthlings/'
checkin 14 '03/30 09:46:24' 'Add "earthlings".' '' '-wzurg'

# 1.5
bop '/^greetings/s/^/;; Here is some trailing text\
/'
checkin 15 '03/30 09:46:50' 'Add text after Log block.'

# 1.6
bop '1s/^/wow, rcs!$Revision$YES!\
\
/'
checkin 16 '04/12 12:16:58' 'Add $Revision with pre and suffix.'

# 1.7
bop '1s/$/$1$2$3$RevisionZ$/'
checkin 17 '04/12 13:20:50' 'Add more keyword weirdness.'

# 1.8
bop '1s/$/Nym:42$/'
checkin 18 '04/12 13:23:04' 'Add even more keyword weirdness.'

# 1.9
bop '2s/.*/z/'
checkin 19 '04/18 09:39:02' 'Minor change, plus set state.' '' '-sQQQ'

# Start branch: 1.1.1
changebranch 1.1 1

# 1.1.1.1
echo '(so long)' > b
checkin 1111 '03/17 09:01:43' '(so long)'

# 1.1.1.2
checkin 1112 '03/17 09:01:43' 'No change, forced checkin.'

# 1.1.1.3
checkin 1113 '03/17 09:11:48' 'Another changeless checkin.'

# 1.1.1.4
bop '1s/.*/$Id$/'
checkin 1114 '03/18 06:12:32' 'Replace first line with RCS keyword.'

# 1.1.1.5
ls=`for k in $ls ; do echo $k ; done | sort`
for k in $ls ; do echo '$'$k'$' ; echo ; done > b
checkin 1115 '03/18 06:21:03' 'WOW is 1.1.1.5! (+ sorted kw)' WOW

# 1.1.1.6
bop '/^.Log/{
s|^|/*\
 * |
n
s/^/ * /
n
s/^/ * /
n
s/^/ * /
s|$|\
 */|
}'
checkin 1116 '03/18 06:22:00' 'Surround Log kw with C-style comment.'

# 1.1.1.7
bop '$s/$/z/'
checkin 1117 '03/28 16:04:26' 'Replace last line with "z".'

# Start branch: 1.6.1
changebranch 1.6 1

# 1.6.1.1
bop '1s/$/ HMMM/'
checkin 1611 '05/05 12:18:30' 'This should in theory go to 1.6.1.1.'

# 1.6.1.2
bop '1s/$/ THIS SHOULD BE NAMED "ZOW"!/'
checkin 1612 '10/21 22:48:48' 'Add a name.' ZOW

# Finish up.
rcs -u b,v
mv -f b,v b
chmod -w b.d/*

exit 0

# b.make ends here

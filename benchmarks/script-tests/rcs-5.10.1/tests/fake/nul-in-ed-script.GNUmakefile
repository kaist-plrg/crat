# nul-in-ed-script.GNUmakefile --- make nul-in-ed-script

# Copyright (C) 2016-2022 Thien-Thi Nguyen
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

w	= nul-in-ed-script
v	= $(w),v

quiet	= -q

mogrify	= printf $(1) > $(w)
checkin	= ci $(quiet) -l -m'$(1)' $(w) $(v)

$(w): prep 1.1 1.2 done

prep:
	rm -f $(w) $(v)
	touch $(w)
	rcs $(quiet) -i -t- $(w) $(v)

# Initial commit:
# |^@         (actually: single NUL on its own line)
# |two
# |three
1.1:
	$(call mogrify, \\0\\ntwo\\nthree\\n)
	$(call checkin, A)

# Second commit, comprising a change in two lines (1 and 3):
# |one
# |two
# |THREE
#
# This essentially sets up x,v to contain frag:
# |text
# |@d1 1
# |a1 1
# |^@         (actually: single NUL on its own line)
# |d3 1
# |a3 1
# |three
# |@
1.2:
	$(call mogrify, one\\ntwo\\nTHREE\\n)
	$(call checkin, B)

done:
	rcs $(quiet) -u $(v)
	mv -f $(v) $(w)

# nul-in-ed-script.GNUmakefile ends here

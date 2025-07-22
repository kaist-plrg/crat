# stdio-sync.GNUmakefile --- make stdio-sync

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

w	= stdio-sync
v	= $(w),v

quiet	= -q

mogrify	= sed -i -e $(strip $(1)) $(w)
checkin	= ci $(quiet) -j -l -d'$(1)' -m$(strip $(2)) $(w)

$(w): prep 1.1 1.2 1.3 done

prep:
	rm -f $(w) $(v)
	rcs $(quiet) -i -c' + ' -t-'foo foo@foo@@ foo@' $(v)

1.1:
	printf '%s\n%s\n' one two > $(w)
	$(call checkin, 2010-01-01, 'Two lines.')

1.2:
	$(call mogrify, '1s/.*/&&/')
	$(call checkin, 2010-01-02, 'Double the first line.')

1.3:
	$(call mogrify, '1s/.*/&&/')
	$(call checkin, 2010-01-03, 'Double the first line again.')

done:
	rcs $(quiet) -u $(v)
	mv -f $(v) $(w)

# stdio-sync.GNUmakefile ends here

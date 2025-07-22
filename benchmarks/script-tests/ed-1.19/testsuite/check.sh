#! /bin/sh
# check script for GNU ed - The GNU line editor
# Copyright (C) 2006-2023 Antonio Diaz Diaz.
#
# This script is free software; you have unlimited permission
# to copy, distribute, and modify it.

LC_ALL=C
export LC_ALL
objdir=`pwd`
testdir=`cd "$1" ; pwd`
ED="${objdir}"/ed
framework_failure() { echo "failure in testing framework" ; exit 1 ; }

if [ ! -f "${ED}" ] || [ ! -x "${ED}" ] ; then
	echo "${ED}: cannot execute"
	exit 1
fi

if [ -d tmp ] ; then rm -rf tmp ; fi
mkdir tmp
cd "${objdir}"/tmp || framework_failure

cat "${testdir}"/test.txt > test.txt || framework_failure
cat "${testdir}"/test.bin > test.bin || framework_failure
touch zero || framework_failure
fail=0

printf "testing ed-%s...\n" "$2"

# Run the .err scripts first with a regular file connected to standard
# input, since these don't generate output; they exit with non-zero status.
for i in "${testdir}"/*.err ; do
	if "${ED}" -s test.txt < "$i" > /dev/null 2>&1 ; then
		echo "*** The script $i exited abnormally ***"
		fail=127
	fi
done

# Run the .err scripts again with a regular file connected to standard
# input, but with '--loose-exit-status'; they should exit with zero status.
for i in "${testdir}"/*.err ; do
	if "${ED}" -sl test.txt < "$i" > /dev/null 2>&1 ; then
		true
	else
		echo "*** The script $i failed '--loose-exit-status' ***"
		fail=127
	fi
done

# Run the .err scripts again as pipes - these should exit with non-zero
# status without altering the contents of the buffer; the produced
# 'out.ro' must be identical to 'test.txt'.
for i in "${testdir}"/*.err ; do
	base=`echo "$i" | sed 's,^.*/,,;s,\.err$,,'`	# remove dir and ext
	if cat "$i" | "${ED}" -s test.txt > /dev/null 2>&1 ; then
		echo "*** The piped script $i exited abnormally ***"
		fail=127
	else
		if cmp -s out.ro test.txt ; then
			true
		else
			mv -f out.ro ${base}.ro
			echo "*** Output ${base}.ro of piped script $i is incorrect ***"
			fail=127
		fi
	fi
	rm -f out.ro
done

# Run the .ed scripts and compare their output against the .r files,
# which contain the correct output.
# The .ed scripts should exit with zero status.
for i in "${testdir}"/*.ed ; do
	base=`echo "$i" | sed 's,^.*/,,;s,\.ed$,,'`	# remove dir and ext
	if "${ED}" -s test.txt < "$i" > /dev/null 2> out.log ; then
		if cmp -s out.o "${testdir}"/${base}.r ; then
			true
		else
			mv -f out.o ${base}.o
			echo "*** Output ${base}.o of script $i is incorrect ***"
			fail=127
		fi
	else
		mv -f out.log ${base}.log
		echo "*** The script $i exited abnormally ***"
		fail=127
	fi
	rm -f out.o out.log
done

rm -f test.txt test.bin zero

if [ ${fail} = 0 ] ; then
	echo "tests completed successfully."
	cd "${objdir}" && rm -r tmp
else
	echo "tests failed."
	echo "Please, send a bug report to bug-ed@gnu.org."
	echo "Include the (compressed) contents of '${objdir}/tmp' in the report."
fi
exit ${fail}

## Process this file with automake to produce Makefile.in -*-Makefile-*-.

## Copyright (C) 2007-2022 Free Software Foundation, Inc.

## This program is free software: you can redistribute it and/or modify
## it under the terms of the GNU General Public License as published by
## the Free Software Foundation, either version 3 of the License, or
## (at your option) any later version.

## This program is distributed in the hope that it will be useful,
## but WITHOUT ANY WARRANTY; without even the implied warranty of
## MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
## GNU General Public License for more details.

## You should have received a copy of the GNU General Public License
## along with this program.  If not, see <https://www.gnu.org/licenses/>.

built_programs = find xargs frcode locate updatedb

# Indirections required so that we'll still be able to know the
# complete list of our tests even if the user overrides TESTS
# from the command line (as permitted by the test harness API).
TESTS = $(all_tests)
root_tests = $(all_root_tests)

EXTRA_DIST += $(all_tests)

TEST_EXTENSIONS = .sh

SH_LOG_COMPILER = $(SHELL)

# We don't want this to go in the top-level directory.
TEST_SUITE_LOG = tests/test-suite.log

# Note that the first lines are statements.  They ensure that environment
# variables that can perturb tests are unset or set to expected values.
# The rest are envvar settings that propagate build-related Makefile
# variables to test scripts.
TESTS_ENVIRONMENT =				\
  . $(srcdir)/tests/lang-default;		\
  tmp__=$${TMPDIR-/tmp};			\
  test -d "$$tmp__" && test -w "$$tmp__" || tmp__=.;	\
  . $(srcdir)/tests/envvar-check;		\
  TMPDIR=$$tmp__; export TMPDIR;		\
  export					\
  VERSION='$(VERSION)'				\
  LOCALE_FR='$(LOCALE_FR)'			\
  LOCALE_FR_UTF8='$(LOCALE_FR_UTF8)'		\
  abs_top_builddir='$(abs_top_builddir)'	\
  abs_top_srcdir='$(abs_top_srcdir)'		\
  abs_srcdir='$(abs_srcdir)'			\
  built_programs='$(built_programs) $(single_binary_progs)' \
  fail=0					\
  host_os=$(host_os)				\
  host_triplet='$(host_triplet)'		\
  srcdir='$(srcdir)'				\
  top_srcdir='$(top_srcdir)'			\
  CONFIG_HEADER='$(abs_top_builddir)/$(CONFIG_INCLUDE)' \
  CC='$(CC)'					\
  AWK='$(AWK)'					\
  EGREP='$(EGREP)'				\
  EXEEXT='$(EXEEXT)'				\
  MAKE=$(MAKE)					\
  PACKAGE_VERSION=$(PACKAGE_VERSION)		\
  PERL='$(PERL)'				\
  SHELL='$(PREFERABLY_POSIX_SHELL)'		\
  ; test -d /usr/xpg4/bin && PATH='/usr/xpg4/bin$(PATH_SEPARATOR)'"$$PATH"; \
  PATH='$(abs_top_builddir)/find$(PATH_SEPARATOR)$(abs_top_builddir)/locate$(PATH_SEPARATOR)$(abs_top_builddir)/xargs$(PATH_SEPARATOR)'"$$PATH" \
  ; 9>&2

# On failure, display the global testsuite log on stdout.
VERBOSE = yes

EXTRA_DIST += \
  init.cfg \
  tests/envvar-check \
  tests/init.sh \
  tests/lang-default \
  tests/other-fs-tmpdir \
  tests/sample-test

all_root_tests = \
  tests/find/type_list.sh


ALL_RECURSIVE_TARGETS += check-root
.PHONY: check-root
check-root:
	$(MAKE) check TESTS='$(root_tests)' SUBDIRS=.

# Do not choose a name that is a shell keyword like 'if', or a
# commonly-used utility like 'cat' or 'test', as the name of a test.
# Otherwise, VPATH builds will fail on hosts like Solaris, since they
# will expand 'if test ...' to 'if .../test ...', and the '.../test'
# will execute the test script rather than the standard utility.

# Notes on the ordering of these tests:
# Place early in the list tests of the tools that
# are most commonly used in test scripts themselves.
# E.g., nearly every test script uses rm and chmod.
# help-version comes early because it's a basic sanity test.
# Put seq early, since lots of other tests use it.
# Put tests that sleep early, but not all together, so in parallel builds
# they share time with tests that burn CPU, not with others that sleep.
# Put head-elide-tail early, because it's long-running.

all_tests = \
  tests/misc/help-version.sh \
  tests/find/depth-unreadable-dir.sh \
  tests/find/inode-zero.sh \
  tests/find/many-dir-entries-vs-OOM.sh \
  tests/find/name-lbracket-literal.sh \
  tests/find/printf_escapechars.sh \
  tests/find/printf_escape_c.sh \
  tests/find/printf_inode.sh \
  tests/find/execdir-fd-leak.sh \
  tests/find/exec-plus-last-file.sh \
  tests/find/files0-from.sh \
  tests/find/refuse-noop.sh \
  tests/find/debug-missing-arg.sh \
  tests/find/used.sh \
  tests/xargs/conflicting_opts.sh \
  tests/xargs/verbose-quote.sh \
  $(all_root_tests)

$(TEST_LOGS): $(PROGRAMS)

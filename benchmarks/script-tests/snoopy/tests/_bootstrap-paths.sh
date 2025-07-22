#!/bin/bash



### Configure shell
#
set -e
set -u



### Get variables from the build system
#
SNOOPY_LIBDIR="/usr/local/lib"
LIBSNOOPY_SO_PATH="$SNOOPY_LIBDIR/libsnoopy.so"

#!/bin/bash



### Configure shell and bootstrap
#
set -e
set -u
. `dirname $BASH_SOURCE`/_bootstrap.sh



### Get data
#
VAL_SNOOPY=`$SNOOPY_TEST_CLI run datasource group`
VAL_REAL=`id -a | grep -Eo 'gid=[0-9]+[(][^)]+[)]' | grep -Eo '[(][^)]+[)]' | grep -Eo '[^()]+'`



### Evaluate
#
snoopy_test_compareValues "$VAL_SNOOPY" "$VAL_REAL"

#!/bin/bash

set -e

test ! -z "/usr/bin/php7.4"
test -x "/usr/bin/php7.4"

INI_FILE=`/usr/bin/php7.4 -d 'display_errors=stderr' -r 'echo php_ini_loaded_file();' 2> /dev/null`

if test "$INI_FILE"; then \
  /usr/bin/grep -E -h -v '^(magic_quotes_(gpc|runtime|sybase)?|(zend_)?extension(_debug)?(_ts)?)[\t\ ]*=' "$INI_FILE" > tmp-php.ini
else
  echo > tmp-php.ini
fi

INI_SCANNED_PATH=`/usr/bin/php7.4 -d 'display_errors=stderr' -r '$a = explode(",\n", trim(php_ini_scanned_files())); echo $a[0];' 2> /dev/null`

if test "$INI_SCANNED_PATH"; then \
  INI_SCANNED_PATH=`./build/shtool path -d $INI_SCANNED_PATH`
  /usr/bin/grep -E -h -v '^(magic_quotes_(gpc|runtime|sybase)?|(zend_)?extension(_debug)?(_ts)?)[\t\ ]*=' "$INI_SCANNED_PATH"/*.ini >> tmp-php.ini
fi

/usr/bin/php7.4 -n -c tmp-php.ini -d open_basedir= -d output_buffering=0 -d memory_limit=-1 run-tests.php -n -c tmp-php.ini -d extension_dir=./modules/ -d extension=rdkafka.so
TEST_RESULT_EXIT_CODE=$?

rm tmp-php.ini
exit $TEST_RESULT_EXIT_CODE

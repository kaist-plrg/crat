#!/bin/bash

check_webdis_running() {
  if ps -p $WEBDIS_PID > /dev/null; then
    return 0
  else
    return 1
  fi
}

redis-server --daemonize yes

./webdis &
WEBDIS_PID=$!

python3 tests/basic.py
BASIC_TEST_RESULT=$?

python3 tests/limits.py
LIMITS_TEST_RESULT=$?

check_webdis_running
WEBDIS_RUNNING=$?

if [ $WEBDIS_RUNNING -eq 0 ]; then
  kill $WEBDIS_PID
fi

redis-cli SHUTDOWN

if [ $BASIC_TEST_RESULT -eq 0 ] && [ $LIMITS_TEST_RESULT -eq 0 ] && [ $WEBDIS_RUNNING -eq 0 ]; then
  exit 0
else
  exit 1
fi

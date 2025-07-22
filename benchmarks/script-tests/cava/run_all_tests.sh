#!/bin/bash
set -e

## run these commands before testing:
# squeezelite -v -m 51:fb:32:f8:e6:9f -z
# sudo modprobe snd-aloop
# arecord -D hw:Loopback,1 -c 2 -r 44100 > /tmp/fifo &

cleanup() {
  kill %1 2>/dev/null || true
  pulseaudio -k 2>/dev/null || true
}
trap cleanup EXIT

pulseaudio -D
squeezelite -o pulse -v -m 51:fb:32:f8:e6:9f &

TESTCFGS="example_files/test_configs/*"
for f in $TESTCFGS
do
    echo "testing $f"
    ./cava -p $f > /dev/null
    echo "OK!"
done
echo "all tests completed successfully"

## clean up:
# killall squeezelite
# sudo rmmod snd_aloop
# killall arecord
exit 0

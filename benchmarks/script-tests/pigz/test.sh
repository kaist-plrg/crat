#!/bin/bash

set -e

./pigz -kf pigz.c
./pigz -t pigz.c.gz
./pigz -kfb 32 pigz.c
./pigz -t pigz.c.gz
./pigz -kfp 1 pigz.c
./pigz -t pigz.c.gz
./pigz -kfz pigz.c
./pigz -t pigz.c.zz
./pigz -kfK pigz.c
./pigz -t pigz.c.zip
printf "" | ./pigz -cdf | wc -c | test `cat` -eq 0
printf "x" | ./pigz -cdf | wc -c | test `cat` -eq 1
printf "xy" | ./pigz -cdf | wc -c | test `cat` -eq 2
printf "xyz" | ./pigz -cdf | wc -c | test `cat` -eq 3
(printf "w" | gzip ; printf "x") | ./pigz -cdf | wc -c | test `cat` -eq 2
(printf "w" | gzip ; printf "xy") | ./pigz -cdf | wc -c | test `cat` -eq 3
(printf "w" | gzip ; printf "xyz") | ./pigz -cdf | wc -c | test `cat` -eq 4
if test "`which compress | grep /`" != ""; then
  echo 'compress -f < pigz.c | ./unpigz | cmp - pigz.c'
  compress -f < pigz.c | ./unpigz | cmp - pigz.c
fi

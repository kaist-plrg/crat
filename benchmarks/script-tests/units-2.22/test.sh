#!/bin/bash

set -e

echo Checking units
./units -f ./definitions.units \
      '(((square(kiloinch)+2.84m2) /0.5) meters^2)^(1|4)' m \
    | sed -n -e 's/\t\* //p' > .chk
if [ "`cat .chk`" = 6 ]; then
  echo Units seems to work
else
  echo Something is wrong: units failed the check:
  cat .chk
  exit 1
fi
rm -f .chk

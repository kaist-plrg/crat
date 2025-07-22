#!/bin/bash

set -e

diff -s result <(bin/streem examples/02hello.strm \
              && bin/streem examples/03fizzbuzz.strm \
              && bin/streem examples/04emit.strm \
              && bin/streem examples/05emit.strm \
              && bin/streem examples/05filter.strm \
              && bin/streem examples/09match.strm)

#!/bin/sh

set -e

RUST_BACKTRACE=1 ./src/server/target/debug/server \
--data data \
--static static=/src/front/static \
--static build=/src/front/build
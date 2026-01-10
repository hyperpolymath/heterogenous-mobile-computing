#!/bin/bash -eu
# SPDX-License-Identifier: PMPL-1.0
cd $SRC/heterogenous-mobile-computing
cargo +nightly fuzz build
for target in $(cargo +nightly fuzz list); do
    cp ./target/x86_64-unknown-linux-gnu/release/$target $OUT/
done

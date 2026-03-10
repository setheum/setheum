#!/bin/bash

set -e

RUSTFLAGS="-Z instrument-coverage" \
    LLVM_PROFILE_FILE="set_bft-%m.profraw" \
    cargo test --tests $1 2> covtest.out

version=$(grep Running covtest.out | sed -e "s/.*set_bft-\(.*\))/\1/")
rm covtest.out
cp target/debug/deps/set_bft-"$version" target/debug/deps/set_bft-coverage

cargo profdata -- merge -sparse set_bft-*.profraw -o set_bft.profdata
rm set_bft-*.profraw

cargo cov -- report \
    --use-color \
    --ignore-filename-regex='/rustc' \
    --ignore-filename-regex='/.cargo/registry' \
    --instr-profile=set_bft.profdata \
    --object target/debug/deps/set_bft-coverage

#!/usr/bin/env bash

set -e

# Build the move-stdlib bundle
pushd .
if [ ! -d "MoveStdlib" ]; then
    git clone https://github.com/eigerco/move-stdlib.git MoveStdlib
fi
cd MoveStdlib
$1 bundle
popd

# Build the substrate-stdlib bundle
pushd .
if [ ! -d "SubstrateStdlib" ]; then
    git clone https://github.com/eigerco/substrate-stdlib.git SubstrateStdlib
fi
cd SubstrateStdlib
$1 bundle
popd

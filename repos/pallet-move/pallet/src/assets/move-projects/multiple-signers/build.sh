#!/bin/bash
cd $(dirname $0)
# Participants SS58-addresses.
BOB=5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
ALICE=5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
DAVE=5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy
EVE=5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw
# Build the Move sources.
smove bundle
# Generate script-transactions.
# 1. init_module(Bob)
smove create-transaction --compiled-script-path build/multiple-signers/bytecode_scripts/init_module.mv --args signer:$BOB
# 2. rent_apartment(Alice, Dave, Eve, 2)
smove create-transaction --compiled-script-path build/multiple-signers/bytecode_scripts/rent_apartment.mv --args signer:$ALICE signer:$DAVE signer:$EVE u8:2

#!/bin/sh
cd $(dirname $0)
# Build the project
smove bundle
# Create all Script-Transactions
smove create-transaction --compiled-script-path build/car-wash-example/bytecode_scripts/initial_coin_minting.mv --args signer:5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
smove create-transaction --compiled-script-path build/car-wash-example/bytecode_scripts/register_new_user.mv --args signer:5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
smove create-transaction --compiled-script-path build/car-wash-example/bytecode_scripts/buy_coin.mv --args signer:5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY u8:1
smove create-transaction --compiled-script-path build/car-wash-example/bytecode_scripts/wash_car.mv --args signer:5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY

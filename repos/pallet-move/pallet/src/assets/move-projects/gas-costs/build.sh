#!/bin/sh
cd $(dirname $0)
ALICE=5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
# Build the project
smove build
sh ./gen_cal_scripts.sh
# Create all Script-Transactions
smove create-transaction --compiled-script-path build/gas-costs/bytecode_scripts/short_cheap_script.mv
smove create-transaction --compiled-script-path build/gas-costs/bytecode_scripts/short_expensive_script.mv --args signer:$ALICE
smove create-transaction --compiled-script-path build/gas-costs/bytecode_scripts/long_script.mv --args signer:$ALICE bool:true
mv build/gas-costs/script_transactions/long_script.mvt build/gas-costs/script_transactions/long_cheap_script.mvt
smove create-transaction --compiled-script-path build/gas-costs/bytecode_scripts/long_script.mv --args signer:$ALICE bool:false
mv build/gas-costs/script_transactions/long_script.mvt build/gas-costs/script_transactions/long_expensive_script.mvt
sh ./gen_smove_instr.sh

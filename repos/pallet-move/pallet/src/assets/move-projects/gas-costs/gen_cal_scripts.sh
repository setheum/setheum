#!/bin/sh
cd $(dirname $0)

BASH_SH="./gen_smove_instr.sh"
ITERATIONS=25
ALICE=5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
BOB=5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty

write_smove_cmd() {
    cp build/gas-costs/bytecode_scripts/mint.mv build/gas-costs/bytecode_scripts/mint_$1.mv
    printf "\nsmove create-transaction -c build/gas-costs/bytecode_scripts/mint_$1.mv --args signer:$BOB address:$ALICE u64:$1" >> $BASH_SH
}

printf "#!/bin/sh" > $BASH_SH
for i in $(seq 1 $(($ITERATIONS)))
do
    write_smove_cmd $i
done
printf "\nsmove create-transaction -c build/gas-costs/bytecode_scripts/publish_basic_balance.mv --args signer:$ALICE" >> $BASH_SH

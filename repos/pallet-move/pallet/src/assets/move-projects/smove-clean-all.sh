# #!/bin/bash

# Position the cwd in the same folder with the script (where the below folders are located)
cd $(dirname $0)

build_dir=(
    "balance"
    "base58_smove_build"
    "basic_coin"
    "car-wash-example"
    "gas-costs"
    "gas-costs-bundles"
    "get-resource"
    "move-basics"
    "multiple-signers"
    "prohibited-bundle"
    "signer-scripts"
    "testing-move-stdlib"
    "testing-substrate-stdlib"
    "using_stdlib_natives"
)

# Clean build directories.
for dir in "${build_dir[@]}"; do
    echo $dir
    clean_script=$dir"/clean.sh"
    if [ -f "$clean_script" ];
    then
        sh $clean_script
    else
        rm -rf "$dir/build"
    fi
done

# #!/bin/bash

# Position the cwd in the same folder with the script (where the below folders are located)
cd $(dirname $0)

build_dir=(
    "address_checks"
    "base58_smove_build"
    "basic_coin"
    "depends_on__using_stdlib_full"
    "depends_on__using_stdlib_natives"
    "empty"
    "simple_scripts"
    "using_stdlib_full"
    "substrate_balance"
    "substrate_stdlib_hash"
)
bundle_dir=("using_stdlib_natives")

# Build simple packages
for i in "${build_dir[@]}"; do
    echo $i
    smove build -p $i
done

# Build bundles
for i in "${bundle_dir[@]}"; do
    echo $i
    smove bundle -p $i
done

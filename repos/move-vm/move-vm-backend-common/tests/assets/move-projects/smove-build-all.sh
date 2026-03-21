# #!/bin/bash

# Position the cwd in the same folder with the script (where the below folders are located)
cd $(dirname $0)

build_dir=(
    "signer_scripts"
)

# Build simple packages
for i in "${build_dir[@]}"; do
    echo $i
    smove build -p $i
done

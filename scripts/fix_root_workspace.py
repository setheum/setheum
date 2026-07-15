#!/usr/bin/env python3
# Add pallet-sheyth-vm to root workspace deps
p = '/home/balqaasem/setheum/Cargo.toml'
with open(p, 'r') as f:
    c = f.read()

# Add workspace dep after module-setbft-runtime-api
old = '[workspace.dependencies.module-setbft-runtime-api]'
new = '''[workspace.dependencies.pallet-sheyth-vm]
path = "repos/setheum/runtime-modules/sheyth-vm"
default-features = false

[workspace.dependencies.module-setbft-runtime-api]'''

c = c.replace(old, new)

with open(p, 'w') as f:
    f.write(c)
print('Done')

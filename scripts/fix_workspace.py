#!/usr/bin/env python3
p = '/home/balqaasem/setheum/Cargo.toml'
with open(p, 'r') as f:
    c = f.read()
c = c.replace('"repos/setheum/runtime-modules/unified-accounts"', '"repos/setheum/runtime-modules/sheyth-vm"')
with open(p, 'w') as f:
    f.write(c)
print('Done')

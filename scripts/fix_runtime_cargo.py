#!/usr/bin/env python3
p = '/home/balqaasem/setheum/repos/setheum/runtime/Cargo.toml'
with open(p, 'r') as f:
    c = f.read()
c = c.replace('"module-setbft/std", "module-vesting/std"', '"module-setbft/std", "pallet-sheyth-vm/std", "module-vesting/std"')
with open(p, 'w') as f:
    f.write(c)
print('Done')

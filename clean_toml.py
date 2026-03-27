import tomli
import tomli_w

with open('/home/balqaasem/setheum/Cargo.toml', 'rb') as f:
    doc = tomli.load(f)

# Clean workspace members
new_members = []
for member in doc.get('workspace', {}).get('members', []):
    if not ('move-vm' in member or 'runtime-modules/move' in member or 'setheum-move' in member):
        new_members.append(member)
if 'members' in doc.get('workspace', {}):
    doc['workspace']['members'] = new_members

# Clean workspace dependencies
deps = doc.get('workspace', {}).get('dependencies', {})
keys_to_remove = []
for k, v in deps.items():
    if isinstance(v, dict) and 'path' in v:
        path = v['path']
        if 'move-vm' in path or 'runtime-modules/move' in path or 'setheum-move' in path:
            keys_to_remove.append(k)
for k in keys_to_remove:
    del deps[k]

with open('/home/balqaasem/setheum/Cargo.toml', 'wb') as f:
    tomli_w.dump(doc, f)
print('Cleaned Cargo.toml')

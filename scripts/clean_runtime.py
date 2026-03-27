import re

with open('/home/balqaasem/setheum/repos/setheum/runtime/Cargo.toml', 'r') as f:
    text = f.read()

# Remove features
text = re.sub(r'\"module-move/std\"[,\s]*', '', text)
text = re.sub(r'\"module-move-runtime-api/std\"[,\s]*', '', text)

# Remove the dependency blocks
text = re.sub(r'\[dependencies\.module-move(\-runtime\-api)?\][^\[]*', '', text)
text = re.sub(r'\[dependencies\.setheum-move[^\]]*\][^\[]*', '', text)

with open('/home/balqaasem/setheum/repos/setheum/runtime/Cargo.toml', 'w') as f:
    f.write(text)
print("Cleaned runtime Cargo.toml successfully")

import re
import sys

def main():
    try:
        with open('/home/balqaasem/setheum/Cargo.toml', 'r') as f:
            lines = f.readlines()
            
        new_lines = []
        skip_block = False
        
        for i, line in enumerate(lines):
            # Members list handling
            if "repos/move-vm" in line or "repos/setheum/runtime-modules/move" in line or "repos/setheum/setheum-move" in line:
                if 'members =' in line:
                    # It's a single line members list, use regex to remove
                    cleaned = re.sub(r'\"repos/move-vm[^\"]*\"\s*,?', '', line)
                    cleaned = re.sub(r'\"repos/setheum/runtime-modules/move[^\"]*\"\s*,?', '', cleaned)
                    cleaned = re.sub(r'\"repos/setheum/setheum-move[^\"]*\"\s*,?', '', cleaned)
                    new_lines.append(cleaned)
                    continue
                else:
                    # It's a multi-line members list, just skip this line
                    continue
            
            # Dependencies blocks handling
            if line.startswith('[workspace.dependencies.'):
                if 'move' in line.lower() and ('module-move' in line or 'move-' in line or '-move' in line):
                    skip_block = True
                    continue
                else:
                    skip_block = False
                    
            if skip_block and line.strip() != '' and not line.startswith('['):
                continue
            if skip_block and line.startswith('['):
                skip_block = False
                
            # Features handling
            if '"module-move' in line or '"setheum-move' in line:
                cleaned = re.sub(r'\"module-move[^\"]*\"\s*,?', '', line)
                cleaned = re.sub(r'\"setheum-move[^\"]*\"\s*,?', '', cleaned)
                new_lines.append(cleaned)
                continue
                
            new_lines.append(line)
            
        with open('/home/balqaasem/setheum/Cargo.toml', 'w') as f:
            f.writelines(new_lines)
            
        print("Cleaned Cargo.toml successfully")
    except Exception as e:
        print(f"Error: {e}")
        sys.exit(1)

if __name__ == '__main__':
    main()

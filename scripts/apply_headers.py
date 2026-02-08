#!/usr/bin/env python3
import os
import sys

# Define the paths to the header templates
ROOT_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
HEADER_GPL3 = os.path.join(ROOT_DIR, 'HEADER-GPL3')
HEADER_MIT_APACHE = os.path.join(ROOT_DIR, 'HEADER-MIT-APACHE')

# Map directories to their respective license headers
# Apache2.0/MIT for setheum-js, sheyth, set-bft, clique, setheum-client, aggregator, flooder and rate-limiter
# The rest is all GPL3
MIT_APACHE_PROJECTS = [
    'setheum-js',
    'sheyth',
    'set-bft',
    'setheum/clique',
    'setheum/setheum-client',
    'setheum/aggregator',
    'setheum/tests/flooder',
    'setheum/rate-limiter'
]

def get_header(file_path):
    rel_path = os.path.relpath(file_path, ROOT_DIR)
    
    # Check for local LICENSE files in the directory hierarchy
    current_dir = os.path.dirname(file_path)
    while current_dir.startswith(ROOT_DIR):
        for license_file in ['LICENSE', 'LICENSE.md', 'LICENSE-GPL3.md', 'LICENSE-MIT.md', 'LICENSE-APACHE.md']:
            if os.path.exists(os.path.join(current_dir, license_file)):
                # If we find a local license, we could potentially derive the header.
                # But for this task, we follow the specific project rules provided.
                break
        current_dir = os.path.dirname(current_dir)

    for project in MIT_APACHE_PROJECTS:
        if rel_path.startswith(project):
            return HEADER_MIT_APACHE
    return HEADER_GPL3

def apply_header(file_path, header_path):
    with open(header_path, 'r') as hf:
        header_text = hf.read().strip()
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    # If the file already starts with the exact header, do nothing
    if content.strip().startswith(header_text):
        return False

    content_lines = content.splitlines(keepends=True)
    
    # Check if a header (comment block) already exists at the top
    if content_lines and content_lines[0].strip().startswith('//'):
        end_idx = 0
        is_doc_comment = False
        for i, line in enumerate(content_lines):
            stripped = line.strip()
            # Documentation comments start with /// or //!
            if stripped.startswith('///') or stripped.startswith('//!'):
                is_doc_comment = True
                break
            # Regular comments start with //
            if stripped.startswith('//') or stripped == '':
                end_idx = i + 1
            else:
                break
        
        if not is_doc_comment and end_idx > 0:
            # We found a regular comment block at the top. 
            # Check if it looks like a license header.
            header_candidate = "".join(content_lines[:end_idx])
            if any(indicator in header_candidate for indicator in ['Copyright', 'License', 'SPDX', 'بِسْمِ اللَّهِ']):
                content_lines = content_lines[end_idx:]

    # Clean up leading empty lines
    while content_lines and content_lines[0].strip() == '':
        content_lines.pop(0)

    new_content = header_text + '\n\n' + "".join(content_lines)
    
    with open(file_path, 'w') as f:
        f.write(new_content)
    return True

def main():
    for root, dirs, files in os.walk(ROOT_DIR):
        # Skip hidden directories like .git
        if '.git' in dirs:
            dirs.remove('.git')
        if 'target' in dirs:
            dirs.remove('target')
        if 'node_modules' in dirs:
            dirs.remove('node_modules')

        for file in files:
            if file.endswith(('.rs', '.js', '.ts', '.tsx')):
                file_path = os.path.join(root, file)
                header_path = get_header(file_path)
                if apply_header(file_path, header_path):
                    print(f"Applied {os.path.basename(header_path)} to {os.path.relpath(file_path, ROOT_DIR)}")
            elif file == 'Cargo.toml':
                file_path = os.path.join(root, file)
                header_path = get_header(file_path)
                # For Cargo.toml, we update the license field instead of adding a header
                license_str = "GPL-3.0-or-later WITH Classpath-exception-2.0"
                if header_path == HEADER_MIT_APACHE:
                    license_str = "Apache-2.0 OR MIT"
                
                with open(file_path, 'r') as f:
                    lines = f.readlines()
                
                new_lines = []
                for line in lines:
                    if line.strip().startswith('license ='):
                        new_lines.append(f'license = "{license_str}"\n')
                    else:
                        new_lines.append(line)
                
                with open(file_path, 'w') as f:
                    f.writelines(new_lines)
                print(f"Updated license in {os.path.relpath(file_path, ROOT_DIR)} to {license_str}")

if __name__ == "__main__":
    main()

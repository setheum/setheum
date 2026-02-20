import argparse
import sys
import subprocess
import re
from pathlib import Path
import yaml
import tomllib


ROOT = Path(__file__).resolve().parents[1]
PRDOC_DIR = ROOT / "prdoc"
CHANGELOG_PATH = ROOT / "CHANGELOG.md"
WORKSPACE_TOML = ROOT / "Cargo.toml"


SECTION_ORDER = [
    "Breaking",
    "Added",
    "Changed",
    "Fixed",
    "Removed",
    "Deprecated",
    "Security",
]

BUMP_VALUES = {"patch", "minor", "major", "none"}


def get_git_output(args):
    try:
        result = subprocess.run(
            ["git"] + args,
            capture_output=True,
            text=True,
            check=True,
            cwd=ROOT
        )
        return result.stdout.strip()
    except subprocess.CalledProcessError as e:
        print(f"Git error: {e.stderr}", file=sys.stderr)
        return None


def get_latest_tag():
    return get_git_output(["describe", "--tags", "--abbrev=0"])


def get_commits_since(since):
    output = get_git_output(["log", f"{since}..HEAD", "--pretty=format:%H%x09%s%x09%b%x00"])
    if not output:
        return []
    
    commits = []
    for line in output.split("\x00"):
        if not line.strip():
            continue
        parts = line.strip().split("\x09")
        if len(parts) >= 2:
            hash_val = parts[0]
            subject = parts[1]
            body = parts[2] if len(parts) > 2 else ""
            commits.append({
                "hash": hash_val,
                "subject": subject,
                "body": body
            })
    return commits


def get_changed_files(commit_hash):
    output = get_git_output(["show", "--name-only", "--pretty=format:", commit_hash])
    if not output:
        return []
    return [line.strip() for line in output.splitlines() if line.strip()]


def load_workspace_crates():
    crates = {}
    queue = [ROOT]
    visited = set()
    while queue:
        workspace_dir = queue.pop()
        if workspace_dir in visited:
            continue
        visited.add(workspace_dir)
        cargo_toml = workspace_dir / "Cargo.toml"
        if not cargo_toml.exists():
            continue
        try:
            data = tomllib.loads(cargo_toml.read_text(encoding="utf-8"))
        except Exception:
            continue
            
        package_name = data.get("package", {}).get("name")
        if package_name:
            # Store relative path for easy matching
            rel_path = str(workspace_dir.relative_to(ROOT))
            crates[package_name] = rel_path if rel_path != "." else ""
        
        workspace = data.get("workspace", {})
        members = workspace.get("members", [])
        for member in members:
            if "*" in member:
                base_dir = workspace_dir / member.split("*")[0]
                if base_dir.exists():
                    for p in base_dir.iterdir():
                        if p.is_dir() and (p / "Cargo.toml").exists():
                            queue.append(p)
            else:
                member_path = (workspace_dir / member).resolve()
                if member_path.exists():
                    queue.append(member_path)
    return crates


def map_files_to_crates(files, workspace_crates):
    affected_crates = set()
    # Sort crates by path length descending to match most specific path first
    sorted_crates = sorted(workspace_crates.items(), key=lambda x: len(x[1]), reverse=True)
    
    for file_path in files:
        for name, path in sorted_crates:
            if not path: # root crate
                continue
            if file_path.startswith(path + "/"):
                affected_crates.add(name)
                break
        else:
            # If no specific crate path matched, it might be the root crate
            # But usually we only care about sub-crates in a workspace
            pass
    return sorted(list(affected_crates))


def iter_prdoc_files():
    if not PRDOC_DIR.exists():
        return []
    return sorted(PRDOC_DIR.rglob("*.prdoc"))


def validate_prdoc_data(path, data, workspace_crates):
    errors = []
    if not isinstance(data, dict):
        errors.append("document is not a dictionary/table")
        return errors
    
    title = data.get("title")
    if not title or not isinstance(title, str):
        errors.append("title must be a string")
        
    doc = data.get("doc")
    if not doc or not isinstance(doc, str):
        errors.append("doc must be a string")
        
    crates = data.get("crates")
    if not isinstance(crates, list) or not crates:
        errors.append("crates must be a non-empty array")
        return errors
        
    for index, crate in enumerate(crates):
        if not isinstance(crate, dict):
            errors.append(f"crates[{index}] must be a table")
            continue
        name = crate.get("name")
        section = crate.get("section")
        note = crate.get("note")
        bump = crate.get("bump")
        
        if not name or not isinstance(name, str):
            errors.append(f"crates[{index}].name must be a string")
        elif name not in workspace_crates:
            errors.append(f"crates[{index}].name {name} is not a workspace crate")
            
        if not section or not isinstance(section, str):
            errors.append(f"crates[{index}].section must be a string")
        elif section not in SECTION_ORDER:
            errors.append(
                f"crates[{index}].section must be one of {', '.join(SECTION_ORDER)}"
            )
            
        if not note or not isinstance(note, str):
            errors.append(f"crates[{index}].note must be a string")
            
        if not bump or not isinstance(bump, str) or bump not in BUMP_VALUES:
            errors.append(
                f"crates[{index}].bump must be one of {', '.join(sorted(BUMP_VALUES))}"
            )
            
    return errors


def load_prdocs(workspace_crates):
    prdocs = []
    errors = []
    for path in iter_prdoc_files():
        try:
            with open(path, "r", encoding="utf-8") as f:
                data = yaml.safe_load(f)
        except Exception as exc:
            errors.append(f"{path.name}: {exc}")
            continue
            
        issues = validate_prdoc_data(path, data, workspace_crates)
        if issues:
            for issue in issues:
                errors.append(f"{path.relative_to(PRDOC_DIR)}: {issue}")
            continue
        prdocs.append((path, data))
    return prdocs, errors


def build_entries(prdocs):
    entries = {}
    for path, data in prdocs:
        for crate in data["crates"]:
            crate_name = crate["name"]
            section = crate["section"]
            note = crate["note"]
            suffix = f"({path.name})"
            entries.setdefault(crate_name, {}).setdefault(section, []).append(
                f"{note} {suffix}"
            )
    return entries


def render_block(entries):
    block = ["### PRDocs"]
    if not entries:
        block.append("- No entries.")
        return block
        
    for crate_name in sorted(entries.keys()):
        block.append(f"#### {crate_name}")
        for section in SECTION_ORDER:
            items = entries[crate_name].get(section)
            if not items:
                continue
            block.append(f"##### {section}")
            for item in items:
                block.append(f"- {item}")
    return block


def update_changelog(block):
    if not CHANGELOG_PATH.exists():
        CHANGELOG_PATH.write_text("# Changelog\n\n## [Unreleased]\n", encoding="utf-8")
        
    lines = CHANGELOG_PATH.read_text(encoding="utf-8").splitlines()
    
    try:
        unreleased_index = next(
            index for index, line in enumerate(lines) if "[Unreleased]" in line
        )
    except StopIteration:
        if lines and lines[0].startswith("# "):
            lines.insert(1, "")
            lines.insert(2, "## [Unreleased]")
            unreleased_index = 2
        else:
            lines = ["# Changelog", "", "## [Unreleased]"] + lines
            unreleased_index = 2
            
    section_start = unreleased_index + 1
    section_end = None
    for index in range(section_start, len(lines)):
        if lines[index].startswith("## "):
            section_end = index
            break
    if section_end is None:
        section_end = len(lines)
        
    prdocs_start = None
    for index in range(section_start, section_end):
        if lines[index].strip() == "### PRDocs":
            prdocs_start = index
            break
            
    if prdocs_start is None:
        new_lines = (
            lines[:section_start] + [""] + block + [""] + lines[section_start:]
        )
    else:
        prdocs_end = section_end
        for index in range(prdocs_start + 1, section_end):
            if lines[index].startswith("### "):
                prdocs_end = index
                break
        new_lines = lines[:prdocs_start] + block + lines[prdocs_end:]
        
    CHANGELOG_PATH.write_text("\n".join(new_lines) + "\n", encoding="utf-8")


def run_validate():
    workspace_crates = load_workspace_crates()
    prdocs, errors = load_prdocs(workspace_crates)
    if errors:
        print("Validation errors found:", file=sys.stderr)
        for error in errors:
            print(f"  - {error}", file=sys.stderr)
        return 1
    print(f"Validated {len(prdocs)} PRDoc file(s).")
    return 0


def run_generate():
    workspace_crates = load_workspace_crates()
    prdocs, errors = load_prdocs(workspace_crates)
    if errors:
        print("Cannot generate CHANGELOG due to validation errors:", file=sys.stderr)
        for error in errors:
            print(f"  - {error}", file=sys.stderr)
        return 1
    entries = build_entries(prdocs)
    update_changelog(render_block(entries))
    print("Updated CHANGELOG.md.")
    return 0


def run_scaffold(since=None):
    if not since:
        since = get_latest_tag()
        if not since:
            print("Error: No latest tag found and --since was not provided.", file=sys.stderr)
            return 1
            
    print(f"Scaffolding PRDocs for changes since {since}...")
    
    workspace_crates = load_workspace_crates()
    commits = get_commits_since(since)
    
    if not commits:
        print("No changes found since the specified revision.")
        return 0
        
    draft_dir = PRDOC_DIR / "drafts"
    draft_dir.mkdir(parents=True, exist_ok=True)
    
    scaffolded_count = 0
    for commit in commits:
        # Skip merge commits if they don't have interesting info or are just noise
        # But usually merge commits in PR workflows carry the PR title
        files = get_changed_files(commit["hash"])
        affected_crates = map_files_to_crates(files, workspace_crates)
        
        if not affected_crates:
            continue
            
        # Clean up subject for filename
        safe_subject = re.sub(r"[^a-z0-9]+", "-", commit["subject"].lower()).strip("-")
        filename = f"draft-{commit['hash'][:8]}-{safe_subject[:30]}.prdoc"
        filepath = draft_dir / filename
        
        prdoc_data = {
            "title": commit["subject"],
            "doc": commit["body"] if commit["body"] else "Detailed description of the change.",
            "audience": ["Developer"],
            "crates": [
                {
                    "name": crate,
                    "section": "Changed",
                    "note": commit["subject"],
                    "bump": "patch"
                } for crate in affected_crates
            ]
        }
        
        with open(filepath, "w", encoding="utf-8") as f:
            yaml.dump(prdoc_data, f, sort_keys=False, allow_unicode=True)
        
        print(f"  - Scaffolded {filename}")
        scaffolded_count += 1
        
    print(f"\nSuccessfully scaffolded {scaffolded_count} draft PRDocs in {draft_dir.relative_to(ROOT)}/")
    print("Please review, refine, and move them to the main prdoc/ directory.")
    return 0


def main():
    parser = argparse.ArgumentParser(description="PRDoc management tool")
    subparsers = parser.add_subparsers(dest="command")
    subparsers.required = True
    
    subparsers.add_parser("validate", help="Validate all .prdoc files")
    subparsers.add_parser("generate", help="Generate CHANGELOG entries from PRDocs")
    
    scaffold_parser = subparsers.add_parser("scaffold", help="Generate draft .prdoc files from git history")
    scaffold_parser.add_argument("--since", help="Revision to start from (tag, branch, or hash). Defaults to latest tag.")
    
    args = parser.parse_args()
    
    if args.command == "validate":
        return run_validate()
    if args.command == "generate":
        return run_generate()
    if args.command == "scaffold":
        return run_scaffold(args.since)
    return 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)

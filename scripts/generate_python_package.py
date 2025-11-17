#!/usr/bin/env python3
"""Generate Python package files for cc-check.

This script generates the __init__.py and __main__.py files for the Python package.
"""
import os
import sys
import textwrap
from pathlib import Path


def generate_init_content(version: str) -> str:
    """Generate the content for __init__.py."""
    return textwrap.dedent(f'''"""cc-check: Cross-platform conventional commit checker."""
import os
import sys
import platform
import subprocess
from pathlib import Path

__version__ = "{version}"

def _get_binary_path():
    """Get the path to the platform-specific binary."""
    system = platform.system().lower()
    machine = platform.machine().lower()
    
    # Map platform names
    if system == 'darwin':
        system = 'darwin'
        arch = 'arm64' if machine in ('arm64', 'aarch64') else 'x86_64'
    elif system == 'linux':
        system = 'linux'
        arch = 'aarch64' if machine in ('arm64', 'aarch64') else 'x86_64'
    elif system == 'windows':
        system = 'win32'
        arch = 'x86_64'
    else:
        raise OSError(f"Unsupported platform: {{{{system}}}}")
    
    binary_dir = Path(__file__).parent / 'binaries'
    if system == 'win32':
        binary_name = f'cc-check-{{system}}-{{arch}}.exe'
    else:
        binary_name = f'cc-check-{{system}}-{{arch}}'
    
    binary_path = binary_dir / binary_name
    if not binary_path.exists():
        raise FileNotFoundError(f"Binary not found: {{{{binary_path}}}}")
    
    return binary_path

def check(commit_msg_file=None, **kwargs):
    """Run cc-check on a commit message file."""
    binary = _get_binary_path()
    cmd = [str(binary), 'check']
    
    if commit_msg_file:
        cmd.append(commit_msg_file)
    
    # Add kwargs as flags
    for key, value in kwargs.items():
        flag_name = key.replace("_", "-")
        if value is True:
            cmd.append(f'--{{flag_name}}')
        elif value is not None:
            cmd.append(f'--{{flag_name}}')
            cmd.append(str(value))
    
    result = subprocess.run(cmd, capture_output=True, text=True)
    return result.returncode == 0, result.stdout, result.stderr
''')


def generate_main_content() -> str:
    """Generate the content for __main__.py."""
    return textwrap.dedent('''"""Entry point for python -m cc_check."""
import sys
from cc_check import check

def main():
    commit_file = sys.argv[1] if len(sys.argv) > 1 else None
    success, stdout, stderr = check(commit_file)
    if stdout:
        print(stdout)
    if stderr:
        print(stderr, file=sys.stderr)
    sys.exit(0 if success else 1)

if __name__ == '__main__':
    main()
''')


def main():
    """Main entry point."""
    if len(sys.argv) < 2:
        print("Usage: generate_python_package.py <version>", file=sys.stderr)
        sys.exit(1)
    
    version = sys.argv[1]
    output_dir = Path('cc_check')
    output_dir.mkdir(exist_ok=True)
    
    init_content = generate_init_content(version)
    main_content = generate_main_content()
    
    with open(output_dir / '__init__.py', 'w', encoding='utf-8') as f:
        f.write(init_content)
    
    with open(output_dir / '__main__.py', 'w', encoding='utf-8') as f:
        f.write(main_content)
    
    print(f"Generated Python package files in {output_dir}/")


if __name__ == '__main__':
    main()


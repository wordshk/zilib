#!/usr/bin/env python3

import os
import re
import sys

# Bump version of files
"""
Usage: bump.py <version>
"""
FILES = {
    "Cargo.toml": r'version = "\d+\.\d+\.\d+"',
    "zilib-python/Cargo.toml": r'version = "\d+\.\d+\.\d+"',
    "zilib-python/pyproject.toml": r'version = "\d+\.\d+\.\d+"',
}


def bump_version(file, line_pattern, version):
    lines = []
    already_found = False
    with open(file, 'r') as f:
        while line := f.readline():
            if not already_found and re.match(line_pattern, line):
                line = re.sub(r'\d+\.\d+\.\d+', version, line)
                already_found = True # only replace the first occurrence
            lines.append(line)

    with open(file, 'w') as f:
        f.write(''.join(lines))

def main():
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(1)

    version = sys.argv[1]

    for file, pattern in FILES.items():
        if not os.path.exists(file):
            print(f'File {file} not found')
            sys.exit(1)

        bump_version(file, pattern, version)
        print(f'File {file} updated to version {version}')

if __name__ == '__main__':
    main()

from enum import Enum
import os
import re
import argparse
import sys

os.chdir('.')

CONFIG = 'Cargo.toml'

VERSION_PATTERN = re.compile(r"""
\[package\]
.*?
version\s+=\s+"(.*?)"
""", re.VERBOSE | re.DOTALL)

MEMBER_PATTERN = re.compile(r"""
members\s+=\s+\[
(.*?)
\]
""", re.VERBOSE | re.DOTALL)

class VersionType(Enum):
    """
    Version type levels
    """
    MAJOR = 1
    MINOR = 2
    PATCH = 3

def parse_version_type(string: str) -> VersionType:
    """
    Parse the version type from a string
    """
    if string.lower() == 'major':
        return VersionType.MAJOR
    elif string.lower() == 'minor':
        return VersionType.MINOR
    elif string.lower() == 'patch':
        return VersionType.PATCH
    else:
        raise ValueError(f"Invalid version type: {string}")

def bump_version(version: str, version_type: VersionType) -> str:
    """
    Bump the version number according to the version type
    """
    version = version.split('.')
    assert len(version) == 3, 'Version format must be XX.XX.XX'
    match version_type:
        case VersionType.MAJOR:
            version[0] = str(int(version[0]) + 1)
            version[1] = '0'
            version[2] = '0'
        case VersionType.MINOR:
            version[1] = str(int(version[1]) + 1)
            version[2] = '0'
        case VersionType.PATCH:
            version[2] = str(int(version[2]) + 1)

    return '.'.join(version)

def query_members() -> list[str]:
    """
    Query all members from the root Cargo.toml file
    The members variable does not contain the root directory
    """
    with open(CONFIG, 'r', encoding='utf-8') as f:
        content = f.read()
        members = re.search(MEMBER_PATTERN, content).group(1)
        members = re.findall(r"\"(\S+)\"", members)
        return members

def query_version(path: str = '.') -> str:
    """
    Query the current version number
    """
    path = os.path.join(path, CONFIG)
    with open(path, 'r', encoding='utf-8') as f:
        content = f.read()
        match = VERSION_PATTERN.search(content)
        assert match, 'Version number not found'
        version = match.group(1)
        return version

def write_version(new_version: str):
    """
    Write the new version number in the configuration file
    """
    with open(CONFIG, 'r', encoding='utf-8') as f:
        content = f.read()
        content = VERSION_PATTERN.sub(fr'\g<0>version = "{new_version}"', content)

    print(content)

def main():
    """
    A simple command line interface to bump the version number
    """
    parser = argparse.ArgumentParser(description='Bump the version number in Cargo.toml')
    subparsers = parser.add_subparsers(dest='command')

    bump_parser = subparsers.add_parser('bump')
    bump_parser.add_argument(
        'bump',
        type=parse_version_type,
        nargs='?',
        default=VersionType.PATCH,
        help='Specify the version type [major|minor|patch] (default: patch)'
    )
    parser.add_argument(
        '-c', '--current',
        action='store_true',
        help='Print the current version number'
    )

    subparsers.add_parser('current')

    if len(sys.argv) == 1:
        parser.print_help(sys.stderr)
        sys.exit(1)

    args = parser.parse_args()
    if args.command == 'current' or args.current or args.command is None:
        print("Crate version: " + query_version())
        print("[*] This crate has the following members:")
        for member in query_members():
            print("  - " + member)
    elif args.command == 'bump':
        members = ['.'] + query_members()
        versions = []
        for member in members:
            version = query_version(member)
            new_version = bump_version(version, args.bump)
            print(f"[*] Bumping version of {member}: {version} -> {new_version}")
            versions.append(new_version)

        print("[!] Are you sure you want to bump the version number? [y/N]")
        answer = input()
        if len(answer.strip()) == 0 or answer.lower() == 'n':
            print("[!] Aborted")
        else:
            for member, version in zip(members, versions):
                write_version(version)
            print("[*] Version number bumped")

if __name__ == '__main__':
    main()

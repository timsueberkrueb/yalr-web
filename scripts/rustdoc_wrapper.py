#! /usr/bin/python3

"""
This wrapper is necessary because `cargo doc` doesn't let us use `--html-before-content` and `--html-in-header`. This is
only possible when using `rustdoc` directly.
"""

from typing import *

import sys
import json
import subprocess


def _run_cargo_and_rustdoc(crate_name: str):
    command, cwd = _build_rustdoc_command(crate_name)
    _run_cargo(crate_name)
    subprocess.check_call(command, cwd=cwd)


def _run_cargo(crate_name: str):
    subprocess.check_call(['cargo', '+nightly', 'build', '--all-features'], cwd=crate_name)


def _build_rustdoc_command(crate_name: str) -> Tuple[List[str], str]:
    build_plan = _get_cargo_build_plan(crate_name)
    assert build_plan['invocations'][-1]['package_name'] == crate_name
    relevant_plan = build_plan['invocations'][-1]
    command = ['rustup', 'run', 'nightly', 'rustdoc'] + _process_args(crate_name, relevant_plan['args'])
    command += [
        '-o', 'target/doc',
        '--html-before-content', '../../public/rustdoc_nav/index.html',
        '--html-in-header', '../../public/rustdoc_header/index.html'
    ]
    return command, relevant_plan['cwd']


def _process_args(crate_name, args):
    new_args = []
    idx = 0
    while idx < len(args):
        if args[idx] in ('--crate-name', '-L', '-C', '--cfg', '--extern'):
            new_args += [args[idx], args[idx + 1]]
            idx += 2
        elif args[idx].startswith('--edition=') or \
                (args[idx].startswith(crate_name) and
                (args[idx].endswith('lib.rs') or args[idx].endswith('main.rs'))):
            new_args += [args[idx]]
            idx += 1
        else:
            idx += 1
    return new_args


def _get_cargo_build_plan(crate_name: str):
    command = ['cargo', '+nightly', '-Z', 'unstable-options', 'build', '--build-plan', '--all-features']
    process = subprocess.Popen(command, stdout=subprocess.PIPE, cwd=crate_name)
    out, err = process.communicate()
    return json.loads(out)


def main():
    assert len(sys.argv) == 2
    crate_name = sys.argv[1]
    _run_cargo_and_rustdoc(crate_name)


if __name__ == '__main__':
    main()

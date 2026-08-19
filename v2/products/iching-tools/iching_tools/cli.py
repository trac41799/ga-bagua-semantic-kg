"""itools — unified CLI for the iching-tools suite (coverage / reframe / statediff).

One entry point, identical flag conventions, delegation to the validated packages.
"""

import argparse
import os
import subprocess
import sys

VERSION = "0.2.0"
HERE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
PACKAGE_DIRS = [os.path.join(HERE, d) for d in ("coverage", "reframe", "statediff")]


def _env_with_paths():
    env = dict(os.environ)
    source_dirs = [
        path for path in PACKAGE_DIRS
        if os.path.isdir(os.path.join(path, "iching_coverage"))
        or os.path.isdir(os.path.join(path, "iching_reframe"))
        or os.path.isdir(os.path.join(path, "iching_statediff"))
    ]
    if source_dirs:
        env["PYTHONPATH"] = os.pathsep.join(
            source_dirs + ([env["PYTHONPATH"]] if env.get("PYTHONPATH") else [])
        )
    return env


def _run_package(module, args):
    """Delegate to a package CLI: python -m <module> <args...>."""
    return subprocess.run([sys.executable, "-m", module] + args,
                          capture_output=True, text=True, env=_env_with_paths())


def main(argv=None):
    argv = argv if argv is not None else sys.argv[1:]
    if not argv or argv[0] in ("-h", "--help"):
        print("itools — iching-tools suite (validated I-Ching capabilities)\n")
        print("usage: itools <command> [options]")
        print("commands:")
        print("  coverage   audit/complete a plan against the 8-role checklist")
        print("  reframe    generate 8 algebra-grounded reframes of a statement")
        print("  statediff  summarize a state change as 3 aspect lines")
        print("\ncommon options: --json | --sim | --api-key K | --model M | --provider P")
        return 0
    if argv[0] in ("--version", "-V"):
        print(f"iching-tools {VERSION}")
        return 0
    cmd = argv[0]
    rest = argv[1:]
    if cmd == "coverage":
        module, args = "iching_coverage", rest
    elif cmd == "reframe":
        module, args = "iching_reframe", rest
    elif cmd == "statediff":
        module, args = "iching_statediff", rest
    else:
        print(f"error: unknown command: {cmd} (try 'itools --help')", file=sys.stderr)
        return 2
    proc = _run_package(module, args)
    sys.stdout.write(proc.stdout)
    sys.stderr.write(proc.stderr)
    return proc.returncode


if __name__ == "__main__":
    raise SystemExit(main())

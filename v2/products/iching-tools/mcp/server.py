"""Source-tree compatibility wrapper for the installed ``iching_mcp`` server."""

import os
import sys


HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
if ROOT not in sys.path:
    sys.path.insert(0, ROOT)
for package in ("coverage", "reframe", "statediff", "cl3calc", "xai", "rotor"):
    path = os.path.join(ROOT, package)
    if path not in sys.path:
        sys.path.insert(0, path)

from iching_mcp.server import main  # noqa: E402


if __name__ == "__main__":
    main(sim="--sim" in sys.argv)

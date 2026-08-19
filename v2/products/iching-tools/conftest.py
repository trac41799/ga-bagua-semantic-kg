"""Root conftest: make all tool packages importable from any test dir."""

import os
import sys

ROOT = os.path.dirname(os.path.abspath(__file__))
for pkg in ("coverage", "reframe", "statediff", "cl3calc", "xai", "rotor"):
    path = os.path.join(ROOT, pkg)
    if path not in sys.path:
        sys.path.insert(0, path)

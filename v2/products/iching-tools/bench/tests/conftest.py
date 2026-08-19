"""Make the benchmark helpers importable from the focused test directory."""

import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
BENCH = os.path.dirname(HERE)
TOOLS = os.path.dirname(BENCH)
for path in (TOOLS, BENCH):
    if path not in sys.path:
        sys.path.insert(0, path)

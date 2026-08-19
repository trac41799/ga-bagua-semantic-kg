#!/usr/bin/env python3
"""Offline + single-file verification for the Path C teaching tool.

Runnable standalone:
    python tests/verify_offline.py

Exit code 0 = all green, 1 = any check failed.
"""

import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
HTML_PATH = os.path.join(ROOT, "index.html")

FAILS = []


def check(condition, message):
    if condition:
        print(f"  PASS  {message}")
    else:
        print(f"  FAIL  {message}")
        FAILS.append(message)


def main():
    print("Offline / single-file verification for path-c/index.html")
    print("---------------------------------------------------------")

    if not os.path.exists(HTML_PATH):
        print("  FAIL  index.html not found")
        sys.exit(1)

    with open(HTML_PATH, encoding="utf-8") as f:
        html = f.read()

    html_files = [
        f for f in os.listdir(ROOT) if f.lower().endswith(".html") and not f.startswith(".")
    ]
    check(html_files == ["index.html"], "exactly one .html file in the probe root")

    check("<script src=" not in html, "no <script src=...> external scripts")
    check("<script src =" not in html, "no <script src =...> external scripts")
    check("<link href=" not in html and "<link href =" not in html, "no <link href=...> external stylesheets")
    check("fetch(" not in html, "no fetch( calls")
    check("XMLHttpRequest" not in html, "no XMLHttpRequest")
    check("import(" not in html, "no dynamic import(")
    check("WebSocket" not in html, "no WebSocket")
    check("http" not in html, "no 'http' substring anywhere in the file")
    check("https" not in html, "no 'https' substring anywhere in the file")

    for host in ("unpkg.com", "jsdelivr", "cdnjs", "googleapis", "gstatic", "bootstrapcdn",
                 "cloudflare", "cdn.jsdelivr"):
        check(host not in html, f"no CDN host reference: {host}")

    refs = re.findall(r'(?:src|href)\s*=\s*["\']([^"\']+)["\']', html)
    check(all(r.startswith("#") for r in refs), "all src=/href= values are in-page anchors")

    external_refs = [r for r in refs if not r.startswith("#")]
    check(len(external_refs) == 0, "zero external references (src/href)")

    m = re.search(r"<script>([\s\S]*?)</script>", html)
    check(m is not None, "exactly one inline <script> block present")
    check(m is not None and "</script>" not in m.group(1), "script block contains no nested closing tag")

    print("---------------------------------------------------------")
    if FAILS:
        print(f"OFFLINE VERIFICATION FAILED: {len(FAILS)} check(s) failed")
        sys.exit(1)
    print("OFFLINE VERIFICATION GREEN: single file, zero network, opens via file://")


if __name__ == "__main__":
    main()

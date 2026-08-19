"""T-C2 structural checks for the Path C single-file teaching tool.

Run: python -m pytest tests/test_tool.py -q
"""

import os
import re
import subprocess
import tempfile
from html.parser import HTMLParser

import pytest

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
HTML_PATH = os.path.join(ROOT, "index.html")

REQUIRED_IDS = [
    "gallery",
    "blade-view",
    "line-flip",
    "product-table",
    "hexagram-stack",
    "rotor-demo",
    "quiz",
    "glossary",
]

# trigram -> (bitmask, [line classes bottom->top], blade label, grade)
# masks: bit0 = bottom line, bit1 = middle, bit2 = top
EXPECTED_PANELS = [
    ("KUN", 0b000, ["yin", "yin", "yin"], "1", "0"),
    ("ZHEN", 0b001, ["yang", "yin", "yin"], "e1", "1"),
    ("KAN", 0b010, ["yin", "yang", "yin"], "e2", "1"),
    ("GEN", 0b100, ["yin", "yin", "yang"], "e3", "1"),
    ("DUI", 0b011, ["yang", "yang", "yin"], "e12", "2"),
    ("XUN", 0b110, ["yin", "yang", "yang"], "e23", "2"),
    ("LI", 0b101, ["yang", "yin", "yang"], "\u2212e31", "2"),
    ("QIAN", 0b111, ["yang", "yang", "yang"], "e123", "3"),
]

NETWORK_PATTERNS = [
    r"http://",
    r"https://",
    r"//cdn\.",
    r"src\s*=\s*['\"][^'\"#]",
    r"href\s*=\s*['\"][^'\"#]",
    r"url\s*\(\s*['\"]?https?",
    r"@import",
    r"fetch\s*\(",
    r"XMLHttpRequest",
]

CDN_HOSTS = ["unpkg", "jsdelivr", "googleapis", "cdnjs", "gstatic", "bootstrapcdn", "cloudflare"]


def read_html():
    with open(HTML_PATH, encoding="utf-8") as f:
        return f.read()


def extract_script(html):
    m = re.search(r"<script>([\s\S]*?)</script>", html)
    assert m, "no inline <script> block found"
    return m.group(1)


def run_node_js(js_program):
    """Run a node program that loads the inline script with a window shim."""
    code = (
        "const fs=require('fs');"
        "const h=fs.readFileSync(process.argv[1],'utf8');"
        "const m=h.match(/<script>([\\s\\S]*?)<\\/script>/);"
        "global.window={};eval(m[1]);"
        + js_program
    )
    r = subprocess.run(
        ["node", "-e", code, HTML_PATH],
        capture_output=True,
        text=True,
        encoding="utf-8",
        timeout=30,
    )
    assert r.returncode == 0, f"node subprocess failed: {r.stderr}"
    return r


class NoErrorParser(HTMLParser):
    def __init__(self):
        super().__init__()
        self.errors = []

    def error(self, message):
        self.errors.append(message)


def test_c21_single_file_no_external_references():
    html_files = [
        f for f in os.listdir(ROOT) if f.lower().endswith(".html") and not f.startswith(".")
    ]
    assert html_files == ["index.html"], f"expected exactly one html file, got {html_files}"
    html = read_html()
    for attr in ("src", "href"):
        for m in re.finditer(rf'{attr}\s*=\s*["\']([^"\']+)["\']', html):
            value = m.group(1)
            assert value.startswith("#"), f"external reference {attr}={value}"


def test_c22_zero_network():
    html = read_html()
    for pat in NETWORK_PATTERNS:
        assert not re.search(pat, html), f"network pattern found: {pat}"
    for host in CDN_HOSTS:
        assert host not in html, f"CDN host referenced: {host}"


def test_c23_html_parses():
    parser = NoErrorParser()
    parser.feed(read_html())
    assert parser.errors == []


def test_c24_js_syntax_valid_node_check():
    script = extract_script(read_html())
    fd, tmp = tempfile.mkstemp(suffix=".js")
    try:
        with os.fdopen(fd, "w", encoding="utf-8") as f:
            f.write(script)
        r = subprocess.run(["node", "--check", tmp], capture_output=True, text=True, timeout=30)
        assert r.returncode == 0, f"node --check failed: {r.stderr}"
    finally:
        os.remove(tmp)


def test_c25_required_dom_ids_present():
    html = read_html()
    for dom_id in REQUIRED_IDS:
        assert f'id="{dom_id}"' in html, f"missing DOM id: {dom_id}"


def _gallery_panel_chunks(html):
    gallery = re.search(r'<section[^>]*id="gallery"[\s\S]*?</section>', html)
    assert gallery, "gallery section not found"
    inner = gallery.group(0)
    parts = inner.split('<div class="trigram-panel"')
    assert len(parts) == 9, f"expected 8 panels, got {len(parts) - 1}"
    return parts[1:]


def test_c26_eight_trigram_panels_correct_lines_and_labels():
    html = read_html()
    chunks = _gallery_panel_chunks(html)
    for i, chunk in enumerate(chunks):
        expected = EXPECTED_PANELS[i]
        name = expected[0]
        m = re.search(r'data-trigram="([A-Z]+)" data-blade="([^"]+)" data-grade="([^"]+)"', chunk)
        assert m, f"panel {i}: missing data attributes"
        trigram, blade, grade = m.group(1), m.group(2), m.group(3)
        assert trigram == name, f"panel {i}: expected trigram {name}, got {trigram}"
        assert blade == expected[3], f"panel {i} ({name}): expected blade {expected[3]}, got {blade}"
        assert grade == expected[4], f"panel {i} ({name}): expected grade {expected[4]}, got {grade}"
        lines = re.findall(r'<span class="line (yang|yin)"></span>', chunk)
        assert len(lines) == 3, f"{name}: expected 3 line spans, got {len(lines)}"
        assert lines == expected[2], f"{name}: expected lines {expected[2]}, got {lines}"
        label = re.search(r'<div class="t-blade">([^<]+)</div>', chunk)
        assert label, f"{name}: missing blade label"
        assert label.group(1) == expected[3], f"{name}: label {label.group(1)} != {expected[3]}"


def test_c27_quiz_five_questions_and_scoring():
    script = extract_script(read_html())
    assert "QUIZ_CORRECT" in script
    assert "QUIZ_QUESTIONS" in script
    question_defs = re.findall(r"\{ q: '", script)
    assert len(question_defs) == 5, f"expected 5 quiz questions, got {len(question_defs)}"
    r = run_node_js(
        "console.log(JSON.stringify({all:window.BaguaTool.quizScore([2,1,1,0,1]),none:window.BaguaTool.quizScore([0,0,0,1,0])}));"
    )
    result = eval(r.stdout.strip())
    assert result["all"] == 5, "all-correct answers should score 5/5"
    assert result["none"] == 0, "all-wrong answers should score 0/5"


def test_c28_bagua_tool_exposed_with_required_api():
    html = read_html()
    for key in ("window.BaguaTool", "product:", "flipLine:", "hexagram:", "rotorApply:", "quizScore:"):
        assert key in html, f"missing API key in source: {key}"
    r = run_node_js(
        "console.log(JSON.stringify({p:window.BaguaTool.product('e1','e2'),f:window.BaguaTool.flipLine(2,1),h:window.BaguaTool.hexagram(2,5).name}));"
    )
    result = eval(r.stdout.strip())
    assert result["p"] == {"blade": "e12", "sign": 1}
    assert result["f"] == 0
    assert result["h"] == "\u6c34\u706b\u65e2\u6d4e"

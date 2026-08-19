"""Frozen benchmark: 20 input statements, 5 per domain.

Domains: product claims, policy statements, scientific hypotheses, design
decisions. The statements are frozen BEFORE any run: the canonical serialization
of STATEMENTS is hashed (sha256) into statements.sha256; run_all.py refuses to
run when the marker does not match.

Records: {"id": str, "domain": str, "text": str} in a fixed order.
"""

import hashlib
import json
from pathlib import Path

DOMAINS = ["product", "policy", "science", "design"]

STATEMENTS = [
    # ---- product claims ----
    {"id": "product-1", "domain": "product",
     "text": "The new battery charges from 0 to 80% in 18 minutes."},
    {"id": "product-2", "domain": "product",
     "text": "This smart lock unlocks in under 0.5 seconds with the fingerprint sensor."},
    {"id": "product-3", "domain": "product",
     "text": "Our router sustains 4K video streaming on 20 devices simultaneously without buffering."},
    {"id": "product-4", "domain": "product",
     "text": "The noise-cancelling earbuds reduce ambient noise by 35 dB on average."},
    {"id": "product-5", "domain": "product",
     "text": "This cloud backup service guarantees zero data loss with daily automated snapshots."},
    # ---- policy statements ----
    {"id": "policy-1", "domain": "policy",
     "text": "All public sector procurement must prefer suppliers with net-zero carbon targets by 2030."},
    {"id": "policy-2", "domain": "policy",
     "text": "Municipal parking fees double for SUVs to discourage oversized vehicles in the city core."},
    {"id": "policy-3", "domain": "policy",
     "text": "Schools should begin the academic day no earlier than 8:30 to protect adolescent sleep."},
    {"id": "policy-4", "domain": "policy",
     "text": "Platforms hosting algorithmic feeds must publish their ranking criteria annually."},
    {"id": "policy-5", "domain": "policy",
     "text": "Water utilities must cap residential tariffs at 4% of median household income."},
    # ---- scientific hypotheses ----
    {"id": "science-1", "domain": "science",
     "text": "Elevated night-time light exposure increases the incidence of metabolic syndrome in urban adults."},
    {"id": "science-2", "domain": "science",
     "text": "Tidal friction on Enceladus generates enough heat to sustain a subsurface liquid ocean."},
    {"id": "science-3", "domain": "science",
     "text": "Early bilingual exposure delays the onset of dementia by at least three years."},
    {"id": "science-4", "domain": "science",
     "text": "CRISPR-based gut microbiome editing reduces relapse rates in ulcerative colitis."},
    {"id": "science-5", "domain": "science",
     "text": "Atmospheric microplastic deposition measurably alters cloud nucleation rates."},
    # ---- design decisions ----
    {"id": "design-1", "domain": "design",
     "text": "The dashboard shows the three most frequent error codes instead of a full log."},
    {"id": "design-2", "domain": "design",
     "text": "We ship the app with dark mode as the default theme for battery savings."},
    {"id": "design-3", "domain": "design",
     "text": "The payment flow requires a single confirmation step rather than two-factor entry."},
    {"id": "design-4", "domain": "design",
     "text": "We cache search results for 60 seconds to cut database load during peak hours."},
    {"id": "design-5", "domain": "design",
     "text": "The keyboard shortcut Ctrl+Shift+K triggers the global command palette."},
]

_FREEZE_PATH = Path(__file__).resolve().parent / "statements.sha256"


def canonical_json() -> str:
    """Canonical serialization of the frozen benchmark (stable, sorted keys)."""
    return json.dumps(STATEMENTS, ensure_ascii=False, indent=2, sort_keys=True)


def freeze_marker() -> str:
    """sha256 hex digest of the canonical serialization."""
    return hashlib.sha256(canonical_json().encode("utf-8")).hexdigest()


def verify_frozen() -> bool:
    """True when statements.sha256 matches the current canonical serialization."""
    if not _FREEZE_PATH.exists():
        return False
    return _FREEZE_PATH.read_text(encoding="utf-8").strip() == freeze_marker()

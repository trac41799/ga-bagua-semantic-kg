"""Tagger: LLM description -> {role: strength} with disk cache.

Temperature 0. Responses are cached under data/cache/ (key = sha256 of
run_id|description) so re-runs are deterministic and token-free.
"""

import json
import os

from tags import parse_tags, TagError
from llm_client import LLMClient, load_api_config, cache_key


def tag(description, client=None, run_id=0, use_cache=True, offline=False,
        cache_dir="data/cache"):
    """Return {role: strength} for a concept description (LLM, temperature 0).

    - client: LLMClient or SimulatedLLM (default: real LLMClient via env config).
    - offline=True: never call the LLM; raise FileNotFoundError on cache miss.
    - The cached payload is re-validated through parse_tags on every read.
    """
    if client is None:
        cfg = load_api_config()
        if cfg is None:
            raise RuntimeError("no API key found; pass a client, or use --sim/--offline")
        base, key, model = cfg
        client = LLMClient(base, key, model=model)
    os.makedirs(cache_dir, exist_ok=True)
    path = os.path.join(cache_dir, f"tags_{cache_key(description, run_id)}.json")
    if use_cache and os.path.exists(path):
        with open(path, encoding="utf-8") as f:
            cached = json.load(f)
        return parse_tags(json.dumps(cached["tags"]))
    if offline:
        raise FileNotFoundError(
            f"offline mode: no cached tags for run_id={run_id} ({path})")
    text, _usage = client.tag(description, run_id=run_id)
    try:
        parsed = parse_tags(text)
    except TagError:
        text, _usage = client.tag(description, run_id=run_id)
        parsed = parse_tags(text)
    with open(path, "w", encoding="utf-8") as f:
        json.dump({"run_id": run_id, "tags": parsed, "text": text}, f, ensure_ascii=False)
    return parsed

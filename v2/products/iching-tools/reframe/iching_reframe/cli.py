"""CLI for iching_reframe.

Exit codes: 0 success, 1 LLM/runtime error, 2 usage/config error.
"""

import argparse
import json
import sys

from . import reframe
from .llm_client import DEFAULT_MODEL, LLMClient, SimulatedLLM, resolve_provider_config


def _make_llm(args):
    if args.sim:
        return SimulatedLLM()
    try:
        config = resolve_provider_config(
            api_key=args.api_key,
            provider=args.provider,
            model=args.model,
        )
    except ValueError as exc:
        print(f"iching_reframe: {exc}", file=sys.stderr)
        return None
    if config is None:
        print("iching_reframe: no API key found -- set DEEPSEEK_API_KEY or "
              "OPENROUTER_API_KEY, pass --api-key, or use --sim (SimulatedLLM, "
              "tests/demos only).", file=sys.stderr)
        return None
    return LLMClient(provider_config=config)


def main(argv=None):
    parser = argparse.ArgumentParser(
        prog="iching_reframe",
        description="Generate exactly 8 algebra-grounded reframes of a statement.")
    parser.add_argument("--statement", required=True,
                        help="statement to reframe (required)")
    parser.add_argument("--json", action="store_true",
                        help="emit a single JSON object on stdout")
    parser.add_argument("--api-key", default=None,
                        help="LLM API key (overrides DEEPSEEK_API_KEY / OPENROUTER_API_KEY)")
    parser.add_argument("--model", default=DEFAULT_MODEL,
                        help=f"LLM model (default {DEFAULT_MODEL})")
    parser.add_argument(
        "--provider", choices=("deepseek", "openrouter"), default=None,
        help="provider (defaults to DEEPSEEK_API_KEY, then OPENROUTER_API_KEY)",
    )
    parser.add_argument("--sim", action="store_true",
                        help="use SimulatedLLM (tests/demos only; no network)")
    args = parser.parse_args(argv)

    llm = _make_llm(args)
    if llm is None:
        return 2

    try:
        result = reframe(args.statement, llm)
    except Exception as exc:  # LLM/runtime error -> exit 1
        print(f"iching_reframe: error: {exc}", file=sys.stderr)
        return 1

    if args.json:
        print(json.dumps(result, ensure_ascii=True))
    else:
        for pos in result["positions"]:
            print(f"{pos['move']} | {pos['state']} -> {pos['reframe']}")
    return 0

"""Command-line interface for iching_statediff.

Exit codes: 0 success; 1 LLM/runtime/protocol error; 2 usage/config error.
"""

import argparse
import json
import sys

from . import ProtocolError, summarize
from .llm_client import LLMClient, SimulatedLLM, resolve_provider_config


def build_parser():
    p = argparse.ArgumentParser(
        prog="python -m iching_statediff",
        description="Summarize a state change as exactly 3 aspect lines "
                    "('aspect: before -> after').",
    )
    p.add_argument("--before", required=True, help="state before the change")
    p.add_argument("--after", required=True, help="state after the change")
    p.add_argument("--json", action="store_true", help="emit a single JSON object on stdout")
    p.add_argument("--api-key", default=None, help="override DEEPSEEK_API_KEY/OPENROUTER_API_KEY")
    p.add_argument("--model", default="deepseek-chat", help="LLM model (default: deepseek-chat)")
    p.add_argument(
        "--provider", choices=("deepseek", "openrouter"), default=None,
        help="provider (defaults to DEEPSEEK_API_KEY, then OPENROUTER_API_KEY)",
    )
    p.add_argument("--sim", action="store_true", help="use SimulatedLLM (tests/demos only)")
    return p


def _render_plain(result):
    lines = [f"Before: {result['before']}", f"After: {result['after']}", ""]
    lines += [f"{a['aspect']}: {a['before']} -> {a['after']}" for a in result["aspects"]]
    return "\n".join(lines) + "\n"


def main(argv=None):
    args = build_parser().parse_args(argv)
    if args.sim:
        llm = SimulatedLLM()
    else:
        try:
            cfg = resolve_provider_config(
                explicit_key=args.api_key,
                explicit_provider=args.provider,
                model=args.model,
            )
        except ValueError as exc:
            print(f"error: {exc}", file=sys.stderr)
            return 2
        if cfg is None:
            print("error: no API key (set DEEPSEEK_API_KEY or OPENROUTER_API_KEY, "
                  "pass --api-key, or use --sim)", file=sys.stderr)
            return 2
        llm = LLMClient(provider_config=cfg)
    try:
        result = summarize(args.before, args.after, llm)
    except (ProtocolError, RuntimeError, ValueError, OSError) as e:
        print(f"error: {e}", file=sys.stderr)
        return 1
    if args.json:
        print(json.dumps(result, ensure_ascii=True))
    else:
        sys.stdout.write(_render_plain(result))
    return 0

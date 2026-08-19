"""Command-line interface for iching_coverage.

Exit codes: 0 success; 1 LLM/runtime/protocol error; 2 usage/config error.
"""

import argparse
import json
import sys

from . import CoverageProtocolError, audit
from .llm_client import LLMClient, SimulatedLLM, resolve_api_key, resolve_provider_config

EXIT_OK = 0
EXIT_LLM_ERROR = 1
EXIT_USAGE = 2


def _client_from_provider_config(config):
    """Construct current clients while preserving routing for legacy clients."""
    try:
        return LLMClient(provider_config=config)
    except TypeError as exc:
        if "provider_config" not in str(exc):
            raise

    candidates = (
        {
            "api_key": config.api_key,
            "model": config.model,
            "base_url": config.api_base_url,
            "provider": config.provider,
        },
        {
            "api_key": config.api_key,
            "model": config.model,
            "base_url": config.api_base_url,
        },
        {
            "api_key": config.api_key,
            "model": config.model,
            "provider": config.provider,
        },
        {"api_key": config.api_key, "model": config.model},
    )
    last_error = None
    for candidate in candidates:
        try:
            return LLMClient(**candidate)
        except TypeError as exc:
            last_error = exc
            if not any(keyword in str(exc) for keyword in candidate):
                raise
    raise last_error


def build_parser():
    parser = argparse.ArgumentParser(
        prog="iching_coverage",
        description="Audit a plan against the 8-role completeness checklist.",
    )
    parser.add_argument("--task", required=True, help="the task the plan is for")
    parser.add_argument("--plan", required=True, help="the plan to audit and improve")
    parser.add_argument(
        "--json", action="store_true", help="emit a single JSON object on stdout"
    )
    parser.add_argument(
        "--api-key",
        dest="api_key",
        default=None,
        help="API key (overrides DEEPSEEK_API_KEY / OPENROUTER_API_KEY)",
    )
    parser.add_argument(
        "--model", default="deepseek-chat", help="LLM model (default: deepseek-chat)"
    )
    parser.add_argument(
        "--provider", choices=("deepseek", "openrouter"), default=None,
        help="provider (defaults to DEEPSEEK_API_KEY, then OPENROUTER_API_KEY)",
    )
    parser.add_argument(
        "--sim",
        action="store_true",
        help="use the deterministic SimulatedLLM (tests/demos only)",
    )
    return parser


def main(argv=None):
    args = build_parser().parse_args(argv)

    if args.sim:
        llm = SimulatedLLM()
    else:
        try:
            config = resolve_provider_config(args.api_key, args.provider, args.model)
        except ValueError as exc:
            print(f"error: {exc}", file=sys.stderr)
            return EXIT_USAGE
        if config is None:
            print(
                "error: no API key: set DEEPSEEK_API_KEY or OPENROUTER_API_KEY, "
                "pass --api-key, or use --sim",
                file=sys.stderr,
            )
            return EXIT_USAGE
        llm = _client_from_provider_config(config)

    try:
        result = audit(args.task, args.plan, llm)
    except CoverageProtocolError as exc:
        print(f"error: audit protocol failure: {exc}", file=sys.stderr)
        return EXIT_LLM_ERROR
    except Exception as exc:
        print(f"error: audit failed: {exc}", file=sys.stderr)
        return EXIT_LLM_ERROR

    if args.json:
        print(json.dumps(result, ensure_ascii=True))
    else:
        print("Task: " + result["task"])
        print()
        print("Original plan:")
        print(result["original_plan"])
        print()
        print("Audited plan:")
        print(result["audited_plan"])
        print()
        print("Checklist applied: " + ("yes" if result["checklist"] else "no"))

    return EXIT_OK

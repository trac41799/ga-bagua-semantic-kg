"""Deterministic promotion-evidence report generation for the v2 portfolio."""

from __future__ import annotations

from collections.abc import Iterable, Mapping
import argparse
import hashlib
import json
from pathlib import Path


ALLOWED_STATUSES = frozenset(
    {
        "PASS",
        "FAIL",
        "PENDING",
        "NOT_RUN",
        "INCONCLUSIVE",
        "MODEL_DEPENDENT",
        "SUPERSEDED",
    }
)
PRODUCT_ALLOWED_STATUSES = ALLOWED_STATUSES | {"PROMOTE_BETA", "PROMOTE_PRODUCT"}
GREEN_PRODUCT_STATUSES = frozenset({"PASS", "PROMOTE_BETA", "PROMOTE_PRODUCT"})

STATUS_FIELDS = (
    "claim_status",
    "execution_status",
    "evidence_status",
    "replication_status",
    "product_status",
)

REQUIRED_FIELDS = (
    "claim_id",
    "name",
    *STATUS_FIELDS,
    "primary_metric",
    "bar",
    "observed",
    "artifact",
    "protocol_id",
    "model_scope",
    "next_gate",
)

CLAIMS_PATH = Path(__file__).resolve().with_name("claims.json")


def validate_claims(claims: Iterable[Mapping[str, object]]) -> list[dict[str, object]]:
    """Validate and return claim rows without changing their statuses."""

    rows = list(claims)
    seen_ids: set[str] = set()
    validated: list[dict[str, object]] = []
    for row in rows:
        if not isinstance(row, Mapping):
            raise ValueError("each claim must be an object")
        missing = [field for field in REQUIRED_FIELDS if field not in row]
        if missing:
            raise ValueError(f"missing claim fields: {', '.join(missing)}")

        claim_id = row["claim_id"]
        if not isinstance(claim_id, str) or not claim_id:
            raise ValueError("claim_id must be a non-empty string")
        if claim_id in seen_ids:
            raise ValueError(f"duplicate claim_id: {claim_id}")
        seen_ids.add(claim_id)

        for field in STATUS_FIELDS:
            status = row[field]
            allowed = PRODUCT_ALLOWED_STATUSES if field == "product_status" else ALLOWED_STATUSES
            if not isinstance(status, str) or status not in allowed:
                raise ValueError(f"invalid {field}: {status}")

        claim_status = row["claim_status"]
        execution_status = row["execution_status"]
        evidence_status = row["evidence_status"]
        replication_status = row["replication_status"]
        product_status = row["product_status"]

        if claim_status == "PASS" and execution_status != "PASS":
            raise ValueError(f"{claim_id}: PASS claim requires execution_status PASS")
        if claim_status == "PASS" and evidence_status != "PASS":
            raise ValueError(f"{claim_id}: PASS claim requires evidence_status PASS")
        if replication_status == "PASS" and evidence_status != "PASS":
            raise ValueError(f"{claim_id}: replication PASS requires evidence_status PASS")
        if product_status in GREEN_PRODUCT_STATUSES:
            if claim_status != "PASS":
                raise ValueError(
                    f"{claim_id}: product_status cannot be green for {claim_status} claim"
                )
            if execution_status != "PASS" or evidence_status != "PASS":
                raise ValueError(
                    f"{claim_id}: green product_status requires execution_status and evidence_status PASS"
                )
            if replication_status != "PASS":
                raise ValueError(
                    f"{claim_id}: green product_status requires replication_status PASS"
                )

        validated.append(dict(row))
    return validated


def load_claims(path: str | Path = CLAIMS_PATH) -> list[dict[str, object]]:
    """Load the explicit claim matrix and validate it before use."""

    source = Path(path)
    try:
        data = json.loads(source.read_text(encoding="utf-8"))
    except OSError as exc:
        raise ValueError(f"cannot read claim source: {source}") from exc
    except json.JSONDecodeError as exc:
        raise ValueError(f"invalid claim source JSON: {source}") from exc
    if not isinstance(data, list):
        raise ValueError("claim source must contain a JSON array")
    return validate_claims(data)


def _repo_root() -> Path:
    return Path(__file__).resolve().parents[2]


def _text(value: object) -> str:
    return str(value).replace("|", "\\|").replace("\n", "<br>")


def _artifact_paths(claim: Mapping[str, object]) -> list[str]:
    raw = claim["artifact"]
    if isinstance(raw, str) and raw:
        paths = [raw]
    elif isinstance(raw, Iterable) and not isinstance(raw, (bytes, Mapping)):
        paths = [str(path) for path in raw]
    else:
        raise ValueError(f"{claim['claim_id']}: artifact must be a path or list of paths")

    if not paths or any(not path for path in paths):
        raise ValueError(f"{claim['claim_id']}: artifact paths must be non-empty")

    related = claim.get("related_artifacts", [])
    if isinstance(related, str):
        paths.append(related)
    elif isinstance(related, Iterable) and not isinstance(related, (bytes, Mapping)):
        paths.extend(str(path) for path in related)
    elif related:
        raise ValueError(f"{claim['claim_id']}: related_artifacts must contain paths")
    if any(not path for path in paths):
        raise ValueError(f"{claim['claim_id']}: artifact paths must be non-empty")
    return paths


def _artifact_hash(repo_root: Path, relative_path: str) -> str:
    path = Path(relative_path)
    if not path.is_absolute():
        path = repo_root / path
    try:
        return hashlib.sha256(path.read_bytes()).hexdigest()
    except OSError:
        raise ValueError(f"missing artifact: {relative_path}") from None


def _display_path(path: Path, repo_root: Path) -> str:
    """Prefer a stable repository-relative path, retaining outside paths exactly."""

    try:
        return path.resolve().relative_to(repo_root.resolve()).as_posix()
    except ValueError:
        return str(path)


def _status_table(
    claims: list[dict[str, object]],
    status_field: str,
    heading: str,
) -> list[str]:
    lines = [
        f"| {heading} | Count | Claim IDs |",
        "|---|---:|---|",
    ]
    for status in sorted({str(claim[status_field]) for claim in claims}):
        status_claims = [claim for claim in claims if claim[status_field] == status]
        ids = ", ".join(f"`{claim['claim_id']}`" for claim in status_claims)
        lines.append(f"| {status} | {len(status_claims)} | {ids} |")
    return lines


def render_report(
    claims: Iterable[Mapping[str, object]],
    repo_root: str | Path | None = None,
    claims_source: str | Path | None = None,
) -> str:
    """Render a stable Markdown report from explicit claim rows."""

    rows = sorted(validate_claims(claims), key=lambda claim: str(claim["claim_id"]))
    root = Path(repo_root) if repo_root is not None else _repo_root()
    source = Path(claims_source) if claims_source is not None else CLAIMS_PATH
    source_path = source if source.is_absolute() else Path.cwd() / source
    source_display = _display_path(source_path, root)
    source_hash = _artifact_hash(root, str(source_path))

    lines = [
        "# Promotion Evidence Report",
        "",
        "## Executive Decision",
        "",
        "- **Recommendation:** `TECHNICAL_BETA` only; do not promote the portfolio as a fully market-validated product.",
        "- **Yarrow:** deterministic internal correctness evidence is present, but `product_status` remains `PENDING` because external-user and task-time records are absent.",
        "- **R-08:** the replication is `FAIL` and the reframe claim is `MODEL_DEPENDENT`; it is not claim-green or model-general.",
        "- **Status discipline:** execution-gate results are mechanical evidence and do not rewrite claim or product status.",
        "",
        "## Status Transition Rules",
        "",
        "- A claim `PASS` requires both `execution_status` and `evidence_status` to be `PASS`.",
        "- Execution `PASS` never changes `claim_status`; deterministic product rows remain `PENDING` until host/product-fit evidence exists.",
        "- A green product status is rejected for `FAIL`, `INCONCLUSIVE`, `MODEL_DEPENDENT`, `PENDING`, `NOT_RUN`, or `SUPERSEDED` claims.",
        "- Any green product status, including `PROMOTE_BETA` and `PROMOTE_PRODUCT`, requires replication `PASS`; this report does not infer host fit from tests.",
        "",
        "## Claim Matrix",
        "",
        "| Claim ID | Name | Claim | Execution | Evidence | Replication | Product | Primary metric | Bar | Observed | Artifact | Protocol | Model scope | Next gate |",
        "|---|---|---|---|---|---|---|---|---|---|---|---|---|---|",
    ]
    for claim in rows:
        lines.append(
            "| "
            + " | ".join(
                _text(claim[field])
                for field in (
                    "claim_id",
                    "name",
                    "claim_status",
                    "execution_status",
                    "evidence_status",
                    "replication_status",
                    "product_status",
                    "primary_metric",
                    "bar",
                    "observed",
                    "artifact",
                    "protocol_id",
                    "model_scope",
                    "next_gate",
                )
            )
            + " |"
        )

    lines.extend(["", "## Blockers", ""])
    blockers = [
        claim
        for claim in rows
        if claim["claim_status"] != "PASS"
        or claim["product_status"] not in GREEN_PRODUCT_STATUSES
    ]
    for claim in blockers:
        lines.append(
            f"- `{claim['claim_id']}`: claim `{claim['claim_status']}`, product `{claim['product_status']}`; "
            f"next gate: {_text(claim['next_gate'])}."
        )

    lines.extend(["", "## Evidence Artifacts and Hashes", "", f"- Claim source `{source_display}`: `{source_hash}`", ""])
    lines.extend(["| Claim ID | Artifact | sha256 |", "|---|---|---|"])
    for claim in rows:
        for artifact in _artifact_paths(claim):
            digest = _artifact_hash(root, artifact)
            lines.append(f"| `{claim['claim_id']}` | `{_text(artifact)}` | `{digest}` |")

    lines.extend(["", "## Execution-Gate Summary", "", "Execution status is reported separately from claim status. A green test command is not a claim PASS.", ""])
    lines.extend(_status_table(rows, "execution_status", "Execution status"))

    lines.extend(["", "## Claim-Status Summary", ""])
    lines.extend(_status_table(rows, "claim_status", "Claim status"))

    lines.extend(["", "## Replication Status Summary", ""])
    lines.extend(_status_table(rows, "replication_status", "Replication status"))

    lines.extend(["", "## Product Status Summary", ""])
    lines.extend(_status_table(rows, "product_status", "Product status"))

    return "\n".join(lines) + "\n"


def write_report(
    output: str | Path,
    claims_path: str | Path = CLAIMS_PATH,
    repo_root: str | Path | None = None,
) -> str:
    """Write and return the deterministic report."""

    report = render_report(
        load_claims(claims_path),
        repo_root=repo_root,
        claims_source=claims_path,
    )
    Path(output).write_text(report, encoding="utf-8")
    return report


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", default=str(CLAIMS_PATH.with_name("promotion-report.md")))
    parser.add_argument("--claims", default=str(CLAIMS_PATH))
    args = parser.parse_args(argv)
    print(write_report(args.output, args.claims), end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

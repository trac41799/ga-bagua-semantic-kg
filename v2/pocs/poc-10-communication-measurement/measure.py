"""POC-10: calibration sets + historical and repaired comprehension-QA protocols.

Calibration: expert-authored known-good vs known-bad outputs per task (06 summaries,
09 explanations, 07 naming). The rater must discriminate them before any real verdict.
The historical QA builder is generated from planted ground truth and remains
for evidence compatibility. The repaired builder accepts only generated
summary context and aspect prompts so planted values stay evaluator-hidden.
"""

import hashlib
import json

# ---- calibration pairs (good, bad) — expert-authored, frozen ----
# 06: state-change summaries. bad = drops 2 of 3 planted deltas + filler.
CAL_06 = [
    ("cache hit ratio 94% -> 99%, p99 latency 120ms -> 95ms, error rate 0.2% -> 0.1%",
     "The system became faster overall. Cache behavior improved and errors went down somewhat."),
    ("replicas 3 -> 6, CPU utilization 80% -> 45%, queue depth 400 -> 40",
     "The service now runs with more capacity. Utilization is healthier."),
    ("connections 500 -> 250, read throughput 2k/s -> 5k/s, write latency 8ms -> 5ms",
     "Database performance changed. Throughput is better now."),
    ("rollout 10% -> 100%, error budget 60% -> 45%, alerts 12/day -> 3/day",
     "The rollout completed. Alerts decreased."),
    ("tokens/hour 1k -> 1.4k, revocation list 5k -> 12k, login failures 2% -> 0.4%",
     "Auth volumes grew. The failure rate improved."),
    ("inventory turns 4 -> 5.5, stockout rate 7% -> 2%, order fill time 3 days -> 1.5 days",
     "Operations improved across the board. Supply is more reliable."),
]

# 09: situation explanations. bad = scaffold present but NO factual answer/action.
CAL_09 = [
    ("坎 over 離: the moving line in the middle shows a shift in flow. The situation resolves toward balance.",
     "兑 over 震: lines 1 and 3 are moving, suggesting tension between old and new energies. The trigram pair advises patience."),
    ("Upper 坤, lower 乾: the outer ground receives the inner initiative; the change moves from the second line upward.",
     "乾 over 坤: six yang lines against six yin lines. The dynamic suggests a full cycle is completing."),
    ("震 over 坎: the initiating thunder stirs the abysmal water; the moving line is the first.",
     "離 over 艮: fire rests on mountain; the moving line is the third. The image suggests illumination arriving late."),
    ("艮 over 兑: stillness over joy; the moving line is the second, quieting the outlet.",
     "坎 over 巽: water over wind; two moving lines. The configuration is unstable and requires care."),
    ("巽 over 震: wind over thunder; the first line moves, carrying change outward.",
     "坤 over 兑: earth over lake; all three lines move. The moment is one of deep transformation."),
    ("兑 over 坎: joy over the abyss; the third line moves, surfacing the risk.",
     "震 over 離: thunder over fire; lines 1 and 2 move. Energy and clarity interact strongly."),
]

# 07: interaction explanations. bad = same math facts buried in jargon / wrong names.
CAL_07 = [
    ("the 2-way interaction between latency and cache_miss (coefficient 2.5)",
     "the pairwise coupling of component parameters L and C within the response surface manifold"),
    ("the 3-way interaction among latency, cache_miss, and concurrency (coefficient 0.9)",
     "the third-order mixed effect emergent from the tensor decomposition of the objective"),
    ("the 2-way interaction between batch_size and error_rate (coefficient -1.75)",
     "the bilinear term present in the fitted model's higher-order structure"),
    ("the 2-way interaction between latency and cache_miss dominates",
     "the dominant secondary effect arises from the product space of two latent drivers"),
    ("the 3-way interaction among latency, cache_miss, and concurrency is significant",
     "a triadic coupling manifests within the parameter grid's interaction hierarchy"),
    ("the 2-way interaction between batch_size and error_rate is negative",
     "the cross-term coefficient carries a negative orientation in the design space"),
]

CALIBRATION = {"06": CAL_06, "09": CAL_09, "07": CAL_07}


def freeze_hash():
    return hashlib.sha256(json.dumps(CALIBRATION, ensure_ascii=False).encode()).hexdigest()


# ---- comprehension QA (POC-06) ----

NO_GROUND_TRUTH_QUESTION_PROTOCOL = (
    "state_diff.comprehension.no_ground_truth_in_question_v1"
)


def no_ground_truth_questions(summary: str, aspect_names: list[str]) -> list[str]:
    """Build prompts whose only factual context is supplied separately as summary.

    The summary is validated as the reader context but is intentionally not
    interpolated into the question text. Planted values therefore remain
    hidden from the question and can be held only by the evaluator.
    """
    if not isinstance(summary, str) or not summary.strip():
        raise ValueError("summary must be a non-empty string")
    if not isinstance(aspect_names, list):
        raise ValueError("aspect_names must be a list")

    questions = []
    for aspect in aspect_names:
        if not isinstance(aspect, str) or not aspect.strip():
            raise ValueError("aspect names must be non-empty strings")
        questions.append(
            f"From the generated summary, what was the before value and what was "
            f"the after value for {aspect.strip()}? "
            "Answer 'before: <value>, after: <value>'."
        )
    return questions

def qa_questions(before, after, planted):
    """3 questions, one per planted aspect: 'what was the {aspect} before and after?'"""
    return [f"In the change from '{before}' to '{after}', what was the value of {aspect} "
            f"BEFORE and AFTER? Answer 'before: <value>, after: <value>'."
            for aspect, b, a in planted]


def qa_match_count(answer_text, planted):
    """Count planted aspects whose before and after values both appear."""
    t = answer_text.lower()
    matches = 0
    for aspect, b, a in planted:
        if str(b).lower() in t and str(a).lower() in t:
            matches += 1
    return matches


def qa_score(answer_text, planted):
    """Fraction of planted aspects whose before and after values appear."""
    return qa_match_count(answer_text, planted) / len(planted)


# ---- answer conveyance (POC-09) ----

def conveyance_score(reader_answer, factual_answer):
    """Fraction of distinctive tokens (len>4, non-stopword) of the factual answer
    present in the reader's answer. Objective, documented."""
    import re
    stop = {"should", "would", "could", "after", "before", "about", "there", "their", "these", "those", "which", "with", "from", "than", "that", "this", "then", "they", "have", "will"}
    toks = {w for w in re.findall(r"[a-z0-9]+", factual_answer.lower()) if len(w) > 4 and w not in stop}
    if not toks:
        return 0.0
    ra = reader_answer.lower()
    return sum(1 for w in toks if w in ra) / len(toks)

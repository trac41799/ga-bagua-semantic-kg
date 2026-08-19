"""Regression tests for the no-ground-truth-in-question protocol."""

import importlib.util
import os

import pytest

from measure import (NO_GROUND_TRUTH_QUESTION_PROTOCOL,
                     no_ground_truth_questions)


def _load_transitions():
    path = os.path.join(
        os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
        "..", "poc-06-state-diff", "transitions.py",
    )
    spec = importlib.util.spec_from_file_location("poc06_transitions_for_test", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module.TRANSITIONS


def test_new_question_does_not_contain_before_text():
    before = "cache hit ratio 94%, p99 latency 120ms, error rate 0.2%"
    after = "cache hit ratio 99%, p99 latency 95ms, error rate 0.1%"
    summary = "cache hit ratio: 94% -> 99%\np99 latency: 120ms -> 95ms\nerror rate: 0.2% -> 0.1%"
    aspect_names = ["cache hit ratio", "p99 latency", "error rate"]

    questions = no_ground_truth_questions(summary, aspect_names)

    assert len(questions) == 3
    assert NO_GROUND_TRUTH_QUESTION_PROTOCOL == (
        "state_diff.comprehension.no_ground_truth_in_question_v1"
    )
    for question in questions:
        assert before not in question
        assert after not in question
        assert all(value not in question
                   for value in ("94%", "99%", "120ms", "95ms", "0.2%", "0.1%"))


def test_runner_uses_clean_questions_without_transition_values(monkeypatch, tmp_path):
    """The runner must keep every before/after/planted value out of questions."""
    import run_all

    captured = []
    monkeypatch.setattr(run_all, "CLEAN_VERDICT_PATH",
                        str(tmp_path / "verdict-clean-v1.md"))
    monkeypatch.setattr(run_all, "CLEAN_CACHE_PATH",
                        str(tmp_path / "responses-clean-v1.json"))

    def capture_questions(summary, aspect_names):
        questions = no_ground_truth_questions(summary, aspect_names)
        captured.append((aspect_names, questions))
        return questions

    monkeypatch.setattr(run_all, "no_ground_truth_questions", capture_questions,
                        raising=False)
    monkeypatch.setattr(
        run_all, "qa_questions",
        lambda *args: pytest.fail("runner called the historical QA question builder"),
        raising=False,
    )

    assert run_all.main(real=False) == 0

    transitions = _load_transitions()
    transition_by_aspects = {
        tuple(aspect for aspect, _, _ in planted): (before, after, planted)
        for _, _, before, after, planted in transitions
    }
    assert len(captured) == len(transitions) * 2
    assert sum(len(questions) for _, questions in captured) == len(transitions) * 2 * 3

    for aspect_names, questions in captured:
        before, after, planted = transition_by_aspects[tuple(aspect_names)]
        assert len(questions) == len(planted)
        for question in questions:
            assert before not in question
            assert after not in question
            assert all(str(value) not in question
                       for _, before_value, after_value in planted
                       for value in (before_value, after_value))

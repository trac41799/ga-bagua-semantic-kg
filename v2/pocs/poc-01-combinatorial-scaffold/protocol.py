"""POC-01 protocol: LLM plan -> validated op list -> exact execution -> LLM interpret."""

import json

from calculator import (BLADE_NAMES, BITS_TO_STATE, TRIGRAM_PINYIN, combine,
                        complement, count_blades, double_flip, flip, format_state,
                        grade, product, resolve)

OPS = {
    "flip": {"args": ["state", "line"]},
    "double_flip": {"args": ["state", "lines"]},
    "complement": {"args": ["state"]},
    "grade": {"args": ["state"]},
    "product": {"args": ["a", "b"]},
    "combine": {"args": ["upper", "lower"]},
    "count_blades": {"args": ["grade_k"]},
}

SYSTEM_PROMPT = (
    "You translate combinatorial reasoning problems into exact operations on a Cl(3) "
    "blade calculator. State names: blades '1','e1','e2','e3','e12','e23','e31','e123' "
    "(sign-prefixed like '-e1' allowed), or trigrams by pinyin ('kun','gen','kan','xun',"
    "'zhen','li','dui','qian') or 3-bit patterns ('010').\n"
    "Available ops (JSON objects):\n"
    "- {\"op\":\"flip\",\"state\":<name>,\"line\":0|1|2}   flip one line\n"
    "- {\"op\":\"double_flip\",\"state\":<name>,\"lines\":[i,j]}\n"
    "- {\"op\":\"complement\",\"state\":<name>}            antipode (Hodge dual)\n"
    "- {\"op\":\"grade\",\"state\":<name>}                 number of yang lines\n"
    "- {\"op\":\"product\",\"a\":<name>,\"b\":<name>}      geometric product result\n"
    "- {\"op\":\"combine\",\"upper\":<name>,\"lower\":<name>}  hexagram code\n"
    "- {\"op\":\"count_blades\",\"grade_k\":0|1|2|3}       number of blades of grade k\n"
    "Output ONLY a JSON array of op objects that exactly computes the answer. "
    "If the question is a pure fact question, output the op that yields the fact."
)


class ProtocolError(ValueError):
    pass


def plan_prompt(problem_text):
    return [
        {"role": "system", "content": SYSTEM_PROMPT},
        {"role": "user", "content": f"Problem: {problem_text}\nOutput the JSON op array."},
    ]


def parse_plan(text):
    t = text.strip().strip("```").strip()
    if t.startswith("json"):
        t = t[4:].strip()
    try:
        data = json.loads(t)
    except json.JSONDecodeError as e:
        raise ProtocolError(f"plan not JSON: {e}") from e
    if not isinstance(data, list) or not data:
        raise ProtocolError("plan must be a non-empty JSON array")
    for op in data:
        if not isinstance(op, dict) or "op" not in op:
            raise ProtocolError(f"invalid op entry: {op!r}")
        if op["op"] not in OPS:
            raise ProtocolError(f"unknown op: {op['op']!r}")
    return data


def execute(ops):
    """Strict execution: any invalid arg raises ProtocolError; nothing partial."""
    result = None
    for op in ops:
        kind = op["op"]
        if kind == "flip":
            result = flip(resolve(op["state"]), int(op["line"]))
            result = format_state(result)
        elif kind == "double_flip":
            result = format_state(double_flip(resolve(op["state"]),
                                              [int(x) for x in op["lines"]]))
        elif kind == "complement":
            result = format_state(complement(resolve(op["state"])))
        elif kind == "grade":
            result = str(grade(resolve(op["state"])))
        elif kind == "product":
            result = product(op["a"], op["b"])
        elif kind == "combine":
            result = str(combine(op["upper"], op["lower"]))
        elif kind == "count_blades":
            result = str(count_blades(int(op["grade_k"])))
    return result


def interpret_prompt(problem_text, calculator_result):
    return [
        {"role": "system", "content": "The calculator produced the exact intermediate result "
                                      "below. Answer the original problem in the required form "
                                      "(a number, a blade name like 'e123' or '-e1', a 3-bit "
                                      "pattern, or a hexagram name). Output ONLY the answer."},
        {"role": "user", "content": f"Problem: {problem_text}\nCalculator result: {calculator_result}"},
    ]


def parse_answer(text):
    return text.strip().strip("`").strip()

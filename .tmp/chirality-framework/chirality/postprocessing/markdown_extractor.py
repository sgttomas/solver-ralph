"""
Decoupled normalization: convert relaxed markdown/transcript outputs to strict JSON.

Strategy:
- Consume the relaxed Phase 1 run output (matrices with {"content": "..."} per stage).
- For each matrix/stage pair that has free-form content, call the JSON Normalizer
  system prompt with the stage text as input and the strict schema as response_format.
- Keep deterministic data-drops (transform/transposes) as-is.

This enables a markdown-first transcript while producing schema-accurate JSON for DB.
"""

from typing import Dict, Any, List, Tuple
import hashlib

from ..infrastructure.llm.openai_adapter import call_responses


def _load_normalizer_instructions() -> str:
    try:
        from pathlib import Path
        p = Path("chirality/infrastructure/prompts/assets/phase1/normalize_to_json.md")
        return p.read_text(encoding="utf-8")
    except Exception:
        return (
            "You convert the provided plain text into a single JSON object that matches the schema exactly. "
            "Extract only; do not add or infer; no extra keys."
        )


def _get_schema(matrix: str, stage: str) -> Dict[str, Any]:
    from ..infrastructure.validation.json_schema_converter import get_strict_response_format
    return get_strict_response_format(matrix, stage)


def _try_parse_markdown_table(matrix: str, stage: str, text: str) -> Dict[str, Any] | None:
    """Best-effort GitHub-style markdown table parser to reduce LLM calls.

    Parses a simple pipe table and maps cells into elements with canonical row/col labels.
    Returns a dict with rows, cols, elements when shape matches expected dims; else None.
    """
    try:
        from ..domain.matrices.canonical import get_matrix_info
        info = get_matrix_info(matrix)
        rows_expected = int(info["rows"])  # count
        cols_expected = int(info["cols"])  # count
        row_labels = info["row_labels"]
        col_labels = info["col_labels"]
    except Exception:
        return None

    lines = [ln.rstrip() for ln in text.splitlines() if '|' in ln]
    if len(lines) < 2:
        return None
    # Split into rows; detect header and alignment
    parsed_rows: List[List[str]] = []
    for ln in lines:
        cells = [c.strip() for c in ln.strip('|').split('|')]
        parsed_rows.append(cells)

    # Identify alignment row index (---, :---:, etc.)
    align_idx = None
    for i, row in enumerate(parsed_rows[:3]):  # look at first few rows
        joined = ''.join(row)
        if joined and set(joined) <= set('-: '):
            align_idx = i
            break

    # Header row is line before alignment, if present
    header: List[str] | None = None
    body_rows = parsed_rows
    if align_idx is not None and align_idx > 0:
        header = parsed_rows[align_idx - 1]
        # Body starts after alignment row
        body_rows = parsed_rows[align_idx + 1:]
    else:
        # If no alignment row, try to treat first row as header when it matches col labels
        if parsed_rows:
            header = parsed_rows[0]
            body_rows = parsed_rows[1:]

    # Check if header matches canonical columns (either exact or with a leading blank/row label column)
    has_leading_label_col = False
    header_ok = False
    if header:
        if header == col_labels:
            header_ok = True
        elif len(header) == len(col_labels) + 1 and header[1:] == col_labels:
            header_ok = True
            has_leading_label_col = True

    # If header doesn't match, fall back to normalizer
    if not header_ok:
        return None

    # Parse body rows; allow an optional leading row label column that matches canonical row labels
    cleaned: List[List[str]] = []
    for i, r in enumerate(body_rows):
        # Drop empty rows
        if all(c == '' for c in r):
            continue
        # Accept either exact number of columns or exact + 1 (row label)
        if len(r) == cols_expected + 1:
            # If there is a leading label, ensure it matches expected row label at this index
            if r[0] != row_labels[len(cleaned)] if len(cleaned) < len(row_labels) else True:
                # Label mismatch; bail out
                return None
            cleaned.append(r[1:])
        elif len(r) == cols_expected:
            cleaned.append(r)
        else:
            # Unexpected shape; let LLM normalizer handle
            return None
        if len(cleaned) == rows_expected:
            break

    if len(cleaned) != rows_expected:
        return None

    return {"rows": row_labels, "cols": col_labels, "elements": cleaned}

def _validate_strict(payload: Dict[str, Any], matrix: str, stage: str) -> Tuple[bool, List[str]]:
    from ..infrastructure.validation.json_schema_converter import validate_stage_response_strict
    return validate_stage_response_strict(payload, matrix, stage)


def _summarize_errors(errors: List[str]) -> str:
    # Compact, human-readable summary for a single retry prompt
    if not errors:
        return ""
    prefix = "Validation failed. Correct these strictly: "
    items = "; ".join(e.split("\n", 1)[0] for e in errors[:8])
    if len(errors) > 8:
        items += f"; +{len(errors) - 8} more"
    return prefix + items + ". Do not add keys or invent values. Extract-only."


def extract_structured_from_relaxed(relaxed_output: Dict[str, Any]) -> Dict[str, Any]:
    """
    Normalize a relaxed Phase 1 run into strict JSON per stage using the normalizer.

    Args:
        relaxed_output: Final output dict returned by DialogueOrchestrator in relaxed mode

    Returns:
        Dict with the same matrix keys, each stage normalized to strict JSON when possible.
    """
    matrices = relaxed_output.get("matrices", {}) if isinstance(relaxed_output, dict) else {}
    instructions = _load_normalizer_instructions()

    structured: Dict[str, Any] = {
        "meta": relaxed_output.get("meta", {}),
        "matrices": {},
        "validation": {},
    }

    for matrix_name, stages in matrices.items():
        out_stages: Dict[str, Any] = {}
        if not isinstance(stages, dict):
            structured["matrices"][matrix_name] = stages
            continue
        for stage_name, payload in stages.items():
            # Keep deterministic data-drops or already-structured payloads
            if isinstance(payload, dict) and ("rows" in payload and "cols" in payload and ("elements" in payload or "values_json" in payload)):
                out_stages[stage_name] = payload
                structured["validation"].setdefault(matrix_name, {})[stage_name] = {"ok": True, "errors": []}
                continue

            # Normalize free-form content via strict schema
            text = ""
            if isinstance(payload, dict) and "content" in payload:
                text = str(payload.get("content", ""))
            elif isinstance(payload, str):
                text = payload

            if not text:
                out_stages[stage_name] = {"error": "empty_content"}
                continue

            # Deterministic fast-path: parse markdown tables if shape matches
            parsed = _try_parse_markdown_table(matrix_name, stage_name, text)
            if parsed is not None:
                ok_tbl, errs_tbl = _validate_strict(parsed, matrix_name, stage_name)
                if ok_tbl:
                    stage_hash = hashlib.sha256(text.encode("utf-8")).hexdigest()
                    parsed.setdefault("_provenance", {})["stage_a_sha256"] = stage_hash
                    out_stages[stage_name] = parsed
                    structured["validation"].setdefault(matrix_name, {})[stage_name] = {"ok": True, "errors": []}
                    continue

            try:
                response_format = _get_schema(matrix_name, stage_name)
            except Exception as e:
                out_stages[stage_name] = {"error": f"schema_lookup_failed: {e}"}
                structured["validation"].setdefault(matrix_name, {})[stage_name] = {"ok": False, "errors": [str(e)]}
                continue

            # Build typed input parts for consistency
            typed_input = [{"role": "user", "content": [{"type": "input_text", "text": text}]}]

            # First pass normalization
            try:
                resp = call_responses(
                    instructions=instructions,
                    input=typed_input,
                    response_format=response_format,
                    expects_json=True,
                    store=False,
                    temperature=0.2,
                    top_p=1.0,
                    max_output_tokens=2000,
                )
                out_text = resp.get("output_text", "")
                import json as _json
                normalized = _json.loads(out_text) if out_text else {"error": "empty_normalizer_output"}
            except Exception as e:
                normalized = {"error": f"normalization_failed: {e}"}

            # Validate
            ok, errors = (False, ["no_output"]) if isinstance(normalized, dict) and normalized.get("error") else _validate_strict(normalized, matrix_name, stage_name)

            if not ok:
                note = _summarize_errors(errors if isinstance(errors, list) else [str(errors)])
                # Retry once, with correction note appended to instructions
                try:
                    retry_instructions = instructions + "\n\n" + note
                    resp2 = call_responses(
                        instructions=retry_instructions,
                        input=typed_input,
                        response_format=response_format,
                        expects_json=True,
                        store=False,
                        temperature=0.2,
                        top_p=1.0,
                        max_output_tokens=2000,
                    )
                    out_text2 = resp2.get("output_text", "")
                    import json as _json
                    normalized2 = _json.loads(out_text2) if out_text2 else {"error": "empty_normalizer_output"}
                    ok2, errors2 = _validate_strict(normalized2, matrix_name, stage_name)
                    if ok2:
                        normalized = normalized2
                        ok, errors = ok2, errors2
                except Exception as e:
                    errors = [f"retry_failed: {e}"] + (errors if isinstance(errors, list) else [str(errors)])

            # Attach provenance hash
            stage_hash = hashlib.sha256(text.encode("utf-8")).hexdigest()
            if isinstance(normalized, dict):
                normalized.setdefault("_provenance", {})["stage_a_sha256"] = stage_hash
            out_stages[stage_name] = normalized
            structured["validation"].setdefault(matrix_name, {})[stage_name] = {"ok": bool(ok), "errors": errors if isinstance(errors, list) else [str(errors)]}

        structured["matrices"][matrix_name] = out_stages

    return structured

"""
JSON tail templates for Phase 1 dialogue.

These exact strings are appended to prompts to ensure JSON-only responses.
No markdown, no prose - just the contract specification.
"""

# Base template for general matrix operations
TAIL_TEMPLATE = 'Return JSON only using this contract: {{"artifact":"matrix","name":"{name}","station":"{station}","rows":{rows},"cols":{cols},"step":"{step}","op":"{op}","elements":{elements}}}'

# Generic tail for generating a full matrix of lenses in one shot
TAIL_LENSES_GENERATE = (
    'Return JSON only using this contract: '
    '{"artifact":"lenses",'
    '"station":"<station>",'
    '"rows":["row1","row2","row3"],'
    '"cols":["col1","col2","col3","col4"],'
    '"lenses":[["lens1","lens2","lens3","lens4"],["lens5","lens6","lens7","lens8"],["lens9","lens10","lens11","lens12"]]}'
)

# Specific tails for each matrix/step combination

# Matrix C tails
TAIL_C_MECH = 'Return JSON only using this contract: {"artifact":"matrix","name":"C","station":"problem statement","rows":["normative","operative","iterative"],"cols":["necessity (vs contingency)","sufficiency","completeness","consistency"],"step":"mechanical","op":"dot","elements":[[...],[...],[...]]}'

TAIL_C_INTERP = 'Return JSON only using this contract: {"artifact":"matrix","name":"C","station":"problem statement","rows":["normative","operative","iterative"],"cols":["necessity (vs contingency)","sufficiency","completeness","consistency"],"step":"interpreted","op":"dot","elements":[[...],[...],[...]]}'

TAIL_C_LENSES = 'Return JSON only using this contract: {"artifact":"matrix","name":"C","station":"problem statement","rows":["normative","operative","iterative"],"cols":["necessity (vs contingency)","sufficiency","completeness","consistency"],"step":"lenses","lenses":[[...],[...],[...]]}'

TAIL_C_LENSED = 'Return JSON only using this contract: {"artifact":"matrix","name":"C","station":"problem statement","rows":["normative","operative","iterative"],"cols":["necessity (vs contingency)","sufficiency","completeness","consistency"],"step":"lensed","op":"dot","elements":[[...],[...],[...]]}'

# Matrix J (base canonical)
TAIL_J_BASE = 'Return JSON only using this contract: {"artifact":"matrix","name":"J","station":"requirements","rows":["data","information","knowledge"],"cols":["necessity (vs contingency)","sufficiency","completeness","consistency"],"step":"base","elements":[[...],[...],[...]]}'

# Matrix F tails
TAIL_F_MECH = 'Return JSON only using this contract: {"artifact":"matrix","name":"F","station":"requirements","rows":["data","information","knowledge"],"cols":["necessity (vs contingency)","sufficiency","completeness","consistency"],"step":"mechanical","op":"hadamard","elements":[[...],[...],[...]]}'

TAIL_F_INTERP = 'Return JSON only using this contract: {"artifact":"matrix","name":"F","station":"requirements","rows":["data","information","knowledge"],"cols":["necessity (vs contingency)","sufficiency","completeness","consistency"],"step":"interpreted","op":"hadamard","elements":[[...],[...],[...]]}'

TAIL_F_LENSES = 'Return JSON only using this contract: {"artifact":"matrix","name":"F","station":"requirements","rows":["data","information","knowledge"],"cols":["necessity (vs contingency)","sufficiency","completeness","consistency"],"step":"lenses","lenses":[[...],[...],[...]]}'

TAIL_F_LENSED = 'Return JSON only using this contract: {"artifact":"matrix","name":"F","station":"requirements","rows":["data","information","knowledge"],"cols":["necessity (vs contingency)","sufficiency","completeness","consistency"],"step":"lensed","op":"hadamard","elements":[[...],[...],[...]]}'

# Matrix D tails
TAIL_D_MECH = 'Return JSON only using this contract: {"artifact":"matrix","name":"D","station":"objectives","rows":["normative","operative","iterative"],"cols":["guiding","applying","judging","reflecting"],"step":"mechanical","op":"add","elements":[[...],[...],[...]]}'

TAIL_D_INTERP = 'Return JSON only using this contract: {"artifact":"matrix","name":"D","station":"objectives","rows":["normative","operative","iterative"],"cols":["guiding","applying","judging","reflecting"],"step":"interpreted","op":"add","elements":[[...],[...],[...]]}'

TAIL_D_CONSTRUCTED = 'Return JSON only using this contract: {"artifact":"matrix","name":"D","station":"objectives","rows":["normative","operative","iterative"],"cols":["guiding","applying","judging","reflecting"],"step":"constructed","op":"add","elements":[[...],[...],[...]]}'

TAIL_D_LENSES = 'Return JSON only using this contract: {"artifact":"matrix","name":"D","station":"objectives","rows":["normative","operative","iterative"],"cols":["guiding","applying","judging","reflecting"],"step":"lenses","lenses":[[...],[...],[...]]}'

TAIL_D_LENSED = 'Return JSON only using this contract: {"artifact":"matrix","name":"D","station":"objectives","rows":["normative","operative","iterative"],"cols":["guiding","applying","judging","reflecting"],"step":"lensed","op":"add","elements":[[...],[...],[...]]}'

# Matrix K (transpose of D)
TAIL_K_TRANSPOSE = 'Return JSON only using this contract: {"artifact":"matrix","name":"K","station":"objectives","rows":["guiding","applying","judging","reflecting"],"cols":["normative","operative","iterative"],"step":"transpose","op":"transpose","elements":[[...],[...],[...],[...]]}'

# Matrix X tails
TAIL_X_MECH = 'Return JSON only using this contract: {"artifact":"matrix","name":"X","station":"verification","rows":["guiding","applying","judging","reflecting"],"cols":["necessity (vs contingency)","sufficiency","completeness","consistency"],"step":"mechanical","op":"dot","elements":[[...],[...],[...],[...]]}'

TAIL_X_INTERP = 'Return JSON only using this contract: {"artifact":"matrix","name":"X","station":"verification","rows":["guiding","applying","judging","reflecting"],"cols":["necessity (vs contingency)","sufficiency","completeness","consistency"],"step":"interpreted","op":"dot","elements":[[...],[...],[...],[...]]}'

TAIL_X_LENSES = 'Return JSON only using this contract: {"artifact":"matrix","name":"X","station":"verification","rows":["guiding","applying","judging","reflecting"],"cols":["necessity (vs contingency)","sufficiency","completeness","consistency"],"step":"lenses","lenses":[[...],[...],[...],[...]]}'

TAIL_X_LENSED = 'Return JSON only using this contract: {"artifact":"matrix","name":"X","station":"verification","rows":["guiding","applying","judging","reflecting"],"cols":["necessity (vs contingency)","sufficiency","completeness","consistency"],"step":"lensed","op":"dot","elements":[[...],[...],[...],[...]]}'

# Matrix Z tails (station shift from X, maintains 4x4 structure)
TAIL_Z_INTERPRETED = 'Return JSON only using this contract: {"artifact":"matrix","name":"Z","station":"validation","rows":["guiding","applying","judging","reflecting"],"cols":["necessity (vs contingency)","sufficiency","completeness","consistency"],"step":"interpreted","op":"shift","elements":[[...],[...],[...],[...]]}'

TAIL_Z_LENSED = 'Return JSON only using this contract: {"artifact":"matrix","name":"Z","station":"validation","rows":["guiding","applying","judging","reflecting"],"cols":["necessity (vs contingency)","sufficiency","completeness","consistency"],"step":"lensed","op":"shift","elements":[[...],[...],[...],[...]]}'

TAIL_Z_PRINCIPLES = 'Return JSON only using this contract: {"artifact":"principles","station":"validation","source":"Z","cols":["necessity (vs contingency)","sufficiency","completeness","consistency"],"principles":["...","...","...","..."]}' 

# Matrix G (slice of Z)
TAIL_G_BASE = 'Return JSON only using this contract: {"artifact":"matrix","name":"G","station":"evaluation","rows":["guiding","applying","judging"],"cols":["necessity (vs contingency)","sufficiency","completeness","consistency"],"step":"base","elements":[[...],[...],[...]]}'

# Array P (row 3 of Z)
TAIL_P_BASE = 'Return JSON only using this contract: {"artifact":"matrix","name":"P","station":"reflection","rows":["reflecting"],"cols":["necessity (vs contingency)","sufficiency","completeness","consistency"],"step":"base","elements":[[...]]}'

# Matrix T (transpose of J)
TAIL_T_TRANSPOSE = 'Return JSON only using this contract: {"artifact":"matrix","name":"T","station":"requirements","rows":["necessity (vs contingency)","sufficiency","completeness","consistency"],"cols":["data","information","knowledge"],"step":"transpose","op":"transpose","elements":[[...],[...],[...],[...]]}'

# Matrix E tails
TAIL_E_MECH = 'Return JSON only using this contract: {"artifact":"matrix","name":"E","station":"evaluation","rows":["guiding","applying","judging"],"cols":["data","information","knowledge"],"step":"mechanical","op":"dot","elements":[[...],[...],[...]]}'

TAIL_E_INTERP = 'Return JSON only using this contract: {"artifact":"matrix","name":"E","station":"evaluation","rows":["guiding","applying","judging"],"cols":["data","information","knowledge"],"step":"interpreted","op":"dot","elements":[[...],[...],[...]]}'

TAIL_E_LENSES = 'Return JSON only using this contract: {"artifact":"matrix","name":"E","station":"evaluation","rows":["guiding","applying","judging"],"cols":["data","information","knowledge"],"step":"lenses","lenses":[[...],[...],[...]]}'

TAIL_E_LENSED = 'Return JSON only using this contract: {"artifact":"matrix","name":"E","station":"evaluation","rows":["guiding","applying","judging"],"cols":["data","information","knowledge"],"step":"lensed","op":"dot","elements":[[...],[...],[...]]}'

# Final aggregator tail
TAIL_AGGREGATOR = 'Produce a single JSON object exactly matching this schema: {"matrices":{"C":{...},"J":{...},"F":{...},"D":{...},"K":{...},"X":{...},"Z":{...},"G":{...},"P":{...},"T":{...},"E":{...}},"principles":{"from":"Z","items":[...]}}. Return only JSON.'


# Helper function to get tail by matrix and step
def get_tail(matrix: str, step: str) -> str:
    """Get the appropriate JSON tail for a matrix and step."""
    tail_map = {
        ("C", "mechanical"): TAIL_C_MECH,
        ("C", "interpreted"): TAIL_C_INTERP,
        ("C", "lenses"): TAIL_C_LENSES,
        ("C", "lensed"): TAIL_C_LENSED,
        ("J", "base"): TAIL_J_BASE,
        ("F", "mechanical"): TAIL_F_MECH,
        ("F", "interpreted"): TAIL_F_INTERP,
        ("F", "lenses"): TAIL_F_LENSES,
        ("F", "lensed"): TAIL_F_LENSED,
        ("D", "mechanical"): TAIL_D_MECH,
        ("D", "interpreted"): TAIL_D_INTERP,
        ("D", "constructed"): TAIL_D_CONSTRUCTED,
        ("D", "lenses"): TAIL_D_LENSES,
        ("D", "lensed"): TAIL_D_LENSED,
        ("K", "transpose"): TAIL_K_TRANSPOSE,
        ("X", "mechanical"): TAIL_X_MECH,
        ("X", "interpreted"): TAIL_X_INTERP,
        ("X", "lenses"): TAIL_X_LENSES,
        ("X", "lensed"): TAIL_X_LENSED,
        ("Z", "lensed"): TAIL_Z_LENSED,
        ("Z", "principles"): TAIL_Z_PRINCIPLES,
        ("G", "base"): TAIL_G_BASE,
        ("P", "base"): TAIL_P_BASE,
        ("T", "transpose"): TAIL_T_TRANSPOSE,
        ("E", "mechanical"): TAIL_E_MECH,
        ("E", "interpreted"): TAIL_E_INTERP,
        ("E", "lenses"): TAIL_E_LENSES,
        ("E", "lensed"): TAIL_E_LENSED,
        ("aggregator", ""): TAIL_AGGREGATOR,
    }

    key = (matrix, step)
    if key not in tail_map:
        raise ValueError(f"No tail defined for matrix={matrix}, step={step}")

    return tail_map[key]

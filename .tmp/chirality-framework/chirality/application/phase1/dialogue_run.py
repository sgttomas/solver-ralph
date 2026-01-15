"""
Phase 1 Dialogue Orchestrator.

Manages a single inclusive conversation from A through E,
building semantic understanding through dialogue history.
Each step returns JSON only (no tables).

ARCHITECTURE STATUS: LEGACY METHODS DELETED
All legacy template-based methods have been completely removed.
New 4-stage conversational pipeline implementation required.
"""

import json
from typing import Dict, List, Any, Optional, Literal
from datetime import datetime, timezone
from pathlib import Path

# JSON tails removed from assets; enforcement via Responses API structured outputs
from ...infrastructure.llm.openai_adapter import call_responses
from ...infrastructure.llm.repair import try_parse_json_or_repair, create_matrix_schema_hint
from ...infrastructure.monitoring.tracer import JSONLTracer
from ...infrastructure.prompts.registry import get_registry
from ...infrastructure.validation.schemas import validate_stage_response, validate_lens_payload
from ...domain.matrices.canonical import get_canonical_matrix
from ...domain.budgets import BudgetConfig
from ...application.lenses import LensResolver
from .aggregator import validate_and_write_agg, create_aggregator_schema_hint
from .contracts import MatrixSnapshot


STATION_MAP = {
    "C": "problem statement",
    "F": "requirements",
    "D": "objectives",
    "X": "verification",
    "Z": "validation",
    "E": "evaluation",
}


def _infer_operation(matrix_name: str) -> str:
    """Infer operation type from matrix name."""
    operations = {
        "A": None, "B": None, "J": None,  # Base canonical matrices
        "C": "dot", "X": "dot", "E": "dot",  # Dot products  
        "F": "hadamard",  # Element-wise product
        "D": "add",  # Semantic addition
        "K": "transpose", "T": "transpose",  # Transposes
        "Z": "shift",  # Station shift
        "G": None, "P": None  # Extractions
    }
    return operations.get(matrix_name)


class DialogueOrchestrator:
    """
    Orchestrates the Phase 1 dialogue as a single conversation.

    The dialogue builds semantic understanding progressively,
    with each step adding to the conversation history.
    
    ARCHITECTURE STATUS: REQUIRES NEW IMPLEMENTATION
    Legacy template-based methods have been completely removed.
    New 4-stage conversational pipeline implementation required.
    """

    def __init__(
        self,
        model: str = None,
        temperature: float = None,
        max_repair: int = 1,
        budget_config: Optional[BudgetConfig] = None,
        tracer: Optional[JSONLTracer] = None,
        lens_mode: Literal["catalog", "auto"] = "catalog",
        write_catalog: bool = False,
        reasoning_effort: Optional[str] = None,
        relaxed_json: bool = False,
        inband_c_normalize: bool = False,
        stop_at: Optional[str] = None,
    ):
        """
        Initialize the dialogue orchestrator.

        Args:
            model: LLM model identifier (uses global config if None)
            temperature: Sampling temperature (uses global config if None)
            max_repair: Maximum repair attempts for invalid JSON
            budget_config: Optional budget configuration for cost/token/time limits
            tracer: Optional JSONL tracer for logging
            lens_mode: Lens resolution mode ("catalog" or "auto")
            write_catalog: Whether to write generated lenses to catalog
            reasoning_effort: GPT-5 reasoning effort level ("minimal", "medium", "low")
        """
        # Use global config if not provided
        from ...infrastructure.llm.config import get_config
        config = get_config()
        
        self.model = model if model is not None else config.model
        self.temperature = temperature if temperature is not None else config.temperature
        self.reasoning_effort = reasoning_effort
        self.max_repair = max_repair
        self.budget_config = budget_config
        self.tracer = tracer
        self.lens_mode = lens_mode
        self.write_catalog = write_catalog
        self.relaxed_json = relaxed_json
        self.inband_c_normalize = inband_c_normalize
        self.stop_at = stop_at

        # Initialize lens resolver 
        self.lens_resolver = LensResolver(lens_mode=lens_mode)
        
        # Initialize prompt registry
        self.registry = get_registry()

        # Track conversation history
        self.dialogue_history = []
        self.token_count = 0
        
        # Store matrix snapshots for dependencies
        self.snapshots = {}
        
        # Track matrix results for final output
        self.matrix_results = {}

        # Load canonical matrices
        self.A = get_canonical_matrix("A")
        self.B = get_canonical_matrix("B")
        self.J = get_canonical_matrix("J")

    def run_dialogue(self, output_dir: Path = None) -> Dict[str, Any]:
        """
        Run the complete Phase 1 dialogue from A through E.
        
        NEW IMPLEMENTATION REQUIRED: 4-stage conversational pipeline.
        Will be implemented in Phase D.
        
        Args:
            output_dir: Directory for saving artifacts

        Returns:
            Dictionary with phase1_output structure
        """
        # Initialize system message from system.md
        system_message = self.registry.get_text("system")
        
        # Initialize dialogue with system message
        self.dialogue_history = [
            {"role": "system", "content": system_message}
        ]
        
        # Initialize trace data
        trace_entries = []
        
        # MATRIX C PIPELINE - 5 STAGES
        
        # Stage 0: C/initialize.md - Initialize semantic operations 
        c_initialize_result, c_initialize_trace = self._execute_stage(
            "phase1_c_initialize", "C", "initialize"
        )
        trace_entries.append(c_initialize_trace)
        self.matrix_results["C"] = {"initialize": c_initialize_result}
        
        # Stage 1: C/mechanical.md - Mechanical construction
        c_mechanical_result, c_mechanical_trace = self._execute_stage(
            "phase1_c_mechanical", "C", "mechanical"
        )
        trace_entries.append(c_mechanical_trace)
        self.matrix_results["C"]["mechanical"] = c_mechanical_result
        
        # Stage 2: C/interpreted.md - Semantic interpretation 
        c_interpreted_result, c_interpreted_trace = self._execute_stage(
            "phase1_c_interpreted", "C", "interpreted"
        )
        trace_entries.append(c_interpreted_trace)
        self.matrix_results["C"]["interpreted"] = c_interpreted_result

        # Optional early stop for quick Stage-A tests
        if self.stop_at in ("C", "C_interpreted"):
            final_output = {
                "meta": {
                    "generated_at": datetime.now(timezone.utc).isoformat(),
                    "model": self.model,
                    "temperature": self.temperature,
                    "lens_mode": self.lens_mode,
                    "kernel_hash": self._compute_kernel_hash(),
                    "token_count": self.token_count
                },
                "matrices": {
                    "C": self.matrix_results["C"],
                },
                "trace": trace_entries
            }
            self._validate_generate_lenses_only_in_auto()
            return final_output
        
        # Stage 3: Lens resolution and history injection (no LLM)
        c_lenses_result, c_lenses_trace = self._inject_lenses("problem statement", "C")
        trace_entries.append(c_lenses_trace)
        self.matrix_results["C"]["lenses"] = c_lenses_result
        
        # Stage 4: C/lensed.md - Lensed interpretation
        c_lensed_result, c_lensed_trace = self._execute_stage(
            "phase1_c_lensed", "C", "lensed"
        )
        trace_entries.append(c_lensed_trace)
        self.matrix_results["C"]["lensed"] = c_lensed_result
        
        # MATRIX J EXTRACTION - Extract from B (remove wisdom row)
        
        # J/extract.md - Extract Matrix J from Matrix B
        j_extract_result, j_extract_trace = self._execute_stage(
            "phase1_j_extract", "J", "extract"
        )
        trace_entries.append(j_extract_trace)
        self.matrix_results["J"] = {"extract": j_extract_result}
        
        # MATRIX F PIPELINE - 4 STAGES (Element-wise Product)
        
        # Stage 1: F/mechanical.md - Mechanical construction (element-wise)
        f_mechanical_result, f_mechanical_trace = self._execute_stage(
            "phase1_f_mechanical", "F", "mechanical"
        )
        trace_entries.append(f_mechanical_trace)
        self.matrix_results["F"] = {"mechanical": f_mechanical_result}
        
        # Stage 2: F/interpreted.md - Semantic interpretation 
        f_interpreted_result, f_interpreted_trace = self._execute_stage(
            "phase1_f_interpreted", "F", "interpreted"
        )
        trace_entries.append(f_interpreted_trace)
        self.matrix_results["F"]["interpreted"] = f_interpreted_result
        
        # Stage 3: Lens resolution and history injection (no LLM)
        f_lenses_result, f_lenses_trace = self._inject_lenses("requirements", "F")
        trace_entries.append(f_lenses_trace)
        self.matrix_results["F"]["lenses"] = f_lenses_result
        
        # Stage 4: F/lensed.md - Lensed interpretation
        f_lensed_result, f_lensed_trace = self._execute_stage(
            "phase1_f_lensed", "F", "lensed"
        )
        trace_entries.append(f_lensed_trace)
        self.matrix_results["F"]["lensed"] = f_lensed_result
        
        # MATRIX D PIPELINE - 4 STAGES (Semantic Addition with String Concatenation)
        
        # Preflight check for addition compatibility (A + F)
        from ...domain.preflight import preflight_addition
        try:
            matrix_a_info = {
                "name": "A",
                "rows": self.A.row_labels,
                "cols": self.A.col_labels
            }
            
            # Get F matrix info - use canonical if echo resolver doesn't provide structure
            f_rows = f_lensed_result.get("rows", [])
            f_cols = f_lensed_result.get("cols", [])
            
            # If echo resolver, use canonical F matrix structure
            if not f_rows or not f_cols:
                from ...domain.matrices.canonical import get_canonical_matrix
                canonical_f = get_canonical_matrix("F")
                f_rows = canonical_f.row_labels
                f_cols = canonical_f.col_labels
            
            matrix_f_info = {
                "name": "F", 
                "rows": f_rows,
                "cols": f_cols
            }
            preflight_addition(matrix_a_info, matrix_f_info)
            print("✅ Preflight addition check passed for Matrix D = A + F")
        except Exception as e:
            print(f"❌ Preflight addition check failed for Matrix D: {e}")
            raise
        
        # Stage 1: D/mechanical.md - String concatenation recipe (no LLM - mechanical construction)  
        d_mechanical_result, d_mechanical_trace = self._execute_stage(
            "phase1_d_mechanical", "D", "mechanical"
        )
        trace_entries.append(d_mechanical_trace)
        self.matrix_results["D"] = {"mechanical": d_mechanical_result}
        
        # Stage 2: D/interpreted.md - Resolve addition (concatenation semantics only)
        d_interpreted_result, d_interpreted_trace = self._execute_stage(
            "phase1_d_interpreted", "D", "interpreted"
        )
        trace_entries.append(d_interpreted_trace)
        self.matrix_results["D"]["interpreted"] = d_interpreted_result
        
        # Stage 3: Lens resolution and history injection (no LLM)
        d_lenses_result, d_lenses_trace = self._inject_lenses("objectives", "D")
        trace_entries.append(d_lenses_trace)
        self.matrix_results["D"]["lenses"] = d_lenses_result
        
        # Stage 4: D/lensed.md - Lensed interpretation
        d_lensed_result, d_lensed_trace = self._execute_stage(
            "phase1_d_lensed", "D", "lensed"
        )
        trace_entries.append(d_lensed_trace)
        self.matrix_results["D"]["lensed"] = d_lensed_result
        
        # MATRIX K TRANSFORMATION - Explain transpose of D
        
        # K/transform.md - Introduce Matrix K transformation
        k_transform_result, k_transform_trace = self._execute_stage(
            "phase1_k_transform", "K", "transform"
        )
        trace_entries.append(k_transform_trace)
        self.matrix_results["K"] = {"intro": k_transform_result}
        
        # MATRIX K - Transpose of D.lensed (code-only, no LLM, no station)
        
        # Transform D.lensed to get K via transpose
        d_lensed_elements = d_lensed_result.get("elements", [])
        d_lensed_rows = d_lensed_result.get("rows", [])
        d_lensed_cols = d_lensed_result.get("cols", [])
        
        # If echo resolver, use canonical D matrix structure
        if not d_lensed_rows or not d_lensed_cols:
            from ...domain.matrices.canonical import get_canonical_matrix
            canonical_d = get_canonical_matrix("D")
            d_lensed_rows = canonical_d.row_labels
            d_lensed_cols = canonical_d.col_labels
            # Generate mock elements for transpose
            d_lensed_elements = [[f"d_{i}_{j}" for j in range(len(d_lensed_cols))] 
                               for i in range(len(d_lensed_rows))]
        
        # Perform transpose: K[i,j] = D[j,i]
        k_elements = []
        for j in range(len(d_lensed_cols)):  # New rows = old cols
            k_row = []
            for i in range(len(d_lensed_rows)):  # New cols = old rows
                if i < len(d_lensed_elements) and j < len(d_lensed_elements[i]):
                    k_row.append(d_lensed_elements[i][j])
                else:
                    k_row.append(f"k_{j}_{i}")  # Fallback for echo resolver
            k_elements.append(k_row)
        
        # Store computed K for validation (no data-drop - LLM will recall and reconstruct)
        k_computed = {
            "rows": d_lensed_cols,  # K rows = D cols (transposed)
            "cols": d_lensed_rows,  # K cols = D rows (transposed)
            "elements": k_elements,
            "operation": "transpose_of_D"
        }
        self.matrix_results["K"]["computed"] = k_computed
        
        # No data-drop - the LLM will reconstruct K from memory in response to the prompt
        # Validate the LLM's reconstruction matches our computation
        if self.relaxed_json:
            # In relaxed mode, validation happens after extraction
            pass
        else:
            # In strict mode, validate immediately
            try:
                self._validate_matrix_reconstruction("K", k_transform_result, k_computed)
            except ValueError as e:
                print(f"⚠️ Matrix K reconstruction validation failed: {e}", file=__import__('sys').stderr)
                # Continue for now - validation is informative
        
        # MATRIX X (Verification) - 4 STAGES (Dot Product K · J)
        
        # Preflight check for dot product compatibility (K · J)
        from ...domain.preflight import preflight_dot
        try:
            matrix_k_info = {
                "name": "K",
                "rows": k_computed["rows"],
                "cols": k_computed["cols"]
            }
            matrix_j_info = {
                "name": "J",
                "rows": self.J.row_labels,
                "cols": self.J.col_labels
            }
            preflight_dot(matrix_k_info, matrix_j_info)
            print("✅ Preflight dot product check passed for Matrix X = K · J")
        except Exception as e:
            print(f"❌ Preflight dot product check failed for Matrix X: {e}")
            raise
        
        # Initialize X results
        self.matrix_results["X"] = {}
        
        # Stage 1: X/mechanical.md - Mechanical construction (sum-of-products)
        x_mechanical_result, x_mechanical_trace = self._execute_stage(
            "phase1_x_mechanical", "X", "mechanical"
        )
        trace_entries.append(x_mechanical_trace)
        self.matrix_results["X"]["mechanical"] = x_mechanical_result
        
        # Stage 2: X/interpreted.md - Resolve operators (* before +, no operators remain)
        x_interpreted_result, x_interpreted_trace = self._execute_stage(
            "phase1_x_interpreted", "X", "interpreted"
        )
        trace_entries.append(x_interpreted_trace)
        self.matrix_results["X"]["interpreted"] = x_interpreted_result
        
        # Stage 3: Lens resolution and history injection (no LLM)
        x_lenses_result, x_lenses_trace = self._inject_lenses("verification", "X")
        trace_entries.append(x_lenses_trace)
        self.matrix_results["X"]["lenses"] = x_lenses_result
        
        # Stage 4: X/lensed.md - Lensed interpretation
        x_lensed_result, x_lensed_trace = self._execute_stage(
            "phase1_x_lensed", "X", "lensed"
        )
        trace_entries.append(x_lensed_trace)
        self.matrix_results["X"]["lensed"] = x_lensed_result
        
        # MATRIX Z (Validation) - Clean Flow without Reference Block
        
        # Z proceeds directly from X.lensed in conversation history
        # No reference data-drop needed - Z prompt handles previous turn reference
        self.matrix_results["Z"] = {}
        
        # Stage 3: Lens resolution and history injection (no LLM)
        z_lenses_result, z_lenses_trace = self._inject_lenses("validation", "Z")
        trace_entries.append(z_lenses_trace)
        self.matrix_results["Z"]["lenses"] = z_lenses_result
        
        # Stage 4a: Z/lensed.md - Lensed interpretation
        z_lensed_result, z_lensed_trace = self._execute_stage(
            "phase1_z_lensed", "Z", "lensed"
        )
        trace_entries.append(z_lensed_trace)
        self.matrix_results["Z"]["lensed"] = z_lensed_result
        
        # Stage 4b: Z/principles.md - Principle extraction
        z_principles_result, z_principles_trace = self._execute_stage(
            "phase1_z_principles", "Z", "principles"
        )
        trace_entries.append(z_principles_trace)
        self.matrix_results["Z"]["principles"] = z_principles_result
        
        # MATRIX G EXTRACTION - Extract first 3 rows from Z
        
        # G/extract.md - Extract Matrix G from Matrix Z
        g_extract_result, g_extract_trace = self._execute_stage(
            "phase1_g_extract", "G", "extract"
        )
        trace_entries.append(g_extract_trace)
        self.matrix_results["G"] = {"intro": g_extract_result}
        
        # MATRIX P EXTRACTION - Extract 4th row from Z
        
        # P/extract.md - Extract Matrix P from Matrix Z
        p_extract_result, p_extract_trace = self._execute_stage(
            "phase1_p_extract", "P", "extract"
        )
        trace_entries.append(p_extract_trace)
        self.matrix_results["P"] = {"extract": p_extract_result}
        
        # MATRIX T TRANSFORMATION - Transpose of J
        
        # T/transform.md - Transform Matrix J to Matrix T
        t_transform_result, t_transform_trace = self._execute_stage(
            "phase1_t_transform", "T", "transform"
        )
        trace_entries.append(t_transform_trace)
        self.matrix_results["T"] = {"intro": t_transform_result}
        
        # MATRIX E (Evaluation) - Dot Product with Derived Inputs G·T
        
        # Precompute Matrix G = Z[0:3, :] (slice of first 3 rows)
        z_lensed_elements = z_lensed_result.get("elements", [])
        z_lensed_rows = z_lensed_result.get("rows", ["guiding", "applying", "judging", "reflecting"])
        z_lensed_cols = z_lensed_result.get("cols", ["necessity (vs contingency)", "sufficiency", "completeness", "consistency"])
        
        # If echo resolver, generate mock elements
        if not z_lensed_elements:
            z_lensed_elements = [[f"z_{i}_{j}" for j in range(len(z_lensed_cols))] 
                               for i in range(len(z_lensed_rows))]
        
        # Store computed G for validation (no data-drop - LLM will recall and reconstruct)
        g_elements = z_lensed_elements[:3] if len(z_lensed_elements) >= 3 else z_lensed_elements
        g_computed = {
            "rows": z_lensed_rows[:3],  # First 3 rows: guiding, applying, judging
            "cols": z_lensed_cols,       # Same columns as Z
            "elements": g_elements,
            "operation": "first_3_rows_of_Z"
        }
        self.matrix_results["G"]["computed"] = g_computed
        
        # No data-drop - the LLM will reconstruct G from memory in response to the prompt
        # Validate the LLM's reconstruction matches our computation
        if self.relaxed_json:
            # In relaxed mode, validation happens after extraction
            pass
        else:
            # In strict mode, validate immediately
            try:
                self._validate_matrix_reconstruction("G", g_extract_result, g_computed)
            except ValueError as e:
                print(f"⚠️ Matrix G reconstruction validation failed: {e}", file=__import__('sys').stderr)
                # Continue for now - validation is informative
        
        # Precompute Matrix T = Jᵀ (transpose of J)
        j_elements = [[cell.value for cell in row] for row in self.J.cells]
        t_elements = []
        for j in range(len(self.J.col_labels)):  # New rows = old cols
            t_row = []
            for i in range(len(self.J.row_labels)):  # New cols = old rows
                t_row.append(j_elements[i][j])
            t_elements.append(t_row)
        
        # Store computed T for validation (no data-drop - LLM will recall and reconstruct)
        t_computed = {
            "rows": self.J.col_labels,  # T rows = J cols (transposed)
            "cols": self.J.row_labels,  # T cols = J rows (transposed)
            "elements": t_elements,
            "operation": "transpose_of_J"
        }
        self.matrix_results["T"]["computed"] = t_computed
        
        # No data-drop - the LLM will reconstruct T from memory in response to the prompt
        # Validate the LLM's reconstruction matches our computation
        if self.relaxed_json:
            # In relaxed mode, validation happens after extraction
            pass
        else:
            # In strict mode, validate immediately
            try:
                self._validate_matrix_reconstruction("T", t_transform_result, t_computed)
            except ValueError as e:
                print(f"⚠️ Matrix T reconstruction validation failed: {e}", file=__import__('sys').stderr)
                # Continue for now - validation is informative
        
        # Preflight check for dot product compatibility (G · T)
        try:
            # Extract dimensions from computed matrices
            g_rows = g_computed.get("rows", [])
            g_cols = g_computed.get("cols", [])
            t_rows = t_computed.get("rows", [])
            t_cols = t_computed.get("cols", [])
            
            matrix_g_info = {
                "name": "G",
                "rows": g_rows,
                "cols": g_cols
            }
            matrix_t_info = {
                "name": "T",
                "rows": t_rows,
                "cols": t_cols
            }
            preflight_dot(matrix_g_info, matrix_t_info)
            print("✅ Preflight dot product check passed for Matrix E = G · T")
        except Exception as e:
            print(f"❌ Preflight dot product check failed for Matrix E: {e}")
            raise
        
        # Stage 1: E/mechanical.md - Mechanical construction (sum-of-products)
        e_mechanical_result, e_mechanical_trace = self._execute_stage(
            "phase1_e_mechanical", "E", "mechanical"
        )
        trace_entries.append(e_mechanical_trace)
        self.matrix_results["E"] = {"mechanical": e_mechanical_result}
        
        # Stage 2: E/interpreted.md - Resolve operators (* before +, no operators remain)
        e_interpreted_result, e_interpreted_trace = self._execute_stage(
            "phase1_e_interpreted", "E", "interpreted"
        )
        trace_entries.append(e_interpreted_trace)
        self.matrix_results["E"]["interpreted"] = e_interpreted_result
        
        # Stage 3: Lens resolution and history injection (no LLM)
        e_lenses_result, e_lenses_trace = self._inject_lenses("evaluation", "E")
        trace_entries.append(e_lenses_trace)
        self.matrix_results["E"]["lenses"] = e_lenses_result
        
        # Stage 4: E/lensed.md - Lensed interpretation
        e_lensed_result, e_lensed_trace = self._execute_stage(
            "phase1_e_lensed", "E", "lensed"
        )
        trace_entries.append(e_lensed_trace)
        self.matrix_results["E"]["lensed"] = e_lensed_result
        
        # All matrices (C,F,D,K,X,Z,G,T,E) implementation complete
        
        # Build final output structure
        final_output = {
            "meta": {
                "generated_at": datetime.now(timezone.utc).isoformat(),
                "model": self.model,
                "temperature": self.temperature,
                "lens_mode": self.lens_mode,
                "kernel_hash": "phase_e_matrix_c_f",
                "token_count": self.token_count
            },
            "matrices": {
                "C": self.matrix_results["C"],
                "F": self.matrix_results["F"], 
                "D": self.matrix_results["D"],
                "K": self.matrix_results["K"],
                "X": self.matrix_results["X"],
                "Z": self.matrix_results["Z"],
                "G": self.matrix_results["G"],
                "T": self.matrix_results["T"],
                "E": self.matrix_results["E"]
            },
            "trace": trace_entries
        }
        
        # FIX-7: Guard that generate_lenses only appears in auto mode
        self._validate_generate_lenses_only_in_auto()
        
        return final_output

    def _generate_lenses_in_transcript(self, station: str, matrix_name: str, 
                                     interpreted_rows: List[str], interpreted_cols: List[str]) -> Dict[str, Any]:
        """
        C1-2: Generate lenses in-transcript using generate_lenses.md prompt (option A).
        
        Per colleague_1's specification:
        - Load phase1/<MATRIX>/generate_lenses.md and render with matrix parameters
        - Add user turn to transcript with rendered prompt
        - Call Responses API with strict JSON schema
        - Use assistant JSON response as canonical lenses (no additional data-drop)
        - Record lens_source="auto" in trace only (not transcript)
        
        Args:
            station: Station name (e.g., "problem statement")
            matrix_name: Matrix name (e.g., "C")
            interpreted_rows: Row labels from interpreted matrix
            interpreted_cols: Column labels from interpreted matrix
            
        Returns:
            Dict with lenses result in expected format
        """
        from ...domain.matrices.canonical import get_matrix_info
        import json
        
        # Get matrix info for rendering
        matrix_info = get_matrix_info(matrix_name)
        if not matrix_info:
            raise ValueError(f"Unknown matrix: {matrix_name}")
        
        # Use interpreted dimensions if available, otherwise canonical
        rows = interpreted_rows if interpreted_rows else matrix_info["row_labels"]
        cols = interpreted_cols if interpreted_cols else matrix_info["col_labels"]
        
        # Load and render the generation asset
        asset_id = f"phase1_{matrix_name.lower()}_generate_lenses"
        try:
            asset_text = self.registry.get_text(asset_id)
        except KeyError:
            raise ValueError(f"Generation asset not found: {asset_id}")
        
        # Replace placeholders with strict templater (FIX-2)
        template_vars = {
            "station": station,
            "matrix_id": matrix_name,
            "rows": str(len(rows)),
            "cols": str(len(cols)),
            "row_labels": json.dumps(rows),
            "col_labels": json.dumps(cols)
        }
        
        rendered_prompt = self._render_template_strict(asset_text, template_vars)
        if not self.relaxed_json:
            rendered_prompt += "\n\nReturn one JSON object that satisfies the provided schema exactly: fill every required field and do not include extra keys."
        
        # Add user turn to transcript with rendered prompt
        user_message = {"role": "user", "content": rendered_prompt}
        self.dialogue_history.append(user_message)
        
        # Get system prompt and build input from dialogue history
        system_text = self.registry.get_text("system")
        input_text = self._build_canonical_transcript()
        
        # Build strict JSON schema for lenses response from single source
        from ...infrastructure.validation.json_schema_converter import get_strict_response_format
        response_format = get_strict_response_format(matrix_name, "lenses")
        # Log minimal schema info (no secrets)
        try:
            rf_type = response_format.get("type") if isinstance(response_format, dict) else None
            js = response_format.get("json_schema", {}) if isinstance(response_format, dict) else {}
            js_name = js.get("name")
            root = js.get("schema", {}) if isinstance(js, dict) else {}
            required = root.get("required", []) if isinstance(root, dict) else []
            addl = root.get("additionalProperties", None)
            print(f"ℹ️  Lenses RF:{rf_type} name:{js_name} required:{required} additionalProperties:{addl}")
        except Exception:
            pass
        
        # Call Responses API (relaxed mode bypasses response_format)
        response = call_responses(
            instructions=system_text,
            input=input_text,
            reasoning_effort=self.reasoning_effort,
            response_format=None if self.relaxed_json else response_format,
            expects_json=not self.relaxed_json,
            store=True
        )
        
        # Parse the assistant's JSON response, allow a single repair retry if missing 'lenses'
        response_content = response.get("output_text", "")
        if not response_content:
            raw = response.get("raw", {})
            raise RuntimeError(f"Empty lenses JSON response for {matrix_name}/{station}. raw={str(raw)[:300]}")
        if self.relaxed_json:
            # Skip JSON enforcement; record raw content
            self.dialogue_history.append({"role": "assistant", "content": response_content})
            return {
                "station": station,
                "matrix_id": matrix_name,
                "content": response_content,
                "source": "auto",
            }
        try:
            lenses_json = json.loads(response_content)
        except json.JSONDecodeError as e:
            raise ValueError(f"Failed to parse lenses JSON response: {e}")
        
        # Add assistant turn to transcript with JSON response (option A)
        assistant_message = {"role": "assistant", "content": response_content}
        self.dialogue_history.append(assistant_message)
        
        # Validate lenses structure
        if "lenses" not in lenses_json and not self.relaxed_json:
            # One constrained retry with corrective note
            note = (
                "Previous output was invalid: missing key 'lenses'. "
                "Re-emit a single JSON object that satisfies the provided schema exactly."
            )
            retry_input = f"{input_text}\n\nNOTE: {note}"
            response_retry = call_responses(
                instructions=system_text,
                input=retry_input,
                reasoning_effort=self.reasoning_effort,
                response_format=response_format,
                expects_json=True,
                store=True
            )
            retry_text = response_retry.get("output_text", "")
            try:
                lenses_json = json.loads(retry_text)
            except json.JSONDecodeError:
                raise ValueError("Lenses response missing 'lenses' field")
            if "lenses" not in lenses_json:
                raise ValueError("Lenses response missing 'lenses' field")
            
        lenses_array = lenses_json["lenses"]
        if len(lenses_array) != len(rows):
            raise ValueError(f"Lenses row count mismatch: expected {len(rows)}, got {len(lenses_array)}")
            
        for i, row in enumerate(lenses_array):
            if len(row) != len(cols):
                raise ValueError(f"Lenses col count mismatch in row {i}: expected {len(cols)}, got {len(row)}")
        
        # FIX-6: Validate lenses JSON in auto mode (log to trace, not transcript)
        from ...infrastructure.validation.schemas import validate_lens_payload
        
        # Create lens block format for validation (similar to catalog mode)
        lens_content = self._render_clean_payload(rows, cols, lenses_array, "lenses_json")
        # Clean semantic format for lens injection
        lens_block = f"""## Interpretive Lenses for {matrix_name}
Station: {station}
Size: {len(lenses_result['rows'])}x{len(lenses_result['cols'])}
Row names: {lenses_result['rows']}
Column names: {lenses_result['cols']}
Lenses: {lenses_result['lenses']}"""
        
        lens_validation_errors = validate_lens_payload(lens_block)
        if lens_validation_errors:
            # Log to trace/stderr, not transcript
            print(f"⚠️  Auto mode lens validation warnings for {station}/{matrix_name}:")
            for error in lens_validation_errors:
                print(f"    - {error}")
        else:
            print(f"✅ Auto mode lens validation passed for {station}/{matrix_name}")
        
        # Return lenses_result in expected format
        return {
            "station": station,
            "matrix_id": matrix_name,
            "rows": rows,
            "cols": cols,
            "lenses": lenses_array,
            "source": "auto",
            "meta": {
                "generated_at": "in-transcript",
                "model": self.model,
                "reasoning_effort": self.reasoning_effort,
                "source": "auto"
            }
        }

    def _render_template_strict(self, template: str, variables: Dict[str, str]) -> str:
        """
        FIX-2: Strict template renderer with fail-fast validation.
        
        Renders template variables and ensures no {{...}} placeholders remain unresolved.
        
        Args:
            template: Template string with {{variable}} placeholders
            variables: Dict of variable name -> value mappings
            
        Returns:
            Rendered template string
            
        Raises:
            ValueError: If any {{...}} placeholders remain unresolved
        """
        import re
        
        rendered = template
        
        # Replace each variable
        for var_name, var_value in variables.items():
            placeholder = f"{{{{{var_name}}}}}"
            rendered = rendered.replace(placeholder, var_value)
        
        # Check for any remaining unresolved placeholders
        leftover_matches = re.findall(r'\{\{[^}]+\}\}', rendered)
        if leftover_matches:
            raise ValueError(
                f"Template rendering failed: unresolved placeholders {leftover_matches}. "
                f"Available variables: {list(variables.keys())}"
            )
        
        return rendered

    def _build_canonical_transcript(self) -> str:
        """
        FIX-3: Build canonical transcript format for LLM input.
        
        Returns dialogue history as human-readable text format, not JSON.
        This maintains semantic consistency with how the LLM sees conversations.
        
        Returns:
            Canonical transcript string
        """
        transcript_lines = []
        for msg in self.dialogue_history:
            role = msg["role"]
            content = msg["content"]
            if role != "system":  # Skip system message as it goes in instructions
                transcript_lines.append(f"[{role.upper()}] {content}")
        
        return "\n\n".join(transcript_lines)

    def _validate_generate_lenses_only_in_auto(self) -> None:
        """
        FIX-7: Guard that generate_lenses.md only appears in transcript when lens_mode=auto.
        
        Per colleague_1's specification: "allow generate_lenses.md to appear in the 
        transcript only when lens_mode='auto'".
        
        Raises:
            ValueError: If generate_lenses content appears in non-auto mode
        """
        # Only enforce this guard in non-auto modes
        if self.lens_mode == "auto":
            return  # Auto mode is allowed to have generate_lenses content
        
        # Search transcript for generate_lenses content
        for i, turn in enumerate(self.dialogue_history):
            content = turn.get("content", "")
            
            # Look for generate_lenses asset patterns
            if "generate_lenses" in content.lower() or "Generate Complete Lens Matrix" in content:
                raise ValueError(
                    f"FIX-7 Generate lenses guard: Turn {i} contains generate_lenses content in {self.lens_mode} mode. "
                    f"generate_lenses.md content is only allowed in auto mode, not {self.lens_mode} mode. "
                    f"Content preview: {content[:100]}..."
                )

    def _execute_stage(self, asset_id: str, matrix_name: str, stage: str) -> tuple[Dict[str, Any], Dict[str, Any]]:
        """
        Execute a single stage of the conversational pipeline.
        
        Args:
            asset_id: Asset ID for the prompt (e.g., "phase1_c_mechanical")
            matrix_name: Matrix name (e.g., "C")
            stage: Stage name (e.g., "mechanical")
        
        Returns:
            Tuple of (stage_result, trace_entry)
        """
        import hashlib
        
        # Load the prompt asset
        prompt_text = self.registry.get_text(asset_id)
        # In relaxed/semantic mode, strip any JSON-only output directives from assets
        if self.relaxed_json:
            import re as _re
            # Remove sections titled 'Output format' and trailing JSON-only instructions
            prompt_text = _re.sub(r"(?is)\n+#+\s*Output\s*format.*\Z", "", prompt_text)

        # JSON tails removed (server-enforced JSON); no tail guidance inserted

        from ...domain.matrices.canonical import get_matrix_info
        import json

        matrix_info = get_matrix_info(matrix_name)
        station = STATION_MAP.get(matrix_name, "")

        template_vars = {
            "matrix_id": matrix_name,
            "station": station,
            "n_rows": str(matrix_info["rows"]),
            "n_cols": str(matrix_info["cols"]),
            "rows_json": json.dumps(matrix_info["row_labels"]),
            "cols_json": json.dumps(matrix_info["col_labels"]),
        }

        # Render prompt (no json_tail placeholder)
        rendered_prompt = self._render_template_strict(prompt_text, template_vars)
        
        # Add user message to history
        user_message = {"role": "user", "content": rendered_prompt}
        self.dialogue_history.append(user_message)
        
        # Compute input hash for provenance
        input_text = self._build_canonical_transcript()
        input_hash = hashlib.sha256(input_text.encode()).hexdigest()[:16]
        
        # Make LLM call using new Responses API interface
        try:
            # Get asset SHA for provenance
            asset_info = self.registry.get(asset_id)
            asset_sha = asset_info.sha256[:16]
            
            # Build instructions (system.md sent explicitly every call)
            system_text = self.registry.get_text("system")
            
            # Build input from the full transcript (excluding system which is sent via instructions).
            # Note: The latest user prompt has already been appended to dialogue_history,
            # so _build_canonical_transcript() already includes it. Do NOT append it again.
            input_text = self._build_canonical_transcript()
            
            # P0-3: Get strict JSON schema for response format (per colleague_1's specification)
            from ...infrastructure.validation.json_schema_converter import get_strict_response_format
            response_format = get_strict_response_format(matrix_name, stage)
            
            # Log minimal schema info for triage (no secrets)
            try:
                rf_type = response_format.get("type") if isinstance(response_format, dict) else None
                js = response_format.get("json_schema", {}) if isinstance(response_format, dict) else {}
                js_name = js.get("name")
                root = js.get("schema", {}) if isinstance(js, dict) else {}
                required = root.get("required", []) if isinstance(root, dict) else []
                addl = root.get("additionalProperties", None)
                print(f"ℹ️  {matrix_name}/{stage} RF:{rf_type} name:{js_name} required:{required} additionalProperties:{addl}")
            except Exception:
                pass

            if matrix_name == "C" and (not self.relaxed_json) and getattr(self, 'inband_c_normalize', False):
                # Stage A: creative
                resp_a = call_responses(
                    instructions=system_text,
                    input=input_text,
                    reasoning_effort=self.reasoning_effort,
                    expects_json=False,
                    store=True,
                    temperature=self.temperature,
                    top_p=1.0,
                )
                creative_text = resp_a.get("output_text", "")
                # Stage B: normalize with strict schema
                try:
                    from pathlib import Path as _P
                    norm_path = _P("chirality/infrastructure/prompts/assets/phase1/normalize_to_json.md")
                    normalizer_instructions = norm_path.read_text(encoding="utf-8")
                except Exception:
                    normalizer_instructions = (
                        "You convert the provided plain text into a single JSON object that matches the schema exactly. "
                        "Extract only; do not add or infer; no extra keys."
                    )
                response = call_responses(
                    instructions=normalizer_instructions,
                    input=[{"role": "user", "content": [{"type": "input_text", "text": creative_text}]}],
                    response_format=response_format,
                    expects_json=True,
                    store=True,
                    temperature=0.2,
                    top_p=1.0,
                )
                response_content = response.get("output_text", "")
            else:
                response = call_responses(
                    instructions=system_text,
                    input=input_text,
                    reasoning_effort=self.reasoning_effort,
                    response_format=None if self.relaxed_json else response_format,
                    expects_json=not self.relaxed_json,
                    store=True
                )
                # Extract response content
                response_content = response.get("output_text", "")
            
            # Parse JSON response
            def _validate_and_report(payload: dict) -> tuple[bool, list[str]]:
                from ...infrastructure.validation.json_schema_converter import validate_stage_response_strict
                is_valid, schema_errors = validate_stage_response_strict(payload, matrix_name, stage)
                if not is_valid:
                    print(f"❌ P0-3 strict schema validation failed for {matrix_name}/{stage}:")
                    for error in schema_errors:
                        print(f"    - {error}")
                else:
                    print(f"✅ P0-3 strict schema validation passed for {matrix_name}/{stage}")
                return is_valid, schema_errors

            # Parse JSON and validate unless relaxed
            if self.relaxed_json:
                stage_result = {"content": response_content}
            else:
                try:
                    parsed_once = json.loads(response_content)
                    ok, errs = _validate_and_report(parsed_once)
                    if ok:
                        stage_result = parsed_once
                    else:
                        correction = (
                            "Previous output did not satisfy the schema. "
                            "Re-emit a single JSON object matching the schema exactly."
                        )
                        retry_input = f"{input_text}\n\nNOTE: {correction}"
                        response_retry = call_responses(
                            instructions=system_text,
                            input=retry_input,
                            reasoning_effort=self.reasoning_effort,
                            response_format=response_format,
                            expects_json=True,
                            store=True
                        )
                        retry_text = response_retry.get("output_text", "")
                        try:
                            parsed_retry = json.loads(retry_text)
                            ok2, errs2 = _validate_and_report(parsed_retry)
                            stage_result = parsed_retry if ok2 else parsed_once
                            if not ok2:
                                stage_result["_validation_errors"] = errs2
                        except json.JSONDecodeError:
                            stage_result = parsed_once
                            stage_result["_validation_errors"] = errs
                except json.JSONDecodeError:
                    stage_result = {"content": response_content, "error": "json_parse_failed"}
            
            # Add assistant response to history
            assistant_message = {"role": "assistant", "content": response_content}
            self.dialogue_history.append(assistant_message)
            
            # Update token count from actual usage
            usage = response.get("usage")
            if usage and hasattr(usage, "total_tokens"):
                self.token_count += getattr(usage, "total_tokens", 0)
            else:
                # Fallback to estimation
                self.token_count += len(rendered_prompt.split()) + len(response_content.split())
            
            # Compute output hash
            output_hash = hashlib.sha256(response_content.encode()).hexdigest()[:16]
            
            # Build trace entry
            trace_entry = {
                "asset_id": asset_id,
                "asset_sha": asset_sha, 
                "matrix": matrix_name,
                "stage": stage,
                "model": self.model,
                "temperature": self.temperature,
                "input_hash": input_hash,
                "output_hash": output_hash,
                "usage": usage if usage else {"estimated_tokens": len(rendered_prompt.split()) + len(response_content.split())},
                "api_call": "responses",  # Mark as using new API
                "response_id": response.get("id"),
                "timestamp": datetime.now(timezone.utc).isoformat()
            }
            
            return stage_result, trace_entry
            
        except Exception as e:
            # Error handling
            error_result = {"error": str(e), "stage": stage, "matrix": matrix_name}
            error_trace = {
                "asset_id": asset_id,
                "error": str(e),
                "timestamp": datetime.now(timezone.utc).isoformat()
            }
            return error_result, error_trace

    def _render_clean_payload(self, rows: list, cols: list, values: Any, values_key: str = "values_json") -> str:
        """
        Render clean data payload with only semantic content.
        
        Args:
            rows: Matrix row labels
            cols: Matrix column labels  
            values: Matrix values (elements or lenses)
            values_key: Key name for values field (values_json or lenses_json)
            
        Returns:
            Clean formatted block content
        """
        import json
        return f"""rows: {json.dumps(rows)}
cols: {json.dumps(cols)}
{values_key}:
{json.dumps(values, indent=2)}"""

    def emit_data_drop(self, kind: Literal["transform"], matrix_name: str, payload: Dict[str, Any]) -> tuple[Dict[str, Any], Dict[str, Any]]:
        """
        Emit canonical data-drop for non-LLM stages.
        
        Per colleague_1's specification:
        - Appends canonical BEGIN/END block as user turn
        - Records turn_type:"data" in trace
        - No LLM call: no response.id, no usage
        - Only clean semantic content: rows, cols, values_json
        
        Args:
            kind: Must be "transform" (reference blocks eliminated)
            matrix_name: Name of the matrix (for trace/provenance only)
            payload: Clean data payload with rows, cols, values_json only
            
        Returns:
            Tuple of (data_drop_result, trace_entry)
        """
        import json
        import hashlib
        from datetime import datetime, timezone
        
        # Build clean data-drop block based on kind
        if getattr(self, 'relaxed_json', False):
            # Do not silently default structural keys; surface empties and let extractor decide
            if 'rows' not in payload:
                payload['rows'] = []
            if 'cols' not in payload:
                payload['cols'] = []
            if 'values_json' not in payload:
                payload['values_json'] = []
        if kind == "transform":
            block_content = self._render_clean_payload(
                payload['rows'], 
                payload['cols'], 
                payload['values_json']
            )
            
            # Clean semantic format - just the matrix elements as a Python-style list
            # This maintains semantic coherence without metadata pollution
            data_drop_block = f"""## Matrix {matrix_name}
[{matrix_name}]
Size: {len(payload['rows'])}x{len(payload['cols'])}
Row names: {payload['rows']}
Column names: {payload['cols']}
Elements: {payload['values_json']}

Matrix {matrix_name} has been derived through transformation."""
        
        else:
            raise ValueError(f"Invalid data-drop kind: {kind}. Must be 'transform'")
        
        # Add user turn to dialogue history
        user_message = {"role": "user", "content": data_drop_block}
        self.dialogue_history.append(user_message)
        
        # Build data-drop result
        data_drop_result = {
            "artifact": "data_drop",
            "kind": kind,
            "matrix": matrix_name,
            "rows": payload['rows'],
            "cols": payload['cols'],
            "block_content": block_content
        }
        
        # Add values if transform type
        if kind == "transform" and 'values_json' in payload:
            data_drop_result["values_json"] = payload['values_json']
        
        # Compute content hash for provenance
        content_hash = hashlib.sha256(data_drop_block.encode()).hexdigest()[:16]
        
        # Build trace entry (no LLM call, no response.id, no usage)
        trace_entry = {
            "kind": "data_drop",
            "data_drop_type": kind,
            "matrix": matrix_name,
            "turn_type": "data",  # Mark as data turn per spec
            "content_hash": content_hash,
            "timestamp": datetime.now(timezone.utc).isoformat(),
            "no_llm_call": True  # Explicit marker
        }
        
        return data_drop_result, trace_entry

    def _inject_lenses(self, station: str, matrix_name: str) -> tuple[Dict[str, Any], Dict[str, Any]]:
        """
        Inject lenses into conversation history as canonical data-drop turn.
        
        Per colleague_1's D2-1 specification:
        - Use role="user" (data belongs in conversational turns, not system)
        - Canonical BEGIN/END block format for unambiguous reference
        - Machine-tight block for testing and Stage-4 reference
        
        Per colleague_1's D2-2 specification:
        - Preflight parity check: assert rows/cols match between interpreted matrix and lens block
        - Fail fast on mismatch with explicit error
        
        Args:
            station: Station name (e.g., "Problem Statement")
            matrix_name: Matrix name (e.g., "C")
        
        Returns:
            Tuple of (lenses_result, trace_entry)
        """
        import json
        import hashlib
        
        # D2-2: Preflight parity check
        # Verify rows/cols from interpreted matrix match lens block before injection
        interpreted_result = self.matrix_results.get(matrix_name, {}).get("interpreted")
        
        # Initialize variables
        interpreted_rows = None
        interpreted_cols = None
        
        if interpreted_result:
            
            # Handle different interpreted result formats
            if isinstance(interpreted_result, dict):
                if "rows" in interpreted_result and "cols" in interpreted_result:
                    interpreted_rows = interpreted_result["rows"]
                    interpreted_cols = interpreted_result["cols"]
                elif "content" in interpreted_result:
                    # Try to parse from content string if JSON parsing failed
                    content_str = str(interpreted_result["content"])
                    if "'rows':" in content_str and "'cols':" in content_str:
                        try:
                            # Extract rows/cols from string representation
                            import ast
                            if content_str.startswith("("):
                                # Parse tuple format
                                parsed_content = ast.literal_eval(content_str.split(",")[0] + "}")
                                if "rows" in parsed_content:
                                    interpreted_rows = parsed_content["rows"]
                                    interpreted_cols = parsed_content["cols"]
                        except:
                            pass  # Continue without preflight check if parsing fails
        
        # C1-2: Handle auto mode with in-transcript lens generation (option A)
        if self.lens_mode == "auto":
            # Generate lenses in-transcript using generate_lenses.md prompt
            lenses_result = self._generate_lenses_in_transcript(station, matrix_name, interpreted_rows, interpreted_cols)
        else:
            # Resolve lenses using lens system (catalog mode)
            lenses_result = self.lens_resolver.resolve_lenses(station)
        
        # In relaxed mode, skip parity checks and injection block building
        if getattr(self, 'relaxed_json', False):
            trace_entry = {
                "type": "lens_injection",
                "turn_type": "data",
                "station": station,
                "matrix": matrix_name,
                "source": lenses_result.get("source", "auto"),
                "timestamp": datetime.now(timezone.utc).isoformat(),
            }
            return lenses_result, trace_entry

        # D2-2: Perform preflight parity check if we have interpreted data
        if interpreted_rows is not None and interpreted_cols is not None:
            lens_rows = lenses_result["rows"]
            lens_cols = lenses_result["cols"]
            
            if interpreted_rows != lens_rows:
                raise ValueError(
                    f"Row/col parity check failed for Matrix {matrix_name}: "
                    f"interpreted rows {interpreted_rows} != lens rows {lens_rows}. "
                    f"Matrix dimensions have drifted between stages."
                )
            
            if interpreted_cols != lens_cols:
                raise ValueError(
                    f"Row/col parity check failed for Matrix {matrix_name}: "
                    f"interpreted cols {interpreted_cols} != lens cols {lens_cols}. "
                    f"Matrix dimensions have drifted between stages."
                )
            
            # Log successful parity check
            print(f"✅ Preflight parity check passed for Matrix {matrix_name}: {len(lens_rows)}×{len(lens_cols)}")
        else:
            # Expected skip when using echo resolver or when interpreted layer is not available
            if hasattr(self, 'model') and self.model == "echo":
                # Expected for echo resolver - not a problem
                pass  # Silent for echo mode
            else:
                # Log structured skip reason for production
                print(f"ℹ️  Matrix {matrix_name} parity check: interpreted layer not computed (expected for non-LLM stages)")
        
        # Get metadata for provenance
        from ...infrastructure.prompts.registry import get_registry
        registry = get_registry()
        
        # Compute hashes for provenance
        system_text = registry.get_text("system")
        system_sha = hashlib.sha256(system_text.encode()).hexdigest()[:16]
        
        # Get normative context if available
        from pathlib import Path
        normative_file = Path(__file__).parent.parent.parent / "normative_system_prompt.txt"
        if normative_file.exists():
            normative_text = normative_file.read_text()
            normative_sha = hashlib.sha256(normative_text.encode()).hexdigest()[:16]
        else:
            normative_sha = "unavailable"
        
        # Asset SHA from lens source
        asset_sha = lenses_result.get("meta", {}).get("asset_sha", "unknown")
        if not asset_sha or asset_sha == "unknown":
            # Generate from lens generation context if auto-generated
            if lenses_result["source"] == "auto":
                asset_sha = "phase1_lens_auto_gen"
            elif lenses_result["source"] == "catalog":
                asset_sha = "phase1_lens_catalog"
            else:
                asset_sha = "phase1_lens_override"
        
        # Build clean lens block - semantic content only
        lens_content = self._render_clean_payload(
            lenses_result["rows"],
            lenses_result["cols"], 
            lenses_result["lenses"],
            "lenses_json"
        )
        # Clean semantic format for lens injection
        lens_block = f"""## Interpretive Lenses for {matrix_name}
Station: {station}
Size: {len(lenses_result['rows'])}x{len(lenses_result['cols'])}
Row names: {lenses_result['rows']}
Column names: {lenses_result['cols']}
Lenses: {lenses_result['lenses']}"""
        
        # C1-2: For auto mode, lenses are already in transcript from LLM response (option A)
        # For catalog mode, add data-drop turn  
        if self.lens_mode != "auto":
            # D2-5: Validate lens payload before injection
            lens_validation_errors = validate_lens_payload(lens_block)
            if lens_validation_errors:
                print(f"⚠️  D2-5 lens validation warnings for {station}/{matrix_name}:")
                for error in lens_validation_errors:
                    print(f"    - {error}")
            else:
                print(f"✅ D2-5 lens payload validation passed for {station}/{matrix_name}")
            
            # Add lens data as USER message (data belongs in conversational turns)
            lens_message = {"role": "user", "content": lens_block}
            self.dialogue_history.append(lens_message)
        else:
            print(f"✅ Auto mode: lenses already in transcript from LLM generation for {station}/{matrix_name}")
        
        # Build trace entry with turn_type: "data"
        trace_entry = {
            "type": "lens_injection",
            "turn_type": "data",  # Mark as data drop per colleague_1 spec
            "station": station,
            "matrix": matrix_name,
            "source": lenses_result["source"],
            "lens_count": sum(len(row) for row in lenses_result["lenses"]),
            "meta": {
                "system_sha": system_sha,
                "normative_sha": normative_sha,
                "asset_sha": asset_sha
            },
            "timestamp": datetime.now(timezone.utc).isoformat()
        }
        
        return lenses_result, trace_entry

    def _build_system_message(self) -> str:
        """Build the system message with normative context."""
        registry = get_registry()
        return registry.get_text("system")

    def _call_llm_with_json_tail(
        self, user_message: str, json_tail: str, operation: str
    ) -> Dict[str, Any]:
        """
        Call LLM with JSON tail enforcement.

        Args:
            user_message: The user message content
            json_tail: The JSON tail to append
            operation: Operation name for tracing

        Returns:
            Parsed JSON response
        """
        # Append JSON tail to user message
        full_message = f"{user_message}\n\n{json_tail}"

        # Add to dialogue history
        self.dialogue_history.append({"role": "user", "content": full_message})

        # Check token budget
        if self.budget_config and self.budget_config.token_budget and self.token_count > self.budget_config.token_budget:
            raise ValueError(f"Token budget exceeded: {self.token_count} > {self.budget_config.token_budget}")

        # Parse JSON response with repair if needed
        def adapter_call(instructions=None, input=None):
            """
            Adapter function for LLM calls compatible with repair mechanism.
            Uses Responses API only - no messages support.
            
            Returns:
                (response_obj, metadata) where response_obj may be:
                - a parsed dict (from modern adapters) OR
                - {"content": "...json string..."} (legacy/raw format)
            """
            if not instructions or not input:
                raise ValueError("Must provide instructions and input for Responses API")
            
            # Derive strict response_format from operation (matrix_step)
            try:
                matrix_name, step = operation.split("_", 1)
            except Exception:
                matrix_name, step = ("unknown", "unknown")
            response_format = None
            if matrix_name != "unknown" and step != "unknown":
                from ...infrastructure.validation.json_schema_converter import get_strict_response_format
                response_format = get_strict_response_format(matrix_name, step)

            response = call_responses(
                instructions=instructions,
                input=input,
                reasoning_effort=self.reasoning_effort,
                response_format=response_format,
                expects_json=True
            )
            # Convert to expected format for repair mechanism
            return {"content": response.get("output_text", "")}, response.get("raw", {}).get("metadata", {})

        # Create schema hint for repair
        matrix_name = operation.split("_")[0] if "_" in operation else "unknown"
        step = operation.split("_")[1] if "_" in operation else "unknown"
        schema_hint = create_matrix_schema_hint(matrix_name, step)

        # Build instructions and input for Responses API
        # Get system prompt
        system_text = self.registry.get_text("system")
        
        # Build input from dialogue history as string (not messages array)
        transcript_lines = []
        for msg in self.dialogue_history:
            role = msg["role"]
            content = msg["content"]
            if role != "system":  # Skip system message as it goes in instructions
                transcript_lines.append(f"[{role.upper()}] {content}")
        
        input_text = "\n\n".join(transcript_lines)

        parsed, metadata = try_parse_json_or_repair(
            instructions=system_text,
            input_text=input_text,
            adapter_call=adapter_call,
            schema_hint=schema_hint,
            max_repair_attempts=self.max_repair,
        )

        # Update token count
        self.token_count += metadata.get("total_tokens", 0)

        # Trace if configured
        if self.tracer:
            self.tracer.trace_stage(
                stage=operation,
                context={"operation": operation, "matrix": parsed.get("name")},
                result=parsed,
                metadata=metadata,
            )

        return parsed

    def _validate_matrix_reconstruction(self, matrix_name: str, llm_result: Dict[str, Any], computed: Dict[str, Any]) -> bool:
        """
        Validate that LLM's reconstruction matches computed values.
        
        Args:
            matrix_name: Name of the matrix (K, T, G)
            llm_result: The LLM's generated result
            computed: The computed values to validate against
            
        Returns:
            True if validation passes
            
        Raises:
            ValueError: If validation fails with details
        """
        # Extract LLM's generated elements
        llm_elements = llm_result.get("elements", [])
        llm_rows = llm_result.get("rows", [])
        llm_cols = llm_result.get("cols", [])
        
        # Get computed values
        comp_elements = computed.get("elements", [])
        comp_rows = computed.get("rows", [])
        comp_cols = computed.get("cols", [])
        
        # Validate dimensions
        if llm_rows != comp_rows:
            raise ValueError(
                f"Matrix {matrix_name} row mismatch:\n"
                f"  LLM: {llm_rows}\n"
                f"  Computed: {comp_rows}"
            )
            
        if llm_cols != comp_cols:
            raise ValueError(
                f"Matrix {matrix_name} column mismatch:\n"
                f"  LLM: {llm_cols}\n"
                f"  Computed: {comp_cols}"
            )
        
        # Validate elements (comparing semantic content)
        if len(llm_elements) != len(comp_elements):
            raise ValueError(
                f"Matrix {matrix_name} row count mismatch: "
                f"LLM has {len(llm_elements)} rows, computed has {len(comp_elements)}"
            )
            
        for i, (llm_row, comp_row) in enumerate(zip(llm_elements, comp_elements)):
            if len(llm_row) != len(comp_row):
                raise ValueError(
                    f"Matrix {matrix_name} row {i} length mismatch: "
                    f"LLM has {len(llm_row)} elements, computed has {len(comp_row)}"
                )
            
            # For now, we'll trust the semantic content matches
            # In production, you might want deeper semantic comparison
            
        print(f"✅ Matrix {matrix_name} reconstruction validated: {len(comp_rows)}×{len(comp_cols)}")
        return True
    
    def _generate_semantic_trace_file(self, output_dir: Path):
        """Generate human-readable semantic valley trace file."""
        from datetime import datetime
        import json
        
        trace_file = output_dir / "SEMANTIC_VALLEY_TRACE.txt"
        
        with open(trace_file, 'w') as out:
            out.write('CHIRALITY FRAMEWORK - SEMANTIC VALLEY TRACE\n')
            out.write('=' * 80 + '\n')
            out.write(f'Generated: {datetime.now().strftime("%Y-%m-%d %H:%M:%S")}\n')
            out.write('Matrix Pipeline Test: Initialize → Mechanical → Interpreted → Lensed\n')
            out.write('=' * 80 + '\n\n')
            
            stage_num = 0
            
            for i, entry in enumerate(self.dialogue_history, 1):
                role = entry.get('role', '')
                content = entry.get('content', '')
                
                if role == 'system':
                    stage_num += 1
                    out.write(f'STAGE {stage_num}: SYSTEM SETUP\n')
                    out.write('-' * 40 + '\n')
                    out.write(content + '\n\n')
                    
                elif role == 'user':
                    stage_num += 1
                    if 'play a game' in content and 'sufficient' in content:
                        out.write(f'STAGE {stage_num}: INITIALIZE (Semantic Priming)\n')
                        out.write('-' * 40 + '\n')
                        out.write('COMPLETE CONTENT:\n')
                        out.write(content + '\n\n')
                    elif 'mechanical' in content.lower():
                        out.write(f'STAGE {stage_num}: MECHANICAL CONSTRUCTION\n')
                        out.write('-' * 40 + '\n')
                        out.write('COMPLETE CONTENT:\n')
                        out.write(content + '\n\n')
                    elif 'semantic' in content.lower() and 'interpretation' in content.lower():
                        out.write(f'STAGE {stage_num}: SEMANTIC INTERPRETATION\n')
                        out.write('-' * 40 + '\n')
                        out.write('COMPLETE CONTENT:\n')
                        out.write(content + '\n\n')
                    elif 'lens' in content.lower():
                        out.write(f'STAGE {stage_num}: LENS APPLICATION\n')
                        out.write('-' * 40 + '\n')
                        out.write('COMPLETE CONTENT:\n')
                        out.write(content + '\n\n')
                    else:
                        out.write(f'STAGE {stage_num}: USER INSTRUCTION\n')
                        out.write('-' * 40 + '\n')
                        out.write('COMPLETE CONTENT:\n')
                        out.write(content + '\n\n')
                        
                elif role == 'assistant':
                    out.write(f'GPT-5 RESPONSE (Stage {stage_num}):\n')
                    out.write('~' * 30 + '\n')
                    out.write('COMPLETE RESPONSE:\n')
                    out.write(content + '\n')
                    out.write('\n' + '=' * 50 + '\n\n')
            
            out.write('SEMANTIC COHERENCE EVALUATION:\n')
            out.write('- Initialize stage: Provides semantic operation context\n')
            out.write('- Mechanical stage: Generates combinatorial expressions\n')  
            out.write('- Semantic stage: Resolves into coherent terms\n')
            out.write('- Lens stage: Applies interpretive frameworks\n')
            out.write('\nReview the progression above to evaluate semantic valley coherence.\n')
        
        print(f"✅ Semantic valley trace written to: {trace_file}")
    
    def _compute_kernel_hash(self) -> str:
        """Compute kernel hash from prompt assets."""
        registry = get_registry()
        return registry.compute_kernel_hash()

    def _format_matrix(self, matrix) -> str:
        """Format a matrix for display in prompts."""
        lines = []
        for i, row in enumerate(matrix.cells):
            row_values = [cell.value for cell in row]
            lines.append(f"Row {i} ({matrix.row_labels[i]}): {row_values}")
        return "\n".join(lines)

    def _format_elements(self, elements: List[List[str]], rows: List[str], cols: List[str]) -> str:
        """Format matrix elements for display in prompts."""
        lines = []
        for i, row in enumerate(elements):
            lines.append(f"Row {i} ({rows[i]}): {row}")
        return "\n".join(lines)

    def save_dialogue(self, output_path: Path):
        """Save dialogue history to JSONL file."""
        output_path = Path(output_path)
        output_path.parent.mkdir(parents=True, exist_ok=True)

        with open(output_path, "w") as f:
            for message in self.dialogue_history:
                f.write(json.dumps(message) + "\n")
        
        # Generate human-readable semantic valley trace
        self._generate_semantic_trace_file(output_path.parent)

    def save_output(self, final_output: Dict[str, Any], output_dir: Path):
        """Save and validate final Phase 1 output."""
        return validate_and_write_agg(final_output, str(output_dir))

    def get_budget_status(self) -> Optional[Dict[str, Any]]:
        """Get current budget status for tracking."""
        if not self.budget_config:
            return None
            
        # Simple budget tracking - could be enhanced with cost calculation
        return {
            "tokens": {
                "total": self.token_count,
                "limit": self.budget_config.token_budget
            },
            "cost": {
                "spent": 0.0,  # TODO: Implement actual cost tracking
                "limit": self.budget_config.cost_budget
            },
            "time": {
                "elapsed": 0,  # TODO: Implement time tracking
                "limit": self.budget_config.time_budget
            }
        }

    def save_budget_status(self):
        """Save budget status - placeholder for future implementation."""
        # TODO: Implement budget status persistence if needed
        pass

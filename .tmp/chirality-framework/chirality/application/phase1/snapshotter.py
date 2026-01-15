"""
Phase 1 Snapshot Generator.

Converts dialogue history into a compact snapshot that serves
as the system prompt for Phase 2 tensor operations.
"""

import json
import hashlib
from typing import List, Dict, Any
from pathlib import Path
from datetime import datetime, timezone


class SnapshotGenerator:
    """
    Generates phase1_snapshot.md from dialogue history.

    The snapshot captures:
    - Semantic operation definitions and examples
    - Station contexts and transformations
    - Matrix computation results
    - Key semantic patterns discovered

    This becomes the complete context for Phase 2.
    """

    def __init__(self, max_tokens: int = 16000):
        """
        Initialize snapshot generator.

        Args:
            max_tokens: Maximum token budget for snapshot (approximate)
        """
        self.max_tokens = max_tokens

    def generate_snapshot(
        self, dialogue_path: Path, output_path: Path, phase1_output: Dict[str, Any]
    ) -> str:
        """
        Generate snapshot from dialogue history.

        Args:
            dialogue_path: Path to phase1_dialogue.jsonl
            output_path: Path to write phase1_snapshot.md
            phase1_output: The aggregated Phase 1 output

        Returns:
            Snapshot hash for provenance
        """
        # Load dialogue history
        dialogue = self._load_dialogue(dialogue_path)

        # Extract key elements
        sections = []

        # Add YAML front matter
        front_matter = self._generate_front_matter(phase1_output)
        sections.append(front_matter)

        # Add title and overview
        sections.append("# Chirality Framework Phase 1 Implementation\n")
        sections.append(self._generate_overview())

        # Extract semantic operation examples
        sections.append("## Semantic Operations\n")
        sections.append(self._extract_semantic_operations(dialogue))

        # Extract station contexts
        sections.append("## Station Contexts\n")
        sections.append(self._extract_station_contexts(dialogue))

        # Add matrix definitions
        sections.append("## Matrix Definitions\n")
        sections.append(self._format_matrix_definitions(phase1_output))

        # Add key transformations
        sections.append("## Key Transformations\n")
        sections.append(self._extract_transformations(dialogue))

        # Add semantic patterns
        sections.append("## Semantic Patterns\n")
        sections.append(self._extract_patterns(dialogue, phase1_output))

        # Combine and trim to budget
        snapshot_content = "\n\n".join(sections)
        snapshot_content = self._trim_to_budget(snapshot_content)

        # Compute snapshot hash
        snapshot_hash = hashlib.sha256(snapshot_content.encode()).hexdigest()

        # Update front matter with hash
        snapshot_content = snapshot_content.replace(
            "snapshot_hash: pending", f"snapshot_hash: {snapshot_hash}"
        )

        # Write snapshot
        output_path = Path(output_path)
        output_path.parent.mkdir(parents=True, exist_ok=True)
        output_path.write_text(snapshot_content)

        return snapshot_hash

    def _load_dialogue(self, dialogue_path: Path) -> List[Dict[str, Any]]:
        """Load dialogue history from JSONL."""
        dialogue = []
        with open(dialogue_path, "r") as f:
            for line in f:
                dialogue.append(json.loads(line))
        return dialogue

    def _generate_front_matter(self, phase1_output: Dict[str, Any]) -> str:
        """Generate YAML front matter for snapshot."""
        meta = phase1_output.get("meta", {})
        return f"""---
type: phase1_snapshot
version: 19.2.0
generated: {datetime.now(timezone.utc).isoformat()}
kernel_hash: {meta.get('kernel_hash', 'unknown')}
snapshot_hash: pending
model: {meta.get('model', 'unknown')}
token_count: {meta.get('token_count', 0)}
---"""

    def _generate_overview(self) -> str:
        """Generate overview section."""
        return """
This snapshot captures the complete Phase 1 implementation through Matrix E.
It serves as the system prompt for Phase 2 tensor operations.

The implementation uses conversational prompting to build semantic understanding
progressively through dialogue, establishing the semantic operations and their
application to the canonical matrices.
"""

    def _extract_semantic_operations(self, dialogue: List[Dict]) -> str:
        """Extract semantic operation definitions and examples."""
        operations = []

        # Look for semantic multiplication examples
        operations.append("### Semantic Multiplication (*)")
        operations.append("Combines word meanings into coherent intersection:")

        # Extract examples from dialogue
        for msg in dialogue:
            content = msg.get("content", "")
            if "semantic multiplication" in content.lower():
                # Extract examples (simplified - would need proper parsing)
                if "*" in content and "=" in content:
                    lines = content.split("\n")
                    for line in lines[:5]:  # First few examples
                        if "*" in line and "=" in line:
                            operations.append(f"- {line.strip()}")

        operations.append("\n### Semantic Addition (+)")
        operations.append("Concatenates elements into coherent statements:")
        operations.append("- Used in Matrix D construction")

        operations.append("\n### Semantic Dot Product (·)")
        operations.append("Matrix multiplication using semantic operations")

        operations.append("\n### Element-wise Product (⊙)")
        operations.append("Pairwise semantic multiplication")

        operations.append("\n### Station Shift (→)")
        operations.append("Context transformation between stations")

        return "\n".join(operations)

    def _extract_station_contexts(self, dialogue: List[Dict]) -> str:
        """Extract station contexts from dialogue."""
        stations = []

        station_map = {
            "Problem Statement": "Understanding the problem space",
            "Requirements": "Defining what is needed",
            "Objectives": "Setting solution goals",
            "Verification": "Checking solution validity",
            "Validation": "Confirming solution value",
            "Evaluation": "Assessing solution quality",
        }

        for station, description in station_map.items():
            stations.append(f"### {station}")
            stations.append(f"{description}")
            stations.append("")

        return "\n".join(stations)

    def _format_matrix_definitions(self, phase1_output: Dict[str, Any]) -> str:
        """Format matrix definitions from Phase 1 output."""
        definitions = []
        matrices = phase1_output.get("matrices", {})

        for name, matrix in matrices.items():
            definitions.append(f"### Matrix {name}")
            definitions.append(f"- Station: {matrix.get('station', 'Unknown')}")
            definitions.append(
                f"- Dimensions: {len(matrix.get('rows', []))}×{len(matrix.get('cols', []))}"
            )
            definitions.append(f"- Rows: {', '.join(matrix.get('rows', []))}")
            definitions.append(f"- Columns: {', '.join(matrix.get('cols', []))}")

            # Add sample elements (first row)
            elements = matrix.get("elements", [])
            if elements and len(elements) > 0:
                first_row = elements[0]
                if isinstance(first_row, list) and len(first_row) > 0:
                    definitions.append(f"- Sample: {first_row[0][:100]}...")  # First 100 chars

            definitions.append("")

        return "\n".join(definitions)

    def _extract_transformations(self, dialogue: List[Dict]) -> str:
        """Extract key transformations from dialogue."""
        transformations = []

        transformations.append("### Matrix Computations")
        transformations.append("- C = A · B (Problem Statement)")
        transformations.append("- F = C ⊙ J (Requirements)")
        transformations.append("- D = A + F (Objectives with canonical formula)")
        transformations.append("- K = transpose(D)")
        transformations.append("- X = K · J (Verification)")
        transformations.append("- Z = shift(X) (Validation)")
        transformations.append("- G = Z[0:3, :] (First 3 rows)")
        transformations.append("- P = Z[3, :] (Fourth row)")
        transformations.append("- T = transpose(J)")
        transformations.append("- E = G · T (Evaluation)")

        return "\n".join(transformations)

    def _extract_patterns(self, dialogue: List[Dict], phase1_output: Dict[str, Any]) -> str:
        """Extract semantic patterns discovered."""
        patterns = []

        patterns.append("### Key Semantic Patterns")

        # Extract principles if available
        principles = phase1_output.get("principles", {}).get("items", [])
        if principles:
            patterns.append("\n#### Validation Principles")
            for i, principle in enumerate(principles[:5], 1):  # First 5
                patterns.append(f"{i}. {principle}")

        # Look for recurring semantic combinations
        patterns.append("\n#### Common Semantic Combinations")
        patterns.append("- Normative × Necessity → Standards and requirements")
        patterns.append("- Operative × Sufficiency → Practical implementations")
        patterns.append("- Iterative × Completeness → Continuous improvement")

        return "\n".join(patterns)

    def _trim_to_budget(self, content: str) -> str:
        """Trim content to fit token budget."""
        # Simple character-based approximation (4 chars ≈ 1 token)
        max_chars = self.max_tokens * 4

        if len(content) <= max_chars:
            return content

        # Trim from the end, keeping structure intact
        lines = content.split("\n")
        trimmed = []
        char_count = 0

        for line in lines:
            if char_count + len(line) > max_chars:
                trimmed.append("\n... [Content trimmed to fit token budget]")
                break
            trimmed.append(line)
            char_count += len(line) + 1

        return "\n".join(trimmed)

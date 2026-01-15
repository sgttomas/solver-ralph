"""
Post-hoc Neo4j Exporter.

Offline loader that reads artifacts and creates comprehensive
graph representation of Chirality Framework runs.
"""

import json
import hashlib
from typing import Dict, List, Any, Optional
from pathlib import Path
from datetime import datetime

try:
    from neo4j import GraphDatabase

    NEO4J_AVAILABLE = True
except ImportError:
    NEO4J_AVAILABLE = False


class Neo4jLoader:
    """
    Post-hoc loader for Chirality Framework artifacts.

    Creates comprehensive graph with:
    - Nodes: :Run, :Component:Matrix|:Tensor, :Cell, :Lens
    - Relationships: :CONTAINS, :DERIVED_FROM, :LENSED_BY
    - Constraints and indexes for performance
    """

    def __init__(self, uri: str, user: str, password: str):
        """
        Initialize Neo4j loader.

        Args:
            uri: Neo4j connection URI (e.g., bolt://localhost:7687)
            user: Neo4j username
            password: Neo4j password
        """
        if not NEO4J_AVAILABLE:
            raise ImportError("neo4j package required. Install with: pip install neo4j")

        self.driver = GraphDatabase.driver(uri, auth=(user, password))

    def load_artifacts(self, artifacts_dir: Path, run_id: Optional[str] = None) -> str:
        """
        Load all artifacts for a run into Neo4j.

        Args:
            artifacts_dir: Path to artifacts directory
            run_id: Optional run ID (generated if not provided)

        Returns:
            Run ID that was created
        """
        artifacts_dir = Path(artifacts_dir)

        # Generate run ID if not provided
        if not run_id:
            run_id = f"run_{datetime.now().strftime('%Y%m%d_%H%M%S')}"

        # Load artifact files
        phase1_output = self._load_json(artifacts_dir / "phase1_output.json")
        phase2_output = self._load_json(artifacts_dir / "phase2_output.json")
        lens_catalog = self._load_jsonl(artifacts_dir / "lens_catalog.jsonl")
        run_manifest = self._load_json(artifacts_dir / "run_manifest.json")

        # Load cell traces
        cell_traces_dir = artifacts_dir / "cell_traces"
        cell_traces = {}
        if cell_traces_dir.exists():
            for tensor_dir in cell_traces_dir.iterdir():
                if tensor_dir.is_dir():
                    tensor_name = tensor_dir.name
                    cell_traces[tensor_name] = {}
                    for trace_file in tensor_dir.glob("*.json"):
                        with open(trace_file, "r") as f:
                            trace = json.load(f)
                        cell_key = trace_file.stem  # e.g., "0_1_2"
                        cell_traces[tensor_name][cell_key] = trace

        # Create graph in transaction
        with self.driver.session() as session:
            session.execute_write(
                self._create_run_graph,
                run_id,
                phase1_output,
                phase2_output,
                lens_catalog,
                cell_traces,
                run_manifest,
            )

        return run_id

    def setup_constraints(self):
        """Create constraints and indexes."""
        constraints = [
            "CREATE CONSTRAINT run_id_unique IF NOT EXISTS FOR (r:Run) REQUIRE r.run_id IS UNIQUE",
            "CREATE CONSTRAINT component_unique IF NOT EXISTS FOR (c:Component) REQUIRE (c.run_id, c.name) IS UNIQUE",
            "CREATE CONSTRAINT cell_id_unique IF NOT EXISTS FOR (c:Cell) REQUIRE c.id IS UNIQUE",
            "CREATE CONSTRAINT lens_id_unique IF NOT EXISTS FOR (l:Lens) REQUIRE l.lens_id IS UNIQUE",
        ]

        with self.driver.session() as session:
            for constraint in constraints:
                try:
                    session.run(constraint)
                    print(
                        f"✓ Created constraint: {constraint.split(' FOR ')[1].split(' REQUIRE')[0]}"
                    )
                except Exception as e:
                    # Don't fail on already exists errors
                    if (
                        "already exists" not in str(e).lower()
                        and "equivalent" not in str(e).lower()
                    ):
                        print(f"⚠ Constraint creation failed: {e}")
                    else:
                        print(
                            f"✓ Constraint already exists: {constraint.split(' FOR ')[1].split(' REQUIRE')[0]}"
                        )

    def _create_run_graph(
        self,
        tx,
        run_id: str,
        phase1_output: Dict[str, Any],
        phase2_output: Optional[Dict[str, Any]],
        lens_catalog: List[Dict[str, Any]],
        cell_traces: Dict[str, Dict[str, Any]],
        run_manifest: Optional[Dict[str, Any]],
    ):
        """Create complete graph for a run in single transaction."""

        # 1. Create Run node
        meta = phase1_output.get("meta", {})
        run_props = {
            "run_id": run_id,
            "kernel_hash": meta.get("kernel_hash"),
            "snapshot_hash": meta.get("snapshot_hash"),
            "model": meta.get("model"),
            "started_at": meta.get("timestamp"),
            "phase1_complete": True,
            "phase2_complete": phase2_output is not None,
        }

        if run_manifest:
            run_props.update(
                {
                    "token_count": run_manifest.get("total_tokens", 0),
                    "cost": run_manifest.get("total_cost", 0.0),
                    "parallel": run_manifest.get("parallel", 1),
                }
            )

        tx.run("CREATE (r:Run $props)", props=run_props)

        # 2. Create Lens nodes (shared across components)
        for lens in lens_catalog:
            lens_props = {
                "lens_id": lens["lens_id"],
                "row": lens["row"],
                "col": lens["col"],
                "station": lens["station"],
                "text": lens["text"],
                "kernel_hash": lens["kernel_hash"],
                "model": lens["model"],
                "prompt_version": lens["prompt_version"],
            }
            tx.run(
                "MERGE (l:Lens {lens_id: $lens_id}) SET l += $props",
                lens_id=lens["lens_id"],
                props=lens_props,
            )

        # 3. Create Phase 1 matrices
        matrices = phase1_output.get("matrices", {})
        for name, matrix in matrices.items():
            self._create_matrix_component(tx, run_id, name, matrix)

        # 4. Create Phase 2 tensors if available
        if phase2_output:
            tensors = phase2_output.get("tensors", {})
            for name, tensor in tensors.items():
                self._create_tensor_component(tx, run_id, name, tensor, cell_traces.get(name, {}))

    def _create_matrix_component(self, tx, run_id: str, name: str, matrix: Dict[str, Any]):
        """Create matrix component and its cells."""

        # Create Component node with dual label
        rows = matrix.get("rows", [])
        cols = matrix.get("cols", [])
        component_props = {
            "run_id": run_id,
            "name": name,
            "rank": 2,
            "dims": [len(rows), len(cols)],
            "station": matrix.get("station"),
            "step": matrix.get("step"),
            "op": matrix.get("op"),
        }

        tx.run(
            """
        MATCH (r:Run {run_id: $run_id})
        CREATE (c:Component:Matrix $props)
        CREATE (r)-[:CONTAINS]->(c)
        """,
            run_id=run_id,
            props=component_props,
        )

        # Create Cell nodes
        elements = matrix.get("elements", [])
        for i, row_elements in enumerate(elements):
            for j, value in enumerate(row_elements):
                cell_id = f"{name}[{i},{j}]"
                cell_props = {
                    "id": cell_id,
                    "idx": [i, j],
                    "value": value,
                    "op": matrix.get("op"),
                    "station": matrix.get("station"),
                }

                # Determine lens_id if this was lensed
                if matrix.get("step") == "lensed" and i < len(rows) and j < len(cols):
                    lens_id = self._compute_lens_id(rows[i], cols[j], matrix.get("station", ""))
                    cell_props["lens_id"] = lens_id

                # Create cell and link to component
                tx.run(
                    """
                MATCH (c:Component {run_id: $run_id, name: $name})
                CREATE (cell:Cell $cell_props)
                CREATE (c)-[:CONTAINS]->(cell)
                """,
                    run_id=run_id,
                    name=name,
                    cell_props=cell_props,
                )

                # Link to lens if applicable
                if "lens_id" in cell_props:
                    tx.run(
                        """
                    MATCH (cell:Cell {id: $cell_id})
                    MATCH (l:Lens {lens_id: $lens_id})
                    CREATE (cell)-[:LENSED_BY]->(l)
                    """,
                        cell_id=cell_id,
                        lens_id=cell_props["lens_id"],
                    )

    def _create_tensor_component(
        self, tx, run_id: str, name: str, tensor: Dict[str, Any], traces: Dict[str, Any]
    ):
        """Create tensor component and its cells."""

        # Create Component node with dual label
        dims = tensor.get("dims", [])
        component_props = {
            "run_id": run_id,
            "name": name,
            "rank": len(dims),
            "dims": dims,
            "op": tensor.get("op"),
            "matrix_operand": tensor.get("matrix_operand"),
        }

        tx.run(
            """
        MATCH (r:Run {run_id: $run_id})
        CREATE (c:Component:Tensor $props)
        CREATE (r)-[:CONTAINS]->(c)
        """,
            run_id=run_id,
            props=component_props,
        )

        # Create Cell nodes from tensor cells
        cells = tensor.get("cells", {})
        for idx_tuple, cell_data in cells.items():
            # Convert tuple back to list if needed
            if isinstance(idx_tuple, str):
                # Parse string representation of tuple
                idx = eval(idx_tuple)  # Careful - only for trusted data
            else:
                idx = list(idx_tuple)

            cell_id = f"{name}[{','.join(map(str, idx))}]"
            cell_props = {
                "id": cell_id,
                "idx": idx,
                "value": cell_data.get("value", ""),
                "op": "cross",
                "left_operand": cell_data.get("left_operand", ""),
                "right_operand": cell_data.get("right_operand", ""),
            }

            # Add lens info if available
            if "lens_id" in cell_data.get("metadata", {}):
                cell_props["lens_id"] = cell_data["metadata"]["lens_id"]

            # Create cell and link to component
            tx.run(
                """
            MATCH (c:Component {run_id: $run_id, name: $name})
            CREATE (cell:Cell $cell_props)
            CREATE (c)-[:CONTAINS]->(cell)
            """,
                run_id=run_id,
                name=name,
                cell_props=cell_props,
            )

            # Link to lens if applicable
            if "lens_id" in cell_props:
                tx.run(
                    """
                MATCH (cell:Cell {id: $cell_id})
                MATCH (l:Lens {lens_id: $lens_id})
                CREATE (cell)-[:LENSED_BY]->(l)
                """,
                    cell_id=cell_id,
                    lens_id=cell_props["lens_id"],
                )

            # Create lineage links based on sources
            sources = cell_data.get("sources", [])
            for source_id in sources:
                try:
                    tx.run(
                        """
                    MATCH (cell:Cell {id: $cell_id})
                    MATCH (source:Cell {id: $source_id})
                    CREATE (cell)-[:DERIVED_FROM]->(source)
                    """,
                        cell_id=cell_id,
                        source_id=source_id,
                    )
                except Exception:
                    # Source cell might not exist yet - skip for now
                    # In production, would need topological ordering
                    pass

    def _compute_lens_id(self, row: str, col: str, station: str) -> str:
        """Compute lens ID for matching."""
        # This should match the computation in lens derivation
        lens_str = f"{row}|{col}|{station}"
        return hashlib.sha256(lens_str.encode()).hexdigest()

    def _load_json(self, path: Path) -> Optional[Dict[str, Any]]:
        """Load JSON file safely."""
        if not path.exists():
            return None
        with open(path, "r") as f:
            return json.load(f)

    def _load_jsonl(self, path: Path) -> List[Dict[str, Any]]:
        """Load JSONL file safely."""
        if not path.exists():
            return []

        items = []
        with open(path, "r") as f:
            for line in f:
                items.append(json.loads(line))
        return items

    def close(self):
        """Close Neo4j connection."""
        self.driver.close()

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.close()


def load_artifacts_to_neo4j(
    artifacts_dir: Path,
    uri: str,
    user: str,
    password: str,
    run_id: Optional[str] = None,
    setup_constraints: bool = True,
) -> str:
    """
    Convenience function to load artifacts into Neo4j.

    Args:
        artifacts_dir: Path to artifacts directory
        uri: Neo4j URI
        user: Neo4j username
        password: Neo4j password
        run_id: Optional run ID
        setup_constraints: Whether to create constraints

    Returns:
        Run ID that was created
    """
    with Neo4jLoader(uri, user, password) as loader:
        if setup_constraints:
            loader.setup_constraints()

        return loader.load_artifacts(artifacts_dir, run_id)

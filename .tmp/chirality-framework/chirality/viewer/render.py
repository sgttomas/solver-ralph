"""
Matrix Snapshot Viewer Generator

This module generates static HTML pages from matrix snapshot files,
providing an elegant web interface for viewing matrix computation results.
"""

import json
import tempfile
import re
from pathlib import Path
from typing import Dict, List, Optional
import html
from datetime import datetime

# Canonical matrix order for consistent display
CANONICAL_ORDER = ["A", "B", "J", "C", "F", "D", "K", "X", "Z", "G", "P", "T", "E"]

# Static CSS for the viewer page
VIEWER_CSS = """
/* Chirality Framework Matrix Viewer Styles */
* {
    box-sizing: border-box;
}

body {
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    line-height: 1.6;
    color: #333;
    max-width: 100%;
    margin: 0;
    padding: 20px;
    background-color: #f8f9fa;
}

.header {
    text-align: center;
    margin-bottom: 30px;
    padding: 20px;
    background: white;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.header h1 {
    color: #2c3e50;
    margin: 0 0 10px 0;
}

.run-info {
    color: #7f8c8d;
    font-size: 0.9em;
}

.navigation {
    margin-bottom: 30px;
    padding: 15px;
    background: white;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    text-align: center;
}

.navigation h2 {
    margin: 0 0 15px 0;
    color: #34495e;
    font-size: 1.2em;
}

.nav-links {
    display: flex;
    flex-wrap: wrap;
    gap: 10px;
    justify-content: center;
}

.nav-links a {
    padding: 8px 16px;
    background: #3498db;
    color: white;
    text-decoration: none;
    border-radius: 4px;
    font-weight: 500;
    transition: background-color 0.2s;
}

.nav-links a:hover {
    background: #2980b9;
}

.nav-links a.not-found {
    background: #bdc3c7;
    color: #7f8c8d;
    cursor: not-allowed;
}

.matrix-section {
    margin-bottom: 40px;
    background: white;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    overflow: hidden;
}

.matrix-header {
    background: #34495e;
    color: white;
    padding: 15px 20px;
    margin: 0;
}

.matrix-info {
    padding: 15px 20px;
    background: #ecf0f1;
    border-bottom: 1px solid #bdc3c7;
}

.matrix-info p {
    margin: 5px 0;
    font-size: 0.9em;
}

.matrix-table {
    width: 100%;
    border-collapse: collapse;
    margin: 0;
}

.matrix-table caption {
    caption-side: top;
    padding: 10px;
    font-weight: bold;
    color: #2c3e50;
    background: #f8f9fa;
}

.matrix-table th,
.matrix-table td {
    border: 1px solid #bdc3c7;
    padding: 12px;
    text-align: left;
    vertical-align: top;
    word-wrap: break-word;
    max-width: 300px;
}

.matrix-table th {
    background: #ecf0f1;
    font-weight: 600;
    color: #2c3e50;
}

.matrix-table tbody tr:nth-child(even) {
    background: #f8f9fa;
}

.matrix-table tbody tr:nth-child(odd) {
    background: white;
}

.matrix-table tbody tr:hover {
    background: #e8f4f8;
}

.cell-value {
    font-weight: 500;
    margin-bottom: 8px;
}

.cell-coords {
    color: #7f8c8d;
    font-size: 0.8em;
    font-family: monospace;
}

.provenance {
    margin-top: 8px;
    padding-top: 8px;
    border-top: 1px solid #ecf0f1;
    font-size: 0.85em;
    color: #7f8c8d;
}

.operation-info {
    font-family: monospace;
    background: #f8f9fa;
    padding: 4px 8px;
    border-radius: 3px;
    display: inline-block;
    margin-bottom: 4px;
}

.not-found {
    padding: 40px 20px;
    text-align: center;
    color: #7f8c8d;
    font-style: italic;
}

.footer {
    text-align: center;
    margin-top: 40px;
    padding: 20px;
    color: #7f8c8d;
    font-size: 0.9em;
    background: white;
    border-radius: 8px;
}

@media (max-width: 768px) {
    body {
        padding: 10px;
    }
    
    .nav-links {
        gap: 5px;
    }
    
    .nav-links a {
        padding: 6px 12px;
        font-size: 0.9em;
    }
    
    .matrix-table th,
    .matrix-table td {
        padding: 8px;
        max-width: 200px;
    }
}
"""


def sanitize_value(value: str) -> str:
    """
    Sanitize cell values by removing common prefixes and cleaning up text.

    Args:
        value: Raw cell value from snapshot

    Returns:
        Cleaned value with prefixes removed
    """
    if not isinstance(value, str):
        return str(value)

    # Strip common prefixes using regex patterns
    patterns = [
        r"^VALIDATION\[[^\]]*\]\s*",  # VALIDATION[...] prefix
        r"^COL\[[^\]]*\]\s*",  # COL[...] prefix
        r"^ROW\[[^\]]*\]\s*",  # ROW[...] prefix
        r"^EVAL\[[^\]]*\]\s*",  # EVAL[...] prefix
        r"^VERIFY\[[^\]]*\]\s*",  # VERIFY[...] prefix
        r"^SYNTH\[[^\]]*\]\s*",  # SYNTH[...] prefix
        r"^LENS\[[^\]]*\]\s*",  # LENS[...] prefix
    ]

    cleaned = value
    for pattern in patterns:
        cleaned = re.sub(pattern, "", cleaned)

    # Strip extra whitespace and return
    return cleaned.strip()


def load_matrix_snapshot(snapshot_path: Path) -> Dict:
    """
    Load and parse a matrix snapshot from JSONL file.
    Reads only the first non-empty line and validates grid reconstruction.

    Args:
        snapshot_path: Path to the snapshot JSONL file

    Returns:
        Dictionary containing the parsed snapshot data

    Raises:
        FileNotFoundError: If the snapshot file doesn't exist
        json.JSONDecodeError: If the file contains invalid JSON
    """
    if not snapshot_path.exists():
        raise FileNotFoundError(f"Snapshot file not found: {snapshot_path}")

    with open(snapshot_path, "r") as f:
        # Read only the first non-empty line for JSONL format
        for line in f:
            line = line.strip()
            if line:
                snapshot_data = json.loads(line)

                # Validate grid reconstruction against shape
                shape = snapshot_data.get("shape", [0, 0])
                cells = snapshot_data.get("cells", [])
                expected_cells = shape[0] * shape[1] if len(shape) == 2 else 0

                if len(cells) != expected_cells:
                    print(
                        f"Warning: Shape {shape} expects {expected_cells} cells, but found {len(cells)}"
                    )

                return snapshot_data

        # If no non-empty lines found, return empty data
        raise json.JSONDecodeError("No valid JSON data found in file", "", 0)


def render_matrix_table(snapshot_data: Dict, matrix_name: str) -> str:
    """
    Generate semantically correct HTML table for a matrix snapshot.

    Args:
        snapshot_data: Parsed snapshot data containing matrix information
        matrix_name: Name of the matrix for the table caption

    Returns:
        HTML string representing the matrix as a table
    """
    # Check shape first
    shape = snapshot_data.get("shape", [0, 0])
    if len(shape) != 2 or shape[0] == 0 or shape[1] == 0:
        return f'<div class="not-found">Matrix {matrix_name}: Invalid shape {shape}</div>'

    if not snapshot_data.get("cells"):
        return f'<div class="not-found">Matrix {matrix_name}: No cell data available</div>'

    rows, cols = shape
    row_labels = snapshot_data.get("row_labels", [])
    col_labels = snapshot_data.get("col_labels", [])

    # Create 2D grid initialized with None
    grid = [[None for _ in range(cols)] for _ in range(rows)]

    # Fill grid from cells array
    for cell_data in snapshot_data["cells"]:
        row = cell_data.get("row")
        col = cell_data.get("col")
        if row is not None and col is not None and 0 <= row < rows and 0 <= col < cols:
            grid[row][col] = cell_data

    # Build HTML table
    html_parts = []
    html_parts.append(f'<table class="matrix-table" id="{matrix_name.lower()}">')
    html_parts.append(f"<caption>Matrix {html.escape(matrix_name)}</caption>")

    # Table header
    html_parts.append("<thead>")
    html_parts.append("<tr>")
    html_parts.append('<th scope="col"></th>')  # Empty cell for row headers
    for j, col_label in enumerate(col_labels):
        escaped_label = html.escape(str(col_label))
        html_parts.append(f'<th scope="col">{escaped_label}</th>')
    html_parts.append("</tr>")
    html_parts.append("</thead>")

    # Table body
    html_parts.append("<tbody>")
    for i in range(rows):
        html_parts.append("<tr>")

        # Row header
        row_label = row_labels[i] if i < len(row_labels) else f"Row {i}"
        escaped_row_label = html.escape(str(row_label))
        html_parts.append(f'<th scope="row">{escaped_row_label}</th>')

        # Cells
        for j in range(cols):
            cell_data = grid[i][j]
            if cell_data:
                value = html.escape(str(cell_data.get("value", "")))
                coords = f"({i},{j})"
                provenance = cell_data.get("provenance", {})
                operation = html.escape(str(provenance.get("operation", "unknown")))
                timestamp = html.escape(str(provenance.get("timestamp", "")))

                cell_html = f"""
                <div class="cell-value">{value}</div>
                <div class="cell-coords">{coords}</div>
                <div class="provenance">
                    <div class="operation-info">{operation}</div>
                    <div>{timestamp}</div>
                </div>
                """
                html_parts.append(f"<td>{cell_html}</td>")
            else:
                html_parts.append('<td><div class="not-found">No data</div></td>')

        html_parts.append("</tr>")
    html_parts.append("</tbody>")
    html_parts.append("</table>")

    return "\n".join(html_parts)


def render_page(
    snapshot_data_by_matrix: Dict[str, Dict], run_id: str, title: Optional[str] = None
) -> str:
    """
    Generate complete HTML page with navigation and all matrices.

    Args:
        snapshot_data_by_matrix: Dictionary mapping matrix names to snapshot data
        run_id: Run ID for the header
        title: Optional custom title for the page

    Returns:
        Complete HTML page as a string
    """
    page_title = title if title else "Chirality Framework - Matrix Viewer"

    html_parts = []
    html_parts.append("<!DOCTYPE html>")
    html_parts.append('<html lang="en">')
    html_parts.append("<head>")
    html_parts.append('<meta charset="UTF-8">')
    html_parts.append('<meta name="viewport" content="width=device-width, initial-scale=1.0">')
    html_parts.append(f"<title>{html.escape(page_title)}</title>")
    html_parts.append("<style>")
    html_parts.append(VIEWER_CSS)
    html_parts.append("</style>")
    html_parts.append("</head>")
    html_parts.append("<body>")

    # Header
    html_parts.append('<div class="header">')
    html_parts.append(f"<h1>{html.escape(page_title)}</h1>")
    html_parts.append(f'<div class="run-info">Run ID: {html.escape(run_id)}</div>')

    # Determine resolver from first available snapshot
    resolver_name = "Unknown"
    for snapshot_data in snapshot_data_by_matrix.values():
        if snapshot_data.get("resolver"):
            resolver_name = snapshot_data["resolver"]
            break
    html_parts.append(f'<div class="run-info">Resolver: {html.escape(resolver_name)}</div>')
    html_parts.append(
        f'<div class="run-info">Generated: {datetime.now().strftime("%Y-%m-%d %H:%M:%S")}</div>'
    )
    html_parts.append("</div>")

    # Navigation
    html_parts.append('<div class="navigation">')
    html_parts.append("<h2>Quick Navigation</h2>")
    html_parts.append('<div class="nav-links">')

    for matrix_name in CANONICAL_ORDER:
        if matrix_name in snapshot_data_by_matrix:
            html_parts.append(f'<a href="#{matrix_name.lower()}">{matrix_name}</a>')
        else:
            html_parts.append(f'<a href="#" class="not-found">{matrix_name}</a>')

    html_parts.append("</div>")
    html_parts.append("</div>")

    # Matrix sections in canonical order
    for matrix_name in CANONICAL_ORDER:
        html_parts.append(f'<div class="matrix-section" id="{matrix_name.lower()}">')

        if matrix_name in snapshot_data_by_matrix:
            snapshot_data = snapshot_data_by_matrix[matrix_name]

            # Matrix header
            html_parts.append(f'<h2 class="matrix-header">Matrix {matrix_name}</h2>')

            # Matrix info
            html_parts.append('<div class="matrix-info">')
            station = html.escape(str(snapshot_data.get("station", "Unknown")))
            shape = snapshot_data.get("shape", [0, 0])
            resolver = html.escape(str(snapshot_data.get("resolver", "Unknown")))
            timestamp = html.escape(str(snapshot_data.get("timestamp", "Unknown")))
            cell_count = len(snapshot_data.get("cells", []))

            html_parts.append(f"<p><strong>Station:</strong> {station}</p>")
            html_parts.append(
                f"<p><strong>Shape:</strong> {shape[0]}×{shape[1]} ({cell_count} cells)</p>"
            )
            html_parts.append(f"<p><strong>Resolver:</strong> {resolver}</p>")
            html_parts.append(f"<p><strong>Timestamp:</strong> {timestamp}</p>")
            html_parts.append("</div>")

            # Matrix table
            table_html = render_matrix_table(snapshot_data, matrix_name)
            html_parts.append(table_html)
        else:
            # Missing matrix
            html_parts.append(f'<h2 class="matrix-header">Matrix {matrix_name}</h2>')
            html_parts.append('<div class="not-found">Snapshot not found</div>')

        html_parts.append("</div>")

    # Footer
    html_parts.append('<div class="footer">')
    html_parts.append("Generated by Chirality Framework Matrix Viewer")
    html_parts.append("</div>")

    html_parts.append("</body>")
    html_parts.append("</html>")

    return "\n".join(html_parts)


def render_elements_page(
    snapshot_data_by_matrix: Dict[str, Dict],
    run_id: str,
    title: Optional[str] = None,
    sanitize: bool = False,
) -> str:
    """
    Generate Elements-style HTML page with nested list display.

    Args:
        snapshot_data_by_matrix: Dictionary mapping matrix names to snapshot data
        run_id: Run ID for the header
        title: Optional custom title for the page
        sanitize: Whether to sanitize cell values by removing prefixes

    Returns:
        Complete HTML page as a string with Elements styling
    """
    page_title = title if title else "Chirality Framework - Elements View"

    html_parts = []
    html_parts.append("<!DOCTYPE html>")
    html_parts.append('<html lang="en">')
    html_parts.append("<head>")
    html_parts.append('<meta charset="UTF-8">')
    html_parts.append('<meta name="viewport" content="width=device-width, initial-scale=1.0">')
    html_parts.append(f"<title>{html.escape(page_title)}</title>")
    html_parts.append("<style>")

    # Elements-specific CSS
    elements_css = """
body {
    font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
    line-height: 1.4;
    color: #2c3e50;
    max-width: 1200px;
    margin: 0 auto;
    padding: 20px;
    background: #fafafa;
}

.header {
    text-align: center;
    margin-bottom: 40px;
    padding: 20px;
    background: white;
    border: 2px solid #34495e;
    border-radius: 4px;
}

.header h1 {
    color: #34495e;
    margin: 0 0 10px 0;
    font-size: 2em;
}

.run-info {
    color: #7f8c8d;
    font-size: 0.9em;
    margin: 5px 0;
}

.matrix-section {
    margin-bottom: 40px;
    background: white;
    border: 1px solid #bdc3c7;
    padding: 20px;
}

.matrix-header {
    color: #2c3e50;
    margin: 0 0 20px 0;
    padding-bottom: 10px;
    border-bottom: 2px solid #ecf0f1;
    font-size: 1.5em;
}

.matrix-info {
    margin-bottom: 20px;
    padding: 10px;
    background: #f8f9fa;
    border-left: 4px solid #3498db;
    font-size: 0.9em;
}

.elements-grid {
    font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
    background: #f8f9fa;
    border: 1px solid #bdc3c7;
    padding: 15px;
    overflow-x: auto;
}

.elements-grid pre {
    margin: 0;
    white-space: pre-wrap;
    word-wrap: break-word;
}

.row-header {
    color: #e74c3c;
    font-weight: bold;
}

.col-header {
    color: #3498db;
    font-weight: bold;
}

.cell-value {
    color: #27ae60;
}

.not-found {
    color: #95a5a6;
    font-style: italic;
    text-align: center;
    padding: 20px;
}

.footer {
    text-align: center;
    margin-top: 40px;
    color: #7f8c8d;
    font-size: 0.9em;
}
"""

    html_parts.append(elements_css)
    html_parts.append("</style>")
    html_parts.append("</head>")
    html_parts.append("<body>")

    # Header with run metadata
    html_parts.append('<div class="header">')
    html_parts.append(f"<h1>{html.escape(page_title)}</h1>")
    html_parts.append(f'<div class="run-info">Run ID: {html.escape(run_id)}</div>')

    # Determine resolver from first available snapshot
    resolver_name = "Unknown"
    for snapshot_data in snapshot_data_by_matrix.values():
        if snapshot_data.get("resolver"):
            resolver_name = snapshot_data["resolver"]
            break
    html_parts.append(f'<div class="run-info">Resolver: {html.escape(resolver_name)}</div>')
    html_parts.append(
        f'<div class="run-info">Generated: {datetime.now().strftime("%Y-%m-%d %H:%M:%S")}</div>'
    )
    html_parts.append("</div>")

    # Render each matrix in canonical order
    for matrix_name in CANONICAL_ORDER:
        html_parts.append(f'<div class="matrix-section" id="{matrix_name.lower()}">')

        if matrix_name in snapshot_data_by_matrix:
            snapshot_data = snapshot_data_by_matrix[matrix_name]

            # Matrix header
            html_parts.append(f'<h2 class="matrix-header">Matrix {matrix_name}</h2>')

            # Matrix info
            html_parts.append('<div class="matrix-info">')
            station = html.escape(str(snapshot_data.get("station", "Unknown")))
            shape = snapshot_data.get("shape", [0, 0])
            timestamp = snapshot_data.get("timestamp", "Unknown")

            html_parts.append(f"<strong>Station:</strong> {station}<br>")
            html_parts.append(f"<strong>Shape:</strong> {shape[0]}×{shape[1]}<br>")
            html_parts.append(f"<strong>Timestamp:</strong> {html.escape(str(timestamp))}")
            html_parts.append("</div>")

            # Elements grid rendering
            elements_html = _render_elements_grid(snapshot_data, matrix_name, sanitize)
            html_parts.append(elements_html)
        else:
            # Missing matrix
            html_parts.append(f'<h2 class="matrix-header">Matrix {matrix_name}</h2>')
            html_parts.append('<div class="not-found">Snapshot not found</div>')

        html_parts.append("</div>")

    # Footer
    html_parts.append('<div class="footer">')
    html_parts.append("Generated by Chirality Framework Elements Viewer")
    html_parts.append("</div>")

    html_parts.append("</body>")
    html_parts.append("</html>")

    return "\n".join(html_parts)


def _render_elements_grid(snapshot_data: Dict, matrix_name: str, sanitize: bool = False) -> str:
    """
    Render matrix data as Elements-style nested list in <pre> tags.

    Args:
        snapshot_data: Matrix snapshot data
        matrix_name: Name of the matrix
        sanitize: Whether to sanitize cell values

    Returns:
        HTML string with Elements-style grid display
    """
    if not snapshot_data.get("cells"):
        return '<div class="not-found">No cell data available</div>'

    shape = snapshot_data.get("shape", [0, 0])
    if len(shape) != 2 or shape[0] == 0 or shape[1] == 0:
        return '<div class="not-found">Invalid matrix shape</div>'

    rows, cols = shape
    row_labels = snapshot_data.get("row_labels", [])
    col_labels = snapshot_data.get("col_labels", [])

    # Reconstruct 2D grid from cells array using (i,j) coordinates
    grid = [[None for _ in range(cols)] for _ in range(rows)]

    for cell_data in snapshot_data["cells"]:
        row = cell_data.get("row")
        col = cell_data.get("col")
        if row is not None and col is not None and 0 <= row < rows and 0 <= col < cols:
            grid[row][col] = cell_data

    # Generate Elements-style display
    elements_lines = []
    elements_lines.append(f"Elements[{matrix_name}] = [")

    for i in range(rows):
        row_label = row_labels[i] if i < len(row_labels) else f"Row{i}"
        elements_lines.append(f'  <span class="row-header"># {row_label}</span>')
        elements_lines.append("  [")

        for j in range(cols):
            col_label = col_labels[j] if j < len(col_labels) else f"Col{j}"
            cell_data = grid[i][j]

            if cell_data:
                value = str(cell_data.get("value", ""))
                if sanitize:
                    value = sanitize_value(value)

                # Escape for HTML but preserve in quoted format
                escaped_value = html.escape(value)
                elements_lines.append(f'    <span class="col-header"># {col_label}</span>')
                elements_lines.append(f'    <span class="cell-value">"{escaped_value}"</span>,')
            else:
                elements_lines.append(f'    <span class="col-header"># {col_label}</span>')
                elements_lines.append('    <span class="not-found">null</span>,')

        elements_lines.append("  ],")

    elements_lines.append("]")

    # Combine into HTML
    html_content = "\n".join(elements_lines)
    return f'<div class="elements-grid"><pre>{html_content}</pre></div>'


def write_assets(html_content: str, output_dir: Path, filename: str = "index.html") -> Path:
    """
    Write HTML content to file using atomic write pattern.

    Args:
        html_content: The HTML content to write
        output_dir: Directory to write the file to
        filename: Name of the HTML file (default: index.html)

    Returns:
        Path to the written HTML file
    """
    # Create output directory idempotently
    output_dir.mkdir(parents=True, exist_ok=True)

    # Write using atomic pattern (temp file + rename)
    output_path = output_dir / filename

    with tempfile.NamedTemporaryFile(
        mode="w", suffix=".html", dir=output_dir, delete=False, encoding="utf-8"
    ) as temp_file:
        temp_file.write(html_content)
        temp_path = Path(temp_file.name)

    # Atomic move
    temp_path.replace(output_path)

    return output_path


def get_latest_run_dir(snapshots_dir: Path) -> Optional[str]:
    """
    Find the most recent run directory in snapshots.

    Args:
        snapshots_dir: Path to the snapshots directory

    Returns:
        Name of the latest run directory, or None if no runs found
    """
    if not snapshots_dir.exists() or not snapshots_dir.is_dir():
        return None

    run_dirs = []
    for item in snapshots_dir.iterdir():
        if item.is_dir():
            run_dirs.append(item.name)

    if not run_dirs:
        return None

    # Sort by directory name (which includes timestamp) and return latest
    return sorted(run_dirs)[-1]


def find_matrix_snapshots(
    run_dir: Path, include_matrices: Optional[List[str]] = None
) -> Dict[str, Path]:
    """
    Find the newest snapshot file for each matrix in a run directory.

    Args:
        run_dir: Path to the run directory containing snapshots
        include_matrices: List of matrix names to include, or None for all

    Returns:
        Dictionary mapping matrix names to their snapshot file paths
    """
    if include_matrices is None:
        include_matrices = CANONICAL_ORDER

    matrix_snapshots = {}

    for matrix_name in include_matrices:
        # Find all snapshot files for this matrix
        pattern = f"{matrix_name}-*.jsonl"
        matching_files = list(run_dir.glob(pattern))

        if matching_files:
            # Sort by filename (which includes timestamp) and take the newest
            newest_file = sorted(matching_files)[-1]
            matrix_snapshots[matrix_name] = newest_file

    return matrix_snapshots


def load_snapshots_for_run(
    run_dir: Path, include_matrices: Optional[List[str]] = None
) -> Dict[str, Dict]:
    """
    Load all matrix snapshots for a run.

    Args:
        run_dir: Path to the run directory
        include_matrices: List of matrix names to include, or None for all

    Returns:
        Dictionary mapping matrix names to their loaded snapshot data
    """
    snapshot_files = find_matrix_snapshots(run_dir, include_matrices)
    snapshot_data = {}

    for matrix_name, snapshot_path in snapshot_files.items():
        try:
            snapshot_data[matrix_name] = load_matrix_snapshot(snapshot_path)
        except (FileNotFoundError, json.JSONDecodeError):
            # Skip matrices with invalid snapshots
            continue

    return snapshot_data

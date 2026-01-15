## Matrix {{matrix_id}} Lens Construction
Matrix {{matrix_id}}
Dimensions: {{n_rows}} × {{n_cols}} 
Row labels: {{rows_json}}
Column labels: {{cols_json}}

Recall the logical progression of stations to generate reliable knowledge (semantic valley): if problem statement, then requirements, then objectives, then verification, then validation, then evaluation, then assessment, then implementation, then reflection and resolution.

We are at "{{station}}"

## Task: Generate Complete Lens Matrix

Generate interpretive lenses for every position in this {{n_rows}} × {{n_cols}} matrix.

For each cell (row_i, col_j):
1) Take the meaning of the row label and the meaning of the column label.
2) Synthesize them with the meaning of this station using:
   [ station_meaning * row_name * column_name * ]
3) Output a concise, actionable lens phrase.

### Output format
Once the final lenses for Matrix {{matrix_id}}  have been generated then return final output in a table using markdown format.
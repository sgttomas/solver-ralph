/**
 * InputsEditor Component
 *
 * Simplified editor for TypedRef inputs in Intake forms.
 * Per SR-SPEC §1.5.3, TypedRef has: kind, id, rel, meta, label
 */

import { useState } from 'react';
import { Button } from '../ui';
import styles from '../styles/pages.module.css';

export interface TypedRefInput {
  kind: string;
  id: string;
  rel: string;
  meta?: {
    content_hash?: string;
    version?: string;
    type_key?: string;
    selector?: string;
  };
  label?: string;
}

interface InputsEditorProps {
  value: TypedRefInput[];
  onChange: (value: TypedRefInput[]) => void;
}

const REF_KINDS = [
  'GovernedArtifact',
  'Candidate',
  'OracleSuite',
  'EvidenceBundle',
  'Intake',
  'ProcedureTemplate',
  'Iteration',
  'Loop',
  'Record',
  'Decision',
];

const REF_RELATIONS = [
  'about',
  'depends_on',
  'supported_by',
  'produces',
  'verifies',
  'approved_by',
  'acknowledges',
  'supersedes',
  'governed_by',
  'in_scope_of',
  'affects',
  'relates_to',
];

export function InputsEditor({
  value,
  onChange,
}: InputsEditorProps): JSX.Element {
  const [showAddForm, setShowAddForm] = useState(false);
  const [newInput, setNewInput] = useState<TypedRefInput>({
    kind: 'GovernedArtifact',
    id: '',
    rel: 'depends_on',
    label: '',
  });

  const handleAdd = () => {
    if (!newInput.id.trim()) return;
    onChange([
      ...value,
      {
        kind: newInput.kind,
        id: newInput.id.trim(),
        rel: newInput.rel,
        label: newInput.label?.trim() || undefined,
        meta: {},
      },
    ]);
    setNewInput({
      kind: 'GovernedArtifact',
      id: '',
      rel: 'depends_on',
      label: '',
    });
    setShowAddForm(false);
  };

  const handleRemove = (index: number) => {
    onChange(value.filter((_, i) => i !== index));
  };

  const handleUpdate = (index: number, field: keyof TypedRefInput, newValue: string) => {
    const updated = [...value];
    updated[index] = { ...updated[index], [field]: newValue || undefined };
    onChange(updated);
  };

  return (
    <div className={styles.formGroup}>
      <label className={styles.label}>Input References</label>

      {/* Existing inputs */}
      {value.length > 0 && (
        <div style={{
          display: 'flex',
          flexDirection: 'column',
          gap: 'var(--space3)',
          marginBottom: 'var(--space3)',
        }}>
          {value.map((input, index) => (
            <div
              key={index}
              style={{
                padding: 'var(--space3)',
                border: '1px solid var(--border)',
                borderRadius: 'var(--radiusSm)',
                position: 'relative',
              }}
            >
              <button
                type="button"
                onClick={() => handleRemove(index)}
                style={{
                  position: 'absolute',
                  top: 'var(--space2)',
                  right: 'var(--space2)',
                  background: 'none',
                  border: 'none',
                  color: 'var(--danger)',
                  cursor: 'pointer',
                  fontSize: '1rem',
                }}
                title="Remove input"
              >
                &times;
              </button>

              <div style={{
                display: 'grid',
                gridTemplateColumns: '1fr 1fr',
                gap: 'var(--space2)',
              }}>
                <div className={styles.formGroup}>
                  <label className={styles.labelSmall}>Kind</label>
                  <select
                    value={input.kind}
                    onChange={(e) => handleUpdate(index, 'kind', e.target.value)}
                    className={styles.select}
                  >
                    {REF_KINDS.map((kind) => (
                      <option key={kind} value={kind}>
                        {kind}
                      </option>
                    ))}
                  </select>
                </div>
                <div className={styles.formGroup}>
                  <label className={styles.labelSmall}>Relation</label>
                  <select
                    value={input.rel}
                    onChange={(e) => handleUpdate(index, 'rel', e.target.value)}
                    className={styles.select}
                  >
                    {REF_RELATIONS.map((rel) => (
                      <option key={rel} value={rel}>
                        {rel}
                      </option>
                    ))}
                  </select>
                </div>
                <div className={styles.formGroup} style={{ gridColumn: '1 / -1' }}>
                  <label className={styles.labelSmall}>ID</label>
                  <input
                    type="text"
                    value={input.id}
                    onChange={(e) => handleUpdate(index, 'id', e.target.value)}
                    className={styles.input}
                    placeholder="e.g., SR-CONTRACT or sha256:abc123..."
                  />
                </div>
                <div className={styles.formGroup} style={{ gridColumn: '1 / -1' }}>
                  <label className={styles.labelSmall}>Label (optional)</label>
                  <input
                    type="text"
                    value={input.label || ''}
                    onChange={(e) => handleUpdate(index, 'label', e.target.value)}
                    className={styles.input}
                    placeholder="Human-readable label"
                  />
                </div>
              </div>
            </div>
          ))}
        </div>
      )}

      {/* Add new input form */}
      {showAddForm ? (
        <div style={{
          padding: 'var(--space3)',
          border: '1px dashed var(--border)',
          borderRadius: 'var(--radiusSm)',
        }}>
          <div style={{
            display: 'grid',
            gridTemplateColumns: '1fr 1fr',
            gap: 'var(--space2)',
          }}>
            <div className={styles.formGroup}>
              <label className={styles.labelSmall}>Kind</label>
              <select
                value={newInput.kind}
                onChange={(e) => setNewInput({ ...newInput, kind: e.target.value })}
                className={styles.select}
              >
                {REF_KINDS.map((kind) => (
                  <option key={kind} value={kind}>
                    {kind}
                  </option>
                ))}
              </select>
            </div>
            <div className={styles.formGroup}>
              <label className={styles.labelSmall}>Relation</label>
              <select
                value={newInput.rel}
                onChange={(e) => setNewInput({ ...newInput, rel: e.target.value })}
                className={styles.select}
              >
                {REF_RELATIONS.map((rel) => (
                  <option key={rel} value={rel}>
                    {rel}
                  </option>
                ))}
              </select>
            </div>
            <div className={styles.formGroup} style={{ gridColumn: '1 / -1' }}>
              <label className={styles.labelSmall}>ID *</label>
              <input
                type="text"
                value={newInput.id}
                onChange={(e) => setNewInput({ ...newInput, id: e.target.value })}
                className={styles.input}
                placeholder="e.g., SR-CONTRACT or sha256:abc123..."
              />
            </div>
            <div className={styles.formGroup} style={{ gridColumn: '1 / -1' }}>
              <label className={styles.labelSmall}>Label (optional)</label>
              <input
                type="text"
                value={newInput.label || ''}
                onChange={(e) => setNewInput({ ...newInput, label: e.target.value })}
                className={styles.input}
                placeholder="Human-readable label"
              />
            </div>
          </div>
          <div style={{
            display: 'flex',
            gap: 'var(--space2)',
            marginTop: 'var(--space3)',
            justifyContent: 'flex-end',
          }}>
            <Button
              type="button"
              variant="ghost"
              onClick={() => {
                setShowAddForm(false);
                setNewInput({
                  kind: 'GovernedArtifact',
                  id: '',
                  rel: 'depends_on',
                  label: '',
                });
              }}
            >
              Cancel
            </Button>
            <Button
              type="button"
              variant="primary"
              onClick={handleAdd}
              disabled={!newInput.id.trim()}
            >
              Add Input
            </Button>
          </div>
        </div>
      ) : (
        <Button type="button" variant="secondary" onClick={() => setShowAddForm(true)}>
          + Add Input Reference
        </Button>
      )}

      {value.length === 0 && !showAddForm && (
        <p style={{
          color: 'var(--muted)',
          fontSize: '0.875rem',
          marginTop: 'var(--space2)',
        }}>
          Add content-addressed references as input context (optional).
        </p>
      )}
    </div>
  );
}

export default InputsEditor;

/**
 * DeliverablesEditor Component
 *
 * Editor for intake deliverables array.
 * Each deliverable has: name, format, path, description (optional)
 */

import { useState } from 'react';
import { Button } from '../ui';
import styles from '../styles/pages.module.css';

export interface Deliverable {
  name: string;
  format: string;
  path: string;
  description?: string;
}

interface DeliverablesEditorProps {
  value: Deliverable[];
  onChange: (value: Deliverable[]) => void;
}

const FORMAT_OPTIONS = [
  'markdown',
  'json',
  'yaml',
  'text',
  'pdf',
  'html',
  'typescript',
  'python',
  'other',
];

export function DeliverablesEditor({
  value,
  onChange,
}: DeliverablesEditorProps): JSX.Element {
  const [showAddForm, setShowAddForm] = useState(false);
  const [newDeliverable, setNewDeliverable] = useState<Deliverable>({
    name: '',
    format: 'markdown',
    path: '',
    description: '',
  });

  const handleAdd = () => {
    if (!newDeliverable.name.trim() || !newDeliverable.path.trim()) {
      return;
    }
    onChange([
      ...value,
      {
        name: newDeliverable.name.trim(),
        format: newDeliverable.format,
        path: newDeliverable.path.trim(),
        description: newDeliverable.description?.trim() || undefined,
      },
    ]);
    setNewDeliverable({
      name: '',
      format: 'markdown',
      path: '',
      description: '',
    });
    setShowAddForm(false);
  };

  const handleRemove = (index: number) => {
    onChange(value.filter((_, i) => i !== index));
  };

  const handleUpdate = (index: number, field: keyof Deliverable, newValue: string) => {
    const updated = [...value];
    updated[index] = { ...updated[index], [field]: newValue || undefined };
    onChange(updated);
  };

  return (
    <div className={styles.formGroup}>
      <label className={styles.label}>Deliverables</label>

      {/* Existing deliverables */}
      {value.length > 0 && (
        <div style={{
          display: 'flex',
          flexDirection: 'column',
          gap: 'var(--space3)',
          marginBottom: 'var(--space3)',
        }}>
          {value.map((deliverable, index) => (
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
                title="Remove deliverable"
              >
                &times;
              </button>

              <div style={{
                display: 'grid',
                gridTemplateColumns: '1fr 1fr',
                gap: 'var(--space2)',
              }}>
                <div className={styles.formGroup}>
                  <label className={styles.labelSmall}>Name</label>
                  <input
                    type="text"
                    value={deliverable.name}
                    onChange={(e) => handleUpdate(index, 'name', e.target.value)}
                    className={styles.input}
                    placeholder="e.g., Analysis Document"
                  />
                </div>
                <div className={styles.formGroup}>
                  <label className={styles.labelSmall}>Format</label>
                  <select
                    value={deliverable.format}
                    onChange={(e) => handleUpdate(index, 'format', e.target.value)}
                    className={styles.select}
                  >
                    {FORMAT_OPTIONS.map((fmt) => (
                      <option key={fmt} value={fmt}>
                        {fmt}
                      </option>
                    ))}
                  </select>
                </div>
                <div className={styles.formGroup} style={{ gridColumn: '1 / -1' }}>
                  <label className={styles.labelSmall}>Path</label>
                  <input
                    type="text"
                    value={deliverable.path}
                    onChange={(e) => handleUpdate(index, 'path', e.target.value)}
                    className={styles.input}
                    placeholder="e.g., candidate/main.md"
                  />
                </div>
                <div className={styles.formGroup} style={{ gridColumn: '1 / -1' }}>
                  <label className={styles.labelSmall}>Description (optional)</label>
                  <input
                    type="text"
                    value={deliverable.description || ''}
                    onChange={(e) => handleUpdate(index, 'description', e.target.value)}
                    className={styles.input}
                    placeholder="Brief description of the deliverable"
                  />
                </div>
              </div>
            </div>
          ))}
        </div>
      )}

      {/* Add new deliverable form */}
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
              <label className={styles.labelSmall}>Name *</label>
              <input
                type="text"
                value={newDeliverable.name}
                onChange={(e) => setNewDeliverable({ ...newDeliverable, name: e.target.value })}
                className={styles.input}
                placeholder="e.g., Analysis Document"
              />
            </div>
            <div className={styles.formGroup}>
              <label className={styles.labelSmall}>Format</label>
              <select
                value={newDeliverable.format}
                onChange={(e) => setNewDeliverable({ ...newDeliverable, format: e.target.value })}
                className={styles.select}
              >
                {FORMAT_OPTIONS.map((fmt) => (
                  <option key={fmt} value={fmt}>
                    {fmt}
                  </option>
                ))}
              </select>
            </div>
            <div className={styles.formGroup} style={{ gridColumn: '1 / -1' }}>
              <label className={styles.labelSmall}>Path *</label>
              <input
                type="text"
                value={newDeliverable.path}
                onChange={(e) => setNewDeliverable({ ...newDeliverable, path: e.target.value })}
                className={styles.input}
                placeholder="e.g., candidate/main.md"
              />
            </div>
            <div className={styles.formGroup} style={{ gridColumn: '1 / -1' }}>
              <label className={styles.labelSmall}>Description (optional)</label>
              <input
                type="text"
                value={newDeliverable.description || ''}
                onChange={(e) => setNewDeliverable({ ...newDeliverable, description: e.target.value })}
                className={styles.input}
                placeholder="Brief description of the deliverable"
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
                setNewDeliverable({
                  name: '',
                  format: 'markdown',
                  path: '',
                  description: '',
                });
              }}
            >
              Cancel
            </Button>
            <Button
              type="button"
              variant="primary"
              onClick={handleAdd}
              disabled={!newDeliverable.name.trim() || !newDeliverable.path.trim()}
            >
              Add Deliverable
            </Button>
          </div>
        </div>
      ) : (
        <Button type="button" variant="secondary" onClick={() => setShowAddForm(true)}>
          + Add Deliverable
        </Button>
      )}

      {value.length === 0 && !showAddForm && (
        <p style={{
          color: 'var(--muted)',
          fontSize: '0.875rem',
          marginTop: 'var(--space2)',
        }}>
          At least one deliverable is required.
        </p>
      )}
    </div>
  );
}

export default DeliverablesEditor;

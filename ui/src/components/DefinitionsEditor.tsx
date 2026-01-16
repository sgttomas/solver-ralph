/**
 * DefinitionsEditor Component
 *
 * Editor for key-value definitions in Intake forms.
 * Maps term names to their definitions.
 */

import { useState } from 'react';
import { Button } from '../ui';
import styles from '../styles/pages.module.css';

interface DefinitionsEditorProps {
  value: Record<string, string>;
  onChange: (value: Record<string, string>) => void;
}

export function DefinitionsEditor({
  value,
  onChange,
}: DefinitionsEditorProps): JSX.Element {
  const [newTerm, setNewTerm] = useState('');
  const [newDefinition, setNewDefinition] = useState('');

  const entries = Object.entries(value);

  const handleAdd = () => {
    const term = newTerm.trim();
    const definition = newDefinition.trim();
    if (term && definition && !(term in value)) {
      onChange({ ...value, [term]: definition });
      setNewTerm('');
      setNewDefinition('');
    }
  };

  const handleRemove = (term: string) => {
    const updated = { ...value };
    delete updated[term];
    onChange(updated);
  };

  const handleUpdateTerm = (oldTerm: string, newTermValue: string) => {
    if (newTermValue === oldTerm) return;
    const updated: Record<string, string> = {};
    for (const [key, val] of entries) {
      if (key === oldTerm) {
        updated[newTermValue] = val;
      } else {
        updated[key] = val;
      }
    }
    onChange(updated);
  };

  const handleUpdateDefinition = (term: string, newDefinitionValue: string) => {
    onChange({ ...value, [term]: newDefinitionValue });
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') {
      e.preventDefault();
      handleAdd();
    }
  };

  return (
    <div className={styles.formGroup}>
      <label className={styles.label}>Definitions</label>

      {/* Existing definitions */}
      {entries.length > 0 && (
        <div style={{
          display: 'flex',
          flexDirection: 'column',
          gap: 'var(--space2)',
          marginBottom: 'var(--space3)',
        }}>
          {entries.map(([term, definition]) => (
            <div
              key={term}
              style={{
                display: 'grid',
                gridTemplateColumns: '1fr 2fr auto',
                gap: 'var(--space2)',
                alignItems: 'start',
              }}
            >
              <input
                type="text"
                value={term}
                onChange={(e) => handleUpdateTerm(term, e.target.value)}
                className={styles.input}
                placeholder="Term"
                style={{ fontWeight: 500 }}
              />
              <input
                type="text"
                value={definition}
                onChange={(e) => handleUpdateDefinition(term, e.target.value)}
                className={styles.input}
                placeholder="Definition"
              />
              <button
                type="button"
                onClick={() => handleRemove(term)}
                style={{
                  background: 'none',
                  border: 'none',
                  color: 'var(--danger)',
                  cursor: 'pointer',
                  padding: 'var(--space2)',
                  fontSize: '1rem',
                }}
                title="Remove definition"
              >
                &times;
              </button>
            </div>
          ))}
        </div>
      )}

      {/* Add new definition */}
      <div style={{
        display: 'grid',
        gridTemplateColumns: '1fr 2fr auto',
        gap: 'var(--space2)',
        alignItems: 'end',
      }}>
        <div className={styles.formGroup} style={{ marginBottom: 0 }}>
          <label className={styles.labelSmall}>Term</label>
          <input
            type="text"
            value={newTerm}
            onChange={(e) => setNewTerm(e.target.value)}
            onKeyDown={handleKeyDown}
            className={styles.input}
            placeholder="e.g., rate_limit"
          />
        </div>
        <div className={styles.formGroup} style={{ marginBottom: 0 }}>
          <label className={styles.labelSmall}>Definition</label>
          <input
            type="text"
            value={newDefinition}
            onChange={(e) => setNewDefinition(e.target.value)}
            onKeyDown={handleKeyDown}
            className={styles.input}
            placeholder="e.g., Maximum requests per time window"
          />
        </div>
        <Button
          type="button"
          variant="secondary"
          onClick={handleAdd}
          disabled={!newTerm.trim() || !newDefinition.trim() || newTerm.trim() in value}
        >
          Add
        </Button>
      </div>

      {entries.length === 0 && (
        <p style={{
          color: 'var(--muted)',
          fontSize: '0.875rem',
          marginTop: 'var(--space2)',
        }}>
          Define terms used in this intake (optional).
        </p>
      )}
    </div>
  );
}

export default DefinitionsEditor;

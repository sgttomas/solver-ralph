/**
 * ArrayStringEditor Component
 *
 * Reusable component for editing arrays of strings.
 * Used for constraints, unknowns, completion_criteria in Intake forms.
 */

import { useState } from 'react';
import { Button } from '../ui';
import styles from '../styles/pages.module.css';

interface ArrayStringEditorProps {
  value: string[];
  onChange: (value: string[]) => void;
  placeholder?: string;
  label?: string;
  addButtonLabel?: string;
}

export function ArrayStringEditor({
  value,
  onChange,
  placeholder = 'Enter item...',
  label,
  addButtonLabel = 'Add Item',
}: ArrayStringEditorProps): JSX.Element {
  const [newItem, setNewItem] = useState('');

  const handleAdd = () => {
    const trimmed = newItem.trim();
    if (trimmed && !value.includes(trimmed)) {
      onChange([...value, trimmed]);
      setNewItem('');
    }
  };

  const handleRemove = (index: number) => {
    onChange(value.filter((_, i) => i !== index));
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') {
      e.preventDefault();
      handleAdd();
    }
  };

  const handleUpdate = (index: number, newValue: string) => {
    const updated = [...value];
    updated[index] = newValue;
    onChange(updated);
  };

  return (
    <div className={styles.formGroup}>
      {label && <label className={styles.label}>{label}</label>}

      {/* Existing items */}
      {value.length > 0 && (
        <ul style={{
          listStyle: 'none',
          margin: '0 0 var(--space2) 0',
          padding: 0,
          display: 'flex',
          flexDirection: 'column',
          gap: 'var(--space2)',
        }}>
          {value.map((item, index) => (
            <li
              key={index}
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: 'var(--space2)',
              }}
            >
              <input
                type="text"
                value={item}
                onChange={(e) => handleUpdate(index, e.target.value)}
                className={styles.input}
                style={{ flex: 1 }}
              />
              <button
                type="button"
                onClick={() => handleRemove(index)}
                style={{
                  background: 'none',
                  border: 'none',
                  color: 'var(--danger)',
                  cursor: 'pointer',
                  padding: 'var(--space1)',
                  fontSize: '1rem',
                }}
                title="Remove"
              >
                &times;
              </button>
            </li>
          ))}
        </ul>
      )}

      {/* Add new item */}
      <div style={{ display: 'flex', gap: 'var(--space2)' }}>
        <input
          type="text"
          value={newItem}
          onChange={(e) => setNewItem(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder={placeholder}
          className={styles.input}
          style={{ flex: 1 }}
        />
        <Button
          type="button"
          variant="secondary"
          onClick={handleAdd}
          disabled={!newItem.trim()}
        >
          {addButtonLabel}
        </Button>
      </div>
    </div>
  );
}

export default ArrayStringEditor;

/**
 * LoopEditModal Component
 *
 * Modal form for editing existing loops.
 * Only certain fields are editable based on loop state:
 * - CREATED/PAUSED: Goal, Work Unit, Budgets (increase only)
 * - ACTIVE/CLOSED: View only
 */

import { useState, useEffect } from 'react';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Button, Pill, getStatusTone } from '../ui';
import type { LoopState, LoopBudgets, TypedRef } from '../hooks/useLoops';
import styles from './LoopModal.module.css';

// Minimal loop interface for editing - only includes fields we need
interface EditableLoop {
  id: string;
  goal: string;
  work_unit: string | null;
  state: LoopState;
  created_at: string;
  directive_ref: TypedRef;
  budgets: LoopBudgets;
  iteration_count: number;
  oracle_run_count: number;
}

interface LoopEditModalProps {
  isOpen: boolean;
  loop: EditableLoop | null;
  onClose: () => void;
  onUpdated: () => void;
}

export function LoopEditModal({
  isOpen,
  loop,
  onClose,
  onUpdated,
}: LoopEditModalProps): JSX.Element | null {
  const auth = useAuth();

  // Form state
  const [goal, setGoal] = useState('');
  const [workUnit, setWorkUnit] = useState('');
  const [maxIterations, setMaxIterations] = useState(5);
  const [maxOracleRuns, setMaxOracleRuns] = useState(25);
  const [maxWallclockHours, setMaxWallclockHours] = useState(16);

  // Submission state
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Initialize form with loop data
  useEffect(() => {
    if (loop) {
      setGoal(loop.goal);
      setWorkUnit(loop.work_unit || '');
      setMaxIterations(loop.budgets.max_iterations);
      setMaxOracleRuns(loop.budgets.max_oracle_runs);
      setMaxWallclockHours(loop.budgets.max_wallclock_hours);
    }
  }, [loop]);

  const isEditable = (state: LoopState) => state === 'CREATED' || state === 'PAUSED';
  const canEdit = loop ? isEditable(loop.state) : false;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!loop || !canEdit) return;

    if (!goal.trim()) {
      setError('Goal is required');
      return;
    }

    // Validate budget increases only
    if (maxIterations < loop.budgets.max_iterations) {
      setError('Max iterations can only be increased, not decreased');
      return;
    }
    if (maxOracleRuns < loop.budgets.max_oracle_runs) {
      setError('Max oracle runs can only be increased, not decreased');
      return;
    }
    if (maxWallclockHours < loop.budgets.max_wallclock_hours) {
      setError('Max wallclock hours can only be increased, not decreased');
      return;
    }

    if (!auth.user?.access_token) {
      setError('Not authenticated');
      return;
    }

    setSubmitting(true);
    setError(null);

    try {
      const payload = {
        goal: goal.trim(),
        work_unit: workUnit.trim() || null,
        budgets: {
          max_iterations: maxIterations,
          max_oracle_runs: maxOracleRuns,
          max_wallclock_hours: maxWallclockHours,
        },
      };

      const res = await fetch(`${config.apiUrl}/api/v1/loops/${loop.id}`, {
        method: 'PATCH',
        headers: {
          Authorization: `Bearer ${auth.user.access_token}`,
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(payload),
      });

      if (!res.ok) {
        const errorData = await res.json().catch(() => ({}));
        throw new Error(errorData.message || `Failed to update loop: HTTP ${res.status}`);
      }

      onUpdated();
      onClose();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to update loop');
    } finally {
      setSubmitting(false);
    }
  };

  const handleClose = () => {
    setError(null);
    onClose();
  };

  if (!isOpen || !loop) return null;

  return (
    <div className={styles.overlay} onClick={handleClose}>
      <div className={styles.modal} onClick={e => e.stopPropagation()}>
        <div className={styles.header}>
          <h2 className={styles.title}>
            {canEdit ? 'Edit Loop' : 'Loop Details'}
          </h2>
          <button className={styles.closeButton} onClick={handleClose}>
            &times;
          </button>
        </div>

        <form onSubmit={handleSubmit} className={styles.form}>
          {/* Read-only info section */}
          <div className={styles.formGroup}>
            <div className={styles.infoRow}>
              <span className={styles.infoLabel}>Loop ID</span>
              <span className={styles.infoValue}>{loop.id}</span>
            </div>
            <div className={styles.infoRow}>
              <span className={styles.infoLabel}>State</span>
              <Pill tone={getStatusTone(loop.state)}>{loop.state}</Pill>
            </div>
            <div className={styles.infoRow}>
              <span className={styles.infoLabel}>Created</span>
              <span className={styles.infoValue}>
                {new Date(loop.created_at).toLocaleString()}
              </span>
            </div>
            {loop.directive_ref && (
              <div className={styles.infoRow}>
                <span className={styles.infoLabel}>Directive</span>
                <span className={styles.infoValue}>
                  {loop.directive_ref.type_key}:{loop.directive_ref.id}
                </span>
              </div>
            )}
          </div>

          {!canEdit && (
            <div className={styles.error} style={{
              background: 'color-mix(in srgb, var(--warning) 10%, white)',
              borderColor: 'color-mix(in srgb, var(--warning) 25%, transparent)',
              color: 'var(--warning)'
            }}>
              This loop is {loop.state.toLowerCase()} and cannot be edited.
              {loop.state === 'ACTIVE' && ' Pause the loop to make changes.'}
            </div>
          )}

          {/* Goal */}
          <div className={styles.formGroup}>
            <label className={styles.label}>
              Goal <span className={styles.required}>*</span>
            </label>
            <textarea
              className={`${styles.textarea} ${!canEdit ? styles.readOnly : ''}`}
              value={goal}
              onChange={e => setGoal(e.target.value)}
              disabled={!canEdit}
              rows={3}
            />
          </div>

          {/* Work Unit */}
          <div className={styles.formGroup}>
            <label className={styles.label}>Work Unit</label>
            <input
              type="text"
              className={`${styles.input} ${!canEdit ? styles.readOnly : ''}`}
              value={workUnit}
              onChange={e => setWorkUnit(e.target.value)}
              disabled={!canEdit}
            />
          </div>

          {/* Budgets */}
          <div className={styles.sectionLabel}>
            Budgets {canEdit && <span className={styles.hint}>(can only increase)</span>}
          </div>
          <div className={styles.formRow}>
            <div className={styles.formGroup}>
              <label className={styles.label}>Max Iterations</label>
              <input
                type="number"
                className={`${styles.input} ${!canEdit ? styles.readOnly : ''}`}
                value={maxIterations}
                onChange={e => setMaxIterations(parseInt(e.target.value) || maxIterations)}
                min={loop.budgets.max_iterations}
                max={100}
                disabled={!canEdit}
              />
              <span className={styles.hint}>
                Current: {loop.iteration_count} used
              </span>
            </div>
            <div className={styles.formGroup}>
              <label className={styles.label}>Max Oracle Runs</label>
              <input
                type="number"
                className={`${styles.input} ${!canEdit ? styles.readOnly : ''}`}
                value={maxOracleRuns}
                onChange={e => setMaxOracleRuns(parseInt(e.target.value) || maxOracleRuns)}
                min={loop.budgets.max_oracle_runs}
                max={500}
                disabled={!canEdit}
              />
              <span className={styles.hint}>
                Current: {loop.oracle_run_count} used
              </span>
            </div>
            <div className={styles.formGroup}>
              <label className={styles.label}>Max Hours</label>
              <input
                type="number"
                className={`${styles.input} ${!canEdit ? styles.readOnly : ''}`}
                value={maxWallclockHours}
                onChange={e => setMaxWallclockHours(parseInt(e.target.value) || maxWallclockHours)}
                min={loop.budgets.max_wallclock_hours}
                max={720}
                disabled={!canEdit}
              />
            </div>
          </div>

          {/* Error */}
          {error && <div className={styles.error}>{error}</div>}

          {/* Actions */}
          <div className={styles.actions}>
            <Button variant="ghost" type="button" onClick={handleClose}>
              {canEdit ? 'Cancel' : 'Close'}
            </Button>
            {canEdit && (
              <Button variant="primary" type="submit" disabled={submitting}>
                {submitting ? 'Saving...' : 'Save Changes'}
              </Button>
            )}
          </div>
        </form>
      </div>
    </div>
  );
}

export default LoopEditModal;

/**
 * LoopCreateModal Component
 *
 * Modal form for creating new loops with full Work Surface binding.
 * Includes Goal, Work Unit, Template, Stage, Oracle Suite, and Budgets.
 */

import { useState, useEffect } from 'react';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Button } from '../ui';
import styles from './LoopModal.module.css';

interface Template {
  id: string;
  name: string;
  stages: { stage_id: string; stage_name: string }[];
}

interface OracleSuite {
  id: string;
  name: string;
}

interface CreateLoopData {
  goal: string;
  work_unit: string;
  template_id: string;
  stage_id: string;
  oracle_suite_id: string;
  budgets: {
    max_iterations: number;
    max_oracle_runs: number;
    max_wallclock_hours: number;
  };
}

interface LoopCreateModalProps {
  isOpen: boolean;
  onClose: () => void;
  onCreated: (loopId: string) => void;
}

export function LoopCreateModal({
  isOpen,
  onClose,
  onCreated,
}: LoopCreateModalProps): JSX.Element | null {
  const auth = useAuth();

  // Form state
  const [goal, setGoal] = useState('');
  const [workUnit, setWorkUnit] = useState('');
  const [templateId, setTemplateId] = useState('');
  const [stageId, setStageId] = useState('');
  const [oracleSuiteId, setOracleSuiteId] = useState('');
  const [maxIterations, setMaxIterations] = useState(5);
  const [maxOracleRuns, setMaxOracleRuns] = useState(25);
  const [maxWallclockHours, setMaxWallclockHours] = useState(16);

  // Data loading state
  const [templates, setTemplates] = useState<Template[]>([]);
  const [suites, setSuites] = useState<OracleSuite[]>([]);
  const [loadingTemplates, setLoadingTemplates] = useState(false);
  const [loadingSuites, setLoadingSuites] = useState(false);

  // Submission state
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [activateAfterCreate, setActivateAfterCreate] = useState(false);

  // Load templates
  useEffect(() => {
    if (!isOpen || !auth.user?.access_token) return;

    setLoadingTemplates(true);
    fetch(`${config.apiUrl}/api/v1/templates`, {
      headers: { Authorization: `Bearer ${auth.user.access_token}` },
    })
      .then(res => {
        if (res.status === 404) return { templates: [] };
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .then(data => {
        setTemplates(data.templates || []);
      })
      .catch(() => {
        // Silently fail - templates may not be available yet
        setTemplates([]);
      })
      .finally(() => setLoadingTemplates(false));
  }, [isOpen, auth.user?.access_token]);

  // Load oracle suites
  useEffect(() => {
    if (!isOpen || !auth.user?.access_token) return;

    setLoadingSuites(true);
    fetch(`${config.apiUrl}/api/v1/oracle-suites`, {
      headers: { Authorization: `Bearer ${auth.user.access_token}` },
    })
      .then(res => {
        if (res.status === 404) return { suites: [] };
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .then(data => {
        setSuites(data.suites || []);
      })
      .catch(() => {
        setSuites([]);
      })
      .finally(() => setLoadingSuites(false));
  }, [isOpen, auth.user?.access_token]);

  // Get stages for selected template
  const selectedTemplate = templates.find(t => t.id === templateId);
  const availableStages = selectedTemplate?.stages || [];

  // Reset stage when template changes
  useEffect(() => {
    if (availableStages.length > 0 && !availableStages.find(s => s.stage_id === stageId)) {
      setStageId(availableStages[0].stage_id);
    }
  }, [templateId, availableStages, stageId]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!goal.trim()) {
      setError('Goal is required');
      return;
    }

    if (!auth.user?.access_token) {
      setError('Not authenticated');
      return;
    }

    setSubmitting(true);
    setError(null);

    try {
      const payload: CreateLoopData = {
        goal: goal.trim(),
        work_unit: workUnit.trim() || 'general',
        template_id: templateId || 'GENERIC-KNOWLEDGE-WORK',
        stage_id: stageId || 'stage:FRAME',
        oracle_suite_id: oracleSuiteId || 'SR-SUITE-GOV',
        budgets: {
          max_iterations: maxIterations,
          max_oracle_runs: maxOracleRuns,
          max_wallclock_hours: maxWallclockHours,
        },
      };

      const res = await fetch(`${config.apiUrl}/api/v1/loops`, {
        method: 'POST',
        headers: {
          Authorization: `Bearer ${auth.user.access_token}`,
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(payload),
      });

      if (!res.ok) {
        const errorData = await res.json().catch(() => ({}));
        throw new Error(errorData.message || `Failed to create loop: HTTP ${res.status}`);
      }

      const data = await res.json();
      const loopId = data.loop_id || data.id;

      // Optionally activate immediately
      if (activateAfterCreate && loopId) {
        await fetch(`${config.apiUrl}/api/v1/loops/${loopId}/activate`, {
          method: 'POST',
          headers: {
            Authorization: `Bearer ${auth.user.access_token}`,
            'Content-Type': 'application/json',
          },
        });
      }

      // Reset form
      resetForm();
      onCreated(loopId);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create loop');
    } finally {
      setSubmitting(false);
    }
  };

  const resetForm = () => {
    setGoal('');
    setWorkUnit('');
    setTemplateId('');
    setStageId('');
    setOracleSuiteId('');
    setMaxIterations(5);
    setMaxOracleRuns(25);
    setMaxWallclockHours(16);
    setActivateAfterCreate(false);
    setError(null);
  };

  const handleClose = () => {
    resetForm();
    onClose();
  };

  if (!isOpen) return null;

  return (
    <div className={styles.overlay} onClick={handleClose}>
      <div className={styles.modal} onClick={e => e.stopPropagation()}>
        <div className={styles.header}>
          <h2 className={styles.title}>Create New Loop</h2>
          <button className={styles.closeButton} onClick={handleClose}>
            &times;
          </button>
        </div>

        <form onSubmit={handleSubmit} className={styles.form}>
          {/* Goal */}
          <div className={styles.formGroup}>
            <label className={styles.label}>
              Goal <span className={styles.required}>*</span>
            </label>
            <textarea
              className={styles.textarea}
              value={goal}
              onChange={e => setGoal(e.target.value)}
              placeholder="Describe the bounded objective for this loop..."
              rows={3}
            />
          </div>

          {/* Work Unit */}
          <div className={styles.formGroup}>
            <label className={styles.label}>Work Unit</label>
            <input
              type="text"
              className={styles.input}
              value={workUnit}
              onChange={e => setWorkUnit(e.target.value)}
              placeholder="e.g., research_memo, decision_record"
            />
            <span className={styles.hint}>
              Selects default gating policy. Leave blank for &quot;general&quot;.
            </span>
          </div>

          {/* Procedure Template */}
          <div className={styles.formGroup}>
            <label className={styles.label}>Procedure Template</label>
            <select
              className={styles.select}
              value={templateId}
              onChange={e => setTemplateId(e.target.value)}
              disabled={loadingTemplates}
            >
              <option value="">
                {loadingTemplates ? 'Loading...' : 'Select template (optional)'}
              </option>
              {templates.map(t => (
                <option key={t.id} value={t.id}>
                  {t.name || t.id}
                </option>
              ))}
            </select>
            <div className={styles.quickSelect}>
              <button
                type="button"
                className={styles.quickButton}
                onClick={() => setTemplateId('PROBLEM-STATEMENT-INGESTION')}
              >
                Problem Ingestion
              </button>
              <button
                type="button"
                className={styles.quickButton}
                onClick={() => setTemplateId('GENERIC-KNOWLEDGE-WORK')}
              >
                Knowledge Work
              </button>
            </div>
          </div>

          {/* Stage */}
          {availableStages.length > 0 && (
            <div className={styles.formGroup}>
              <label className={styles.label}>Initial Stage</label>
              <select
                className={styles.select}
                value={stageId}
                onChange={e => setStageId(e.target.value)}
              >
                {availableStages.map(s => (
                  <option key={s.stage_id} value={s.stage_id}>
                    {s.stage_name} ({s.stage_id})
                  </option>
                ))}
              </select>
            </div>
          )}

          {/* Oracle Suite */}
          <div className={styles.formGroup}>
            <label className={styles.label}>Oracle Suite</label>
            <select
              className={styles.select}
              value={oracleSuiteId}
              onChange={e => setOracleSuiteId(e.target.value)}
              disabled={loadingSuites}
            >
              <option value="">
                {loadingSuites ? 'Loading...' : 'Select suite (optional)'}
              </option>
              {suites.map(s => (
                <option key={s.id} value={s.id}>
                  {s.name || s.id}
                </option>
              ))}
            </select>
          </div>

          {/* Budgets */}
          <div className={styles.sectionLabel}>Budgets</div>
          <div className={styles.formRow}>
            <div className={styles.formGroup}>
              <label className={styles.label}>Max Iterations</label>
              <input
                type="number"
                className={styles.input}
                value={maxIterations}
                onChange={e => setMaxIterations(parseInt(e.target.value) || 5)}
                min={1}
                max={100}
              />
            </div>
            <div className={styles.formGroup}>
              <label className={styles.label}>Max Oracle Runs</label>
              <input
                type="number"
                className={styles.input}
                value={maxOracleRuns}
                onChange={e => setMaxOracleRuns(parseInt(e.target.value) || 25)}
                min={1}
                max={500}
              />
            </div>
            <div className={styles.formGroup}>
              <label className={styles.label}>Max Hours</label>
              <input
                type="number"
                className={styles.input}
                value={maxWallclockHours}
                onChange={e => setMaxWallclockHours(parseInt(e.target.value) || 16)}
                min={1}
                max={720}
              />
            </div>
          </div>

          {/* Activate after create */}
          <div className={styles.checkboxGroup}>
            <label className={styles.checkboxLabel}>
              <input
                type="checkbox"
                checked={activateAfterCreate}
                onChange={e => setActivateAfterCreate(e.target.checked)}
              />
              Activate loop immediately after creation
            </label>
          </div>

          {/* Error */}
          {error && <div className={styles.error}>{error}</div>}

          {/* Actions */}
          <div className={styles.actions}>
            <Button variant="ghost" type="button" onClick={handleClose}>
              Cancel
            </Button>
            <Button variant="primary" type="submit" disabled={submitting}>
              {submitting ? 'Creating...' : activateAfterCreate ? 'Create & Activate' : 'Create Loop'}
            </Button>
          </div>
        </form>
      </div>
    </div>
  );
}

export default LoopCreateModal;

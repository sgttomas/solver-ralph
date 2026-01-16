/**
 * StageCompletionForm Component
 *
 * Form for completing a stage with evidence and gate result.
 * Per SR-PLAN-V5 §3.2-3.7: Handles stage completion workflow.
 */

import { useState, useEffect } from 'react';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Button } from '../ui';
import { EvidenceBundleSelector } from './EvidenceBundleSelector';
import styles from '../styles/pages.module.css';

interface OracleSuiteBinding {
  suite_id: string;
  suite_hash: string;
}

interface OracleResultEntry {
  oracle_id: string;
  status: string;
  evidence_ref: string;
}

type GateResultStatus = 'PASS' | 'PASS_WITH_WAIVERS' | 'FAIL' | null;

interface StageCompletionFormProps {
  workSurfaceId: string;
  stageId: string;
  stageName: string;
  oracleSuites: OracleSuiteBinding[];
  onComplete: () => void;
  onCancel: () => void;
}

interface StageCompletionResponse {
  work_surface_id: string;
  completed_stage_id: string;
  next_stage_id: string | null;
  is_terminal: boolean;
  work_surface_status: string;
}

export function StageCompletionForm({
  workSurfaceId,
  stageId,
  stageName,
  oracleSuites,
  onComplete,
  onCancel,
}: StageCompletionFormProps): JSX.Element {
  const auth = useAuth();

  // Form state
  const [evidenceBundleRef, setEvidenceBundleRef] = useState('');
  const [gateResultStatus, setGateResultStatus] = useState<GateResultStatus>(null);
  const [oracleResults, setOracleResults] = useState<OracleResultEntry[]>([]);
  const [waiverRefs, setWaiverRefs] = useState('');

  // UI state
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [successMessage, setSuccessMessage] = useState<string | null>(null);

  // Initialize oracle results from suites
  useEffect(() => {
    const initialResults = oracleSuites.map((suite) => ({
      oracle_id: suite.suite_id,
      status: '',
      evidence_ref: '',
    }));
    setOracleResults(initialResults);
  }, [oracleSuites]);

  const updateOracleResult = (index: number, field: keyof OracleResultEntry, value: string) => {
    setOracleResults((prev) => {
      const updated = [...prev];
      updated[index] = { ...updated[index], [field]: value };
      return updated;
    });
  };

  const addOracleResult = () => {
    setOracleResults((prev) => [...prev, { oracle_id: '', status: '', evidence_ref: '' }]);
  };

  const removeOracleResult = (index: number) => {
    setOracleResults((prev) => prev.filter((_, i) => i !== index));
  };

  // Validation
  const validate = (): string | null => {
    if (!evidenceBundleRef.trim()) {
      return 'Evidence bundle reference is required';
    }
    if (!gateResultStatus) {
      return 'Gate result status is required';
    }
    if (gateResultStatus === 'PASS_WITH_WAIVERS' && !waiverRefs.trim()) {
      return 'At least one waiver reference is required for PASS_WITH_WAIVERS';
    }
    return null;
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    const validationError = validate();
    if (validationError) {
      setError(validationError);
      return;
    }

    if (!auth.user?.access_token) {
      setError('Not authenticated');
      return;
    }

    setIsSubmitting(true);
    setError(null);
    setSuccessMessage(null);

    try {
      const requestBody = {
        evidence_bundle_ref: evidenceBundleRef.trim(),
        gate_result: {
          status: gateResultStatus,
          oracle_results: oracleResults
            .filter((r) => r.oracle_id.trim())
            .map((r) => ({
              oracle_id: r.oracle_id.trim(),
              status: r.status.trim() || 'PASS',
              evidence_ref: r.evidence_ref.trim() || undefined,
            })),
          waiver_refs: waiverRefs
            .split(',')
            .map((r) => r.trim())
            .filter((r) => r.length > 0),
        },
      };

      const res = await fetch(
        `${config.apiUrl}/api/v1/work-surfaces/${workSurfaceId}/stages/${stageId}/complete`,
        {
          method: 'POST',
          headers: {
            Authorization: `Bearer ${auth.user.access_token}`,
            'Content-Type': 'application/json',
          },
          body: JSON.stringify(requestBody),
        }
      );

      if (!res.ok) {
        const errorData = await res.json().catch(() => ({}));
        const message = errorData.message || `HTTP ${res.status}`;

        // Handle specific error codes per SR-PLAN-V5 §3.7
        if (res.status === 400) {
          throw new Error(`Stage ID mismatch: ${message}. Try refreshing the page.`);
        }
        if (res.status === 409) {
          throw new Error(`Invalid state transition: ${message}. Try refreshing the page.`);
        }
        if (res.status === 412) {
          throw new Error(`Approval required: ${message}`);
        }
        throw new Error(message);
      }

      const result: StageCompletionResponse = await res.json();

      // Show success message
      if (result.is_terminal) {
        setSuccessMessage(`Stage "${stageName}" completed. Work Surface completed!`);
      } else if (result.next_stage_id) {
        const nextStageName = result.next_stage_id.replace(/^stage:/, '').replace(/_/g, ' ');
        setSuccessMessage(`Stage "${stageName}" completed. Advancing to: ${nextStageName}`);
      } else {
        setSuccessMessage(`Stage "${stageName}" completed.`);
      }

      // Brief delay to show success message, then trigger refresh
      setTimeout(() => {
        onComplete();
      }, 1500);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to complete stage');
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div
      style={{
        padding: 'var(--space4)',
        backgroundColor: 'var(--bg)',
        borderRadius: 'var(--radius)',
        border: '1px solid var(--border)',
        marginTop: 'var(--space3)',
      }}
    >
      <h4 style={{ margin: '0 0 var(--space4) 0', fontSize: '1rem' }}>
        Complete Stage: {stageName}
      </h4>

      {successMessage && <div className={styles.success}>{successMessage}</div>}

      {error && <div className={styles.error}>{error}</div>}

      <form onSubmit={handleSubmit} className={styles.form}>
        {/* Evidence Bundle Selector */}
        <EvidenceBundleSelector
          value={evidenceBundleRef}
          onChange={setEvidenceBundleRef}
          disabled={isSubmitting}
        />

        {/* Gate Result Status */}
        <div className={styles.formGroup}>
          <label className={styles.label}>Gate Result *</label>
          <div style={{ display: 'flex', gap: 'var(--space4)', flexWrap: 'wrap' }}>
            {(['PASS', 'PASS_WITH_WAIVERS', 'FAIL'] as const).map((status) => (
              <label
                key={status}
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: 'var(--space1)',
                  cursor: isSubmitting ? 'default' : 'pointer',
                }}
              >
                <input
                  type="radio"
                  name="gateResultStatus"
                  value={status}
                  checked={gateResultStatus === status}
                  onChange={() => setGateResultStatus(status)}
                  disabled={isSubmitting}
                />
                <span style={{ fontSize: '0.875rem' }}>
                  {status === 'PASS_WITH_WAIVERS' ? 'Pass with Waivers' : status.charAt(0) + status.slice(1).toLowerCase()}
                </span>
              </label>
            ))}
          </div>
        </div>

        {/* FAIL Warning */}
        {gateResultStatus === 'FAIL' && (
          <div className={styles.note}>
            Stage will not advance with FAIL status. Evidence will be recorded for audit purposes.
            Consider recording a waiver if appropriate.
          </div>
        )}

        {/* Waiver Refs (required for PASS_WITH_WAIVERS) */}
        {gateResultStatus === 'PASS_WITH_WAIVERS' && (
          <div className={styles.formGroup}>
            <label className={styles.label}>Waiver References *</label>
            <input
              type="text"
              className={styles.input}
              placeholder="ex:waiver-123, ex:waiver-456"
              value={waiverRefs}
              onChange={(e) => setWaiverRefs(e.target.value)}
              disabled={isSubmitting}
            />
            <span style={{ fontSize: '0.75rem', color: 'var(--muted)' }}>
              Comma-separated list of waiver references covering failed oracles
            </span>
          </div>
        )}

        {/* Oracle Results */}
        <div className={styles.formGroup}>
          <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
            <label className={styles.label}>Oracle Results</label>
            <button
              type="button"
              className={styles.actionLink}
              onClick={addOracleResult}
              disabled={isSubmitting}
            >
              + Add
            </button>
          </div>

          {oracleResults.length === 0 ? (
            <span style={{ fontSize: '0.875rem', color: 'var(--muted)' }}>
              No oracle suites configured for this stage
            </span>
          ) : (
            <div style={{ display: 'flex', flexDirection: 'column', gap: 'var(--space2)' }}>
              {oracleResults.map((result, index) => (
                <div
                  key={index}
                  style={{
                    display: 'grid',
                    gridTemplateColumns: '1fr 100px 1fr auto',
                    gap: 'var(--space2)',
                    alignItems: 'center',
                    padding: 'var(--space2)',
                    backgroundColor: 'white',
                    borderRadius: 'var(--radiusSm)',
                    border: '1px solid var(--border)',
                  }}
                >
                  <input
                    type="text"
                    className={styles.input}
                    placeholder="Oracle ID"
                    value={result.oracle_id}
                    onChange={(e) => updateOracleResult(index, 'oracle_id', e.target.value)}
                    disabled={isSubmitting}
                    style={{ margin: 0, padding: 'var(--space1) var(--space2)', fontSize: '0.75rem' }}
                  />
                  <select
                    className={styles.select}
                    value={result.status}
                    onChange={(e) => updateOracleResult(index, 'status', e.target.value)}
                    disabled={isSubmitting}
                    style={{ margin: 0, padding: 'var(--space1)', fontSize: '0.75rem' }}
                  >
                    <option value="">Status</option>
                    <option value="PASS">PASS</option>
                    <option value="FAIL">FAIL</option>
                    <option value="SKIP">SKIP</option>
                  </select>
                  <input
                    type="text"
                    className={styles.input}
                    placeholder="Evidence ref (optional)"
                    value={result.evidence_ref}
                    onChange={(e) => updateOracleResult(index, 'evidence_ref', e.target.value)}
                    disabled={isSubmitting}
                    style={{ margin: 0, padding: 'var(--space1) var(--space2)', fontSize: '0.75rem' }}
                  />
                  <button
                    type="button"
                    className={styles.actionLink}
                    onClick={() => removeOracleResult(index)}
                    disabled={isSubmitting}
                    style={{ padding: 'var(--space1)' }}
                  >
                    Remove
                  </button>
                </div>
              ))}
            </div>
          )}
          <span style={{ fontSize: '0.75rem', color: 'var(--muted)', marginTop: 'var(--space1)' }}>
            Pre-populated from current oracle suites. Edit status and evidence as needed.
          </span>
        </div>

        {/* Buttons */}
        <div className={styles.buttonRow}>
          <Button variant="ghost" onClick={onCancel} disabled={isSubmitting}>
            Cancel
          </Button>
          <Button variant="primary" disabled={isSubmitting}>
            {isSubmitting ? 'Completing...' : 'Complete Stage'}
          </Button>
        </div>
      </form>
    </div>
  );
}

export default StageCompletionForm;

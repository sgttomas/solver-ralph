/**
 * StageApprovalForm Component
 *
 * Form for recording stage-gate approvals.
 * Per SR-PLAN-V5 Phase 5c: Approval-gated stages require human approval
 * before stage completion is allowed.
 */

import { useState } from 'react';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Button } from '../ui';
import styles from '../styles/pages.module.css';

type ApprovalDecision = 'APPROVED' | 'REJECTED' | 'DEFERRED';

interface StageApprovalFormProps {
  portalId: string;
  workSurfaceId: string;
  onComplete: () => void;
  onCancel: () => void;
}

export function StageApprovalForm({
  portalId,
  workSurfaceId,
  onComplete,
  onCancel,
}: StageApprovalFormProps): JSX.Element {
  const auth = useAuth();

  // Form state
  const [decision, setDecision] = useState<ApprovalDecision>('APPROVED');
  const [rationale, setRationale] = useState('');
  const [evidenceRefs, setEvidenceRefs] = useState('');
  const [exceptionsAcknowledged, setExceptionsAcknowledged] = useState('');

  // UI state
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [successMessage, setSuccessMessage] = useState<string | null>(null);

  // Validation
  const validate = (): string | null => {
    if (!rationale.trim()) {
      return 'Rationale is required for audit purposes';
    }
    if (decision === 'REJECTED' && !rationale.trim()) {
      return 'Rationale is required when rejecting';
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
        portal_id: portalId,
        decision,
        subject_refs: [
          {
            kind: 'WorkSurface',
            id: workSurfaceId,
            rel: 'approves',
            meta: {},
          },
        ],
        evidence_refs: evidenceRefs
          .split(',')
          .map((r) => r.trim())
          .filter((r) => r.length > 0),
        exceptions_acknowledged: exceptionsAcknowledged
          .split(',')
          .map((r) => r.trim())
          .filter((r) => r.length > 0),
        rationale: rationale.trim(),
      };

      const res = await fetch(`${config.apiUrl}/api/v1/approvals`, {
        method: 'POST',
        headers: {
          Authorization: `Bearer ${auth.user.access_token}`,
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(requestBody),
      });

      if (!res.ok) {
        const errorData = await res.json().catch(() => ({}));
        const message = errorData.error || `HTTP ${res.status}`;
        throw new Error(message);
      }

      setSuccessMessage(`Approval recorded: ${decision}`);

      // Brief delay to show success message, then trigger refresh
      setTimeout(() => {
        onComplete();
      }, 1500);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to record approval');
    } finally {
      setIsSubmitting(false);
    }
  };

  // Extract stage name from portal ID for display
  const stageNameMatch = portalId.match(/portal:STAGE_COMPLETION:(stage:)?(.+)/);
  const stageName = stageNameMatch ? stageNameMatch[2].replace(/_/g, ' ') : portalId;

  return (
    <div
      style={{
        padding: 'var(--space4)',
        backgroundColor: 'var(--bg)',
        borderRadius: 'var(--radius)',
        border: '1px solid var(--border)',
        marginBottom: 'var(--space4)',
      }}
    >
      <h4 style={{ margin: '0 0 var(--space2) 0', fontSize: '1rem' }}>
        Record Stage Approval
      </h4>
      <div style={{ marginBottom: 'var(--space4)', fontSize: '0.875rem', color: 'var(--muted)' }}>
        Recording approval for stage: <strong>{stageName}</strong>
      </div>

      {successMessage && <div className={styles.success}>{successMessage}</div>}

      {error && <div className={styles.error}>{error}</div>}

      <form onSubmit={handleSubmit} className={styles.form}>
        {/* Decision */}
        <div className={styles.formGroup}>
          <label className={styles.label}>Decision *</label>
          <div style={{ display: 'flex', gap: 'var(--space4)', flexWrap: 'wrap' }}>
            {(['APPROVED', 'REJECTED', 'DEFERRED'] as const).map((d) => (
              <label
                key={d}
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: 'var(--space1)',
                  cursor: isSubmitting ? 'default' : 'pointer',
                }}
              >
                <input
                  type="radio"
                  name="decision"
                  value={d}
                  checked={decision === d}
                  onChange={() => setDecision(d)}
                  disabled={isSubmitting}
                />
                <span style={{ fontSize: '0.875rem' }}>
                  {d.charAt(0) + d.slice(1).toLowerCase()}
                </span>
              </label>
            ))}
          </div>
        </div>

        {/* Rejection/Deferral Warning */}
        {decision === 'REJECTED' && (
          <div className={styles.note}>
            Rejecting this approval will prevent stage completion until a new approval is recorded.
          </div>
        )}
        {decision === 'DEFERRED' && (
          <div className={styles.note}>
            Deferring this approval will not count as an active approval for stage completion.
          </div>
        )}

        {/* Rationale */}
        <div className={styles.formGroup}>
          <label className={styles.label}>Rationale *</label>
          <textarea
            className={styles.textarea}
            placeholder="Explain your decision for audit purposes..."
            value={rationale}
            onChange={(e) => setRationale(e.target.value)}
            disabled={isSubmitting}
            rows={3}
            style={{ resize: 'vertical' }}
          />
        </div>

        {/* Evidence Refs */}
        <div className={styles.formGroup}>
          <label className={styles.label}>Evidence References</label>
          <input
            type="text"
            className={styles.input}
            placeholder="evidence:abc123, evidence:def456"
            value={evidenceRefs}
            onChange={(e) => setEvidenceRefs(e.target.value)}
            disabled={isSubmitting}
          />
          <span style={{ fontSize: '0.75rem', color: 'var(--muted)' }}>
            Comma-separated list of evidence references supporting this approval
          </span>
        </div>

        {/* Exceptions Acknowledged */}
        <div className={styles.formGroup}>
          <label className={styles.label}>Exceptions Acknowledged</label>
          <input
            type="text"
            className={styles.input}
            placeholder="exception:abc123, exception:def456"
            value={exceptionsAcknowledged}
            onChange={(e) => setExceptionsAcknowledged(e.target.value)}
            disabled={isSubmitting}
          />
          <span style={{ fontSize: '0.75rem', color: 'var(--muted)' }}>
            Comma-separated list of exception IDs acknowledged for this approval
          </span>
        </div>

        {/* Hidden fields info */}
        <div
          style={{
            padding: 'var(--space2)',
            backgroundColor: 'var(--bg-subtle)',
            borderRadius: 'var(--radius)',
            fontSize: '0.75rem',
            color: 'var(--muted)',
          }}
        >
          <div>
            <strong>Portal:</strong> {portalId}
          </div>
          <div>
            <strong>Work Surface:</strong> {workSurfaceId}
          </div>
        </div>

        {/* Buttons */}
        <div className={styles.buttonRow}>
          <Button variant="ghost" onClick={onCancel} disabled={isSubmitting}>
            Cancel
          </Button>
          <Button
            variant={decision === 'APPROVED' ? 'primary' : 'secondary'}
            disabled={isSubmitting}
          >
            {isSubmitting ? 'Recording...' : `Record ${decision.charAt(0) + decision.slice(1).toLowerCase()}`}
          </Button>
        </div>
      </form>
    </div>
  );
}

export default StageApprovalForm;

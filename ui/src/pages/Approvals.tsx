/**
 * Portal Workflows Page (D-30)
 *
 * Full portal workflows UI for human governance decisions:
 * - Approvals: binding decisions at portal touchpoints (HUMAN-only per SR-CONTRACT C-TB-3)
 * - Exceptions: deviations, deferrals, waivers (HUMAN-only per SR-SPEC 1.8)
 * - Decisions: precedent-setting judgments (HUMAN-only per SR-CONTRACT C-DEC-1)
 */

import { useState, useEffect, useCallback, FormEvent } from 'react';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Pill, Button, getStatusTone, truncate } from '../ui';
import styles from '../styles/pages.module.css';

// ============================================================================
// Types
// ============================================================================

interface ActorInfo {
  kind: string;
  id: string;
}

interface Approval {
  approval_id: string;
  portal_id: string;
  decision: string;
  subject_refs: unknown;
  evidence_refs: string[];
  exceptions_acknowledged: string[];
  rationale: string | null;
  approved_by: ActorInfo;
  approved_at: string;
}

interface Exception {
  exception_id: string;
  kind: string;
  status: string;
  scope: unknown;
  rationale: string;
  target_description: string;
  created_by: ActorInfo;
  created_at: string;
  expires_at: string | null;
  resolved_at: string | null;
  resolved_by: ActorInfo | null;
}

interface Decision {
  decision_id: string;
  trigger: string;
  scope: unknown;
  decision: string;
  rationale: string;
  is_precedent: boolean;
  applicability: string | null;
  decided_by: ActorInfo;
  decided_at: string;
}

// ============================================================================
// Tab Types
// ============================================================================

type TabType = 'approvals' | 'exceptions' | 'decisions';

// ============================================================================
// Component
// ============================================================================

export function Approvals(): JSX.Element {
  const auth = useAuth();
  const [activeTab, setActiveTab] = useState<TabType>('approvals');

  // Data states
  const [approvals, setApprovals] = useState<Approval[]>([]);
  const [exceptions, setExceptions] = useState<Exception[]>([]);
  const [decisions, setDecisions] = useState<Decision[]>([]);

  // UI states
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);
  const [showApprovalForm, setShowApprovalForm] = useState(false);
  const [showExceptionForm, setShowExceptionForm] = useState(false);
  const [showDecisionForm, setShowDecisionForm] = useState(false);

  // Form states - Approval
  const [approvalPortalId, setApprovalPortalId] = useState('');
  const [approvalDecision, setApprovalDecision] = useState('APPROVED');
  const [approvalRationale, setApprovalRationale] = useState('');
  const [approvalSubjectKind, setApprovalSubjectKind] = useState('');
  const [approvalSubjectId, setApprovalSubjectId] = useState('');
  const [approvalEvidenceRefs, setApprovalEvidenceRefs] = useState('');

  // Form states - Exception
  const [exceptionKind, setExceptionKind] = useState('DEVIATION');
  const [exceptionRationale, setExceptionRationale] = useState('');
  const [exceptionTarget, setExceptionTarget] = useState('');
  const [exceptionLoopId, setExceptionLoopId] = useState('');
  const [exceptionCandidateId, setExceptionCandidateId] = useState('');
  const [exceptionOracleId, setExceptionOracleId] = useState('');
  const [exceptionExpiresAt, setExceptionExpiresAt] = useState('');

  // Form states - Decision
  const [decisionTrigger, setDecisionTrigger] = useState('');
  const [decisionValue, setDecisionValue] = useState('');
  const [decisionRationale, setDecisionRationale] = useState('');
  const [decisionIsPrecedent, setDecisionIsPrecedent] = useState(false);
  const [decisionApplicability, setDecisionApplicability] = useState('');

  const getHeaders = (): Record<string, string> => {
    const h: Record<string, string> = { 'Content-Type': 'application/json' };
    if (auth.user?.access_token) {
      h.Authorization = `Bearer ${auth.user.access_token}`;
    }
    return h;
  };

  // ============================================================================
  // Data Fetching
  // ============================================================================

  const fetchApprovals = useCallback(async () => {
    if (!auth.user?.access_token) return;
    try {
      const res = await fetch(`${config.apiUrl}/api/v1/approvals`, { headers: getHeaders() });
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      const data = await res.json();
      setApprovals(data.approvals || []);
    } catch (err) {
      console.error('Failed to fetch approvals:', err);
    }
  }, [auth.user?.access_token]);

  const fetchExceptions = useCallback(async () => {
    if (!auth.user?.access_token) return;
    try {
      const res = await fetch(`${config.apiUrl}/api/v1/exceptions`, { headers: getHeaders() });
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      const data = await res.json();
      setExceptions(data.exceptions || []);
    } catch (err) {
      console.error('Failed to fetch exceptions:', err);
    }
  }, [auth.user?.access_token]);

  const fetchDecisions = useCallback(async () => {
    if (!auth.user?.access_token) return;
    try {
      const res = await fetch(`${config.apiUrl}/api/v1/decisions`, { headers: getHeaders() });
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      const data = await res.json();
      setDecisions(data.decisions || []);
    } catch (err) {
      console.error('Failed to fetch decisions:', err);
    }
  }, [auth.user?.access_token]);

  useEffect(() => {
    if (!auth.user?.access_token) return;
    setLoading(true);
    Promise.all([fetchApprovals(), fetchExceptions(), fetchDecisions()])
      .then(() => setLoading(false))
      .catch(err => {
        setError(err.message);
        setLoading(false);
      });
  }, [auth.user?.access_token, fetchApprovals, fetchExceptions, fetchDecisions]);

  // ============================================================================
  // Form Handlers
  // ============================================================================

  const clearMessages = () => {
    setError(null);
    setSuccess(null);
  };

  const resetApprovalForm = () => {
    setApprovalPortalId('');
    setApprovalDecision('APPROVED');
    setApprovalRationale('');
    setApprovalSubjectKind('');
    setApprovalSubjectId('');
    setApprovalEvidenceRefs('');
    setShowApprovalForm(false);
  };

  const resetExceptionForm = () => {
    setExceptionKind('DEVIATION');
    setExceptionRationale('');
    setExceptionTarget('');
    setExceptionLoopId('');
    setExceptionCandidateId('');
    setExceptionOracleId('');
    setExceptionExpiresAt('');
    setShowExceptionForm(false);
  };

  const resetDecisionForm = () => {
    setDecisionTrigger('');
    setDecisionValue('');
    setDecisionRationale('');
    setDecisionIsPrecedent(false);
    setDecisionApplicability('');
    setShowDecisionForm(false);
  };

  const handleSubmitApproval = async (e: FormEvent) => {
    e.preventDefault();
    clearMessages();

    if (!approvalPortalId || !approvalRationale) {
      setError('Portal ID and rationale are required');
      return;
    }

    const subjectRefs = approvalSubjectKind && approvalSubjectId
      ? [{ kind: approvalSubjectKind, id: approvalSubjectId, rel: 'approves' }]
      : [];

    const evidenceRefs = approvalEvidenceRefs
      .split(',')
      .map(s => s.trim())
      .filter(Boolean);

    try {
      const res = await fetch(`${config.apiUrl}/api/v1/approvals`, {
        method: 'POST',
        headers: getHeaders(),
        body: JSON.stringify({
          portal_id: approvalPortalId,
          decision: approvalDecision,
          subject_refs: subjectRefs,
          evidence_refs: evidenceRefs,
          rationale: approvalRationale,
        }),
      });

      if (!res.ok) {
        const errData = await res.json();
        throw new Error(errData.message || `HTTP ${res.status}`);
      }

      const data = await res.json();
      setSuccess(`Approval ${data.approval_id} recorded successfully`);
      resetApprovalForm();
      fetchApprovals();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to record approval');
    }
  };

  const handleSubmitException = async (e: FormEvent) => {
    e.preventDefault();
    clearMessages();

    if (!exceptionRationale) {
      setError('Rationale is required');
      return;
    }

    try {
      const res = await fetch(`${config.apiUrl}/api/v1/exceptions`, {
        method: 'POST',
        headers: getHeaders(),
        body: JSON.stringify({
          kind: exceptionKind,
          scope: {
            loop_id: exceptionLoopId || null,
            candidate_id: exceptionCandidateId || null,
            oracle_id: exceptionOracleId || null,
            artifact_refs: [],
          },
          rationale: exceptionRationale,
          target_description: exceptionTarget,
          expires_at: exceptionExpiresAt || null,
        }),
      });

      if (!res.ok) {
        const errData = await res.json();
        throw new Error(errData.message || `HTTP ${res.status}`);
      }

      const data = await res.json();
      setSuccess(`Exception ${data.exception_id} created successfully`);
      resetExceptionForm();
      fetchExceptions();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create exception');
    }
  };

  const handleActivateException = async (exceptionId: string) => {
    clearMessages();
    try {
      const res = await fetch(`${config.apiUrl}/api/v1/exceptions/${exceptionId}/activate`, {
        method: 'POST',
        headers: getHeaders(),
      });

      if (!res.ok) {
        const errData = await res.json();
        throw new Error(errData.message || `HTTP ${res.status}`);
      }

      setSuccess(`Exception ${exceptionId} activated`);
      fetchExceptions();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to activate exception');
    }
  };

  const handleResolveException = async (exceptionId: string) => {
    clearMessages();
    try {
      const res = await fetch(`${config.apiUrl}/api/v1/exceptions/${exceptionId}/resolve`, {
        method: 'POST',
        headers: getHeaders(),
        body: JSON.stringify({}),
      });

      if (!res.ok) {
        const errData = await res.json();
        throw new Error(errData.message || `HTTP ${res.status}`);
      }

      setSuccess(`Exception ${exceptionId} resolved`);
      fetchExceptions();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to resolve exception');
    }
  };

  const handleSubmitDecision = async (e: FormEvent) => {
    e.preventDefault();
    clearMessages();

    if (!decisionTrigger || !decisionValue || !decisionRationale) {
      setError('Trigger, decision, and rationale are required');
      return;
    }

    try {
      const res = await fetch(`${config.apiUrl}/api/v1/decisions`, {
        method: 'POST',
        headers: getHeaders(),
        body: JSON.stringify({
          trigger: decisionTrigger,
          scope: {},
          decision: decisionValue,
          rationale: decisionRationale,
          is_precedent: decisionIsPrecedent,
          applicability: decisionApplicability || null,
        }),
      });

      if (!res.ok) {
        const errData = await res.json();
        throw new Error(errData.message || `HTTP ${res.status}`);
      }

      const data = await res.json();
      setSuccess(`Decision ${data.decision_id} recorded successfully`);
      resetDecisionForm();
      fetchDecisions();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to record decision');
    }
  };

  // ============================================================================
  // Render Helpers
  // ============================================================================

  const renderApprovalsTab = () => (
    <>
      <Card
        title="Record Approval"
        right={
          <Button variant="secondary" onClick={() => setShowApprovalForm(!showApprovalForm)}>
            {showApprovalForm ? 'Cancel' : 'New Approval'}
          </Button>
        }
        className={styles.cardSpacing}
      >
        {showApprovalForm && (
          <form className={styles.form} onSubmit={handleSubmitApproval}>
            <div className={styles.formRow}>
              <div className={styles.formGroup}>
                <label className={styles.label}>Portal ID *</label>
                <input
                  className={styles.input}
                  type="text"
                  placeholder="e.g., portal:intake-acceptance"
                  value={approvalPortalId}
                  onChange={e => setApprovalPortalId(e.target.value)}
                  required
                />
              </div>
              <div className={styles.formGroup}>
                <label className={styles.label}>Decision *</label>
                <select
                  className={styles.select}
                  value={approvalDecision}
                  onChange={e => setApprovalDecision(e.target.value)}
                >
                  <option value="APPROVED">APPROVED</option>
                  <option value="REJECTED">REJECTED</option>
                  <option value="DEFERRED">DEFERRED</option>
                </select>
              </div>
            </div>

            <div className={styles.formRow}>
              <div className={styles.formGroup}>
                <label className={styles.label}>Subject Type</label>
                <input
                  className={styles.input}
                  type="text"
                  placeholder="e.g., Candidate, Intake"
                  value={approvalSubjectKind}
                  onChange={e => setApprovalSubjectKind(e.target.value)}
                />
              </div>
              <div className={styles.formGroup}>
                <label className={styles.label}>Subject ID</label>
                <input
                  className={styles.input}
                  type="text"
                  placeholder="e.g., cand_abc123"
                  value={approvalSubjectId}
                  onChange={e => setApprovalSubjectId(e.target.value)}
                />
              </div>
            </div>

            <div className={styles.formGroup}>
              <label className={styles.label}>Artifact Refs (comma-separated hashes)</label>
              <input
                className={styles.input}
                type="text"
                placeholder="sha256:abc123, sha256:def456"
                value={approvalEvidenceRefs}
                onChange={e => setApprovalEvidenceRefs(e.target.value)}
              />
            </div>

            <div className={styles.formGroup}>
              <label className={styles.label}>Rationale *</label>
              <textarea
                className={styles.textarea}
                placeholder="Explain the decision rationale..."
                value={approvalRationale}
                onChange={e => setApprovalRationale(e.target.value)}
                required
              />
            </div>

            <div className={styles.buttonRow}>
              <Button variant="secondary" type="button" onClick={resetApprovalForm}>
                Cancel
              </Button>
              <Button variant="primary" type="submit">
                Record Approval
              </Button>
            </div>
          </form>
        )}
      </Card>

      <Card title="Recorded Approvals">
        {approvals.length === 0 ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>No approvals recorded.</p>
            <p className={styles.placeholderHint}>
              Approvals bind candidates to freeze baselines at portal touchpoints.
            </p>
          </div>
        ) : (
          <table className={styles.table}>
            <thead>
              <tr>
                <th className={styles.th}>ID</th>
                <th className={styles.th}>Portal</th>
                <th className={styles.th}>Decision</th>
                <th className={styles.th}>Actor</th>
                <th className={styles.th}>Rationale</th>
                <th className={styles.th}>Recorded</th>
              </tr>
            </thead>
            <tbody>
              {approvals.map(approval => (
                <tr key={approval.approval_id}>
                  <td className={styles.td}>
                    <code className={styles.mono}>{approval.approval_id.substring(0, 12)}...</code>
                  </td>
                  <td className={styles.td}>{approval.portal_id}</td>
                  <td className={styles.td}>
                    <Pill tone={getStatusTone(approval.decision)}>{approval.decision}</Pill>
                  </td>
                  <td className={styles.td}>
                    <span style={{ fontSize: '0.75rem', color: 'var(--muted)' }}>
                      [{approval.approved_by.kind}]
                    </span>{' '}
                    {approval.approved_by.id.substring(0, 12)}...
                  </td>
                  <td className={styles.td}>
                    {approval.rationale ? truncate(approval.rationale, 50) : '-'}
                  </td>
                  <td className={styles.td}>{new Date(approval.approved_at).toLocaleString()}</td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </Card>
    </>
  );

  const renderExceptionsTab = () => (
    <>
      <Card
        title="Create Exception"
        right={
          <Button variant="secondary" onClick={() => setShowExceptionForm(!showExceptionForm)}>
            {showExceptionForm ? 'Cancel' : 'New Exception'}
          </Button>
        }
        className={styles.cardSpacing}
      >
        {showExceptionForm && (
          <form className={styles.form} onSubmit={handleSubmitException}>
            <div className={styles.formRow}>
              <div className={styles.formGroup}>
                <label className={styles.label}>Exception Kind *</label>
                <select
                  className={styles.select}
                  value={exceptionKind}
                  onChange={e => setExceptionKind(e.target.value)}
                >
                  <option value="DEVIATION">DEVIATION - Documented departure from spec</option>
                  <option value="DEFERRAL">DEFERRAL - Postponed requirement</option>
                  <option value="WAIVER">WAIVER - Bypassed oracle check (non-integrity)</option>
                </select>
              </div>
              <div className={styles.formGroup}>
                <label className={styles.label}>Target Description</label>
                <input
                  className={styles.input}
                  type="text"
                  placeholder="What is being excepted..."
                  value={exceptionTarget}
                  onChange={e => setExceptionTarget(e.target.value)}
                />
              </div>
            </div>

            <div className={styles.formRow}>
              <div className={styles.formGroup}>
                <label className={styles.label}>Loop ID (optional)</label>
                <input
                  className={styles.input}
                  type="text"
                  placeholder="loop_abc123"
                  value={exceptionLoopId}
                  onChange={e => setExceptionLoopId(e.target.value)}
                />
              </div>
              <div className={styles.formGroup}>
                <label className={styles.label}>Candidate ID (optional)</label>
                <input
                  className={styles.input}
                  type="text"
                  placeholder="cand_abc123"
                  value={exceptionCandidateId}
                  onChange={e => setExceptionCandidateId(e.target.value)}
                />
              </div>
            </div>

            <div className={styles.formRow}>
              <div className={styles.formGroup}>
                <label className={styles.label}>Oracle ID (optional)</label>
                <input
                  className={styles.input}
                  type="text"
                  placeholder="oracle:schema_compliance"
                  value={exceptionOracleId}
                  onChange={e => setExceptionOracleId(e.target.value)}
                />
              </div>
              <div className={styles.formGroup}>
                <label className={styles.label}>Expires At (optional)</label>
                <input
                  className={styles.input}
                  type="datetime-local"
                  value={exceptionExpiresAt}
                  onChange={e => setExceptionExpiresAt(e.target.value)}
                />
              </div>
            </div>

            <div className={styles.formGroup}>
              <label className={styles.label}>Rationale *</label>
              <textarea
                className={styles.textarea}
                placeholder="Explain why this exception is necessary..."
                value={exceptionRationale}
                onChange={e => setExceptionRationale(e.target.value)}
                required
              />
            </div>

            <div className={styles.note}>
              <strong>Warning:</strong> Waivers cannot target integrity conditions (ORACLE_TAMPER,
              ORACLE_GAP, ORACLE_ENV_MISMATCH, ORACLE_FLAKE, EVIDENCE_MISSING).
            </div>

            <div className={styles.buttonRow}>
              <Button variant="secondary" type="button" onClick={resetExceptionForm}>
                Cancel
              </Button>
              <Button variant="primary" type="submit">
                Create Exception
              </Button>
            </div>
          </form>
        )}
      </Card>

      <Card title="Exceptions">
        {exceptions.length === 0 ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>No exceptions recorded.</p>
            <p className={styles.placeholderHint}>
              Exceptions are narrowly scoped permissions to deviate from governing context.
            </p>
          </div>
        ) : (
          <table className={styles.table}>
            <thead>
              <tr>
                <th className={styles.th}>ID</th>
                <th className={styles.th}>Kind</th>
                <th className={styles.th}>Status</th>
                <th className={styles.th}>Target</th>
                <th className={styles.th}>Created By</th>
                <th className={styles.th}>Created</th>
                <th className={styles.th}>Actions</th>
              </tr>
            </thead>
            <tbody>
              {exceptions.map(exception => (
                <tr key={exception.exception_id}>
                  <td className={styles.td}>
                    <code className={styles.mono}>
                      {exception.exception_id.substring(0, 12)}...
                    </code>
                  </td>
                  <td className={styles.td}>
                    <Pill tone={getStatusTone(exception.kind)}>{exception.kind}</Pill>
                  </td>
                  <td className={styles.td}>
                    <Pill tone={getStatusTone(exception.status)}>{exception.status}</Pill>
                  </td>
                  <td className={styles.td}>
                    {exception.target_description || '-'}
                  </td>
                  <td className={styles.td}>
                    <span style={{ fontSize: '0.75rem', color: 'var(--muted)' }}>
                      [{exception.created_by.kind}]
                    </span>{' '}
                    {exception.created_by.id.substring(0, 12)}...
                  </td>
                  <td className={styles.td}>{new Date(exception.created_at).toLocaleString()}</td>
                  <td className={styles.td}>
                    {exception.status === 'CREATED' && (
                      <button
                        className={styles.actionLink}
                        onClick={() => handleActivateException(exception.exception_id)}
                      >
                        Activate
                      </button>
                    )}
                    {exception.status === 'ACTIVE' && (
                      <button
                        className={styles.actionLink}
                        onClick={() => handleResolveException(exception.exception_id)}
                      >
                        Resolve
                      </button>
                    )}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </Card>
    </>
  );

  const renderDecisionsTab = () => (
    <>
      <Card
        title="Record Decision"
        right={
          <Button variant="secondary" onClick={() => setShowDecisionForm(!showDecisionForm)}>
            {showDecisionForm ? 'Cancel' : 'New Decision'}
          </Button>
        }
        className={styles.cardSpacing}
      >
        {showDecisionForm && (
          <form className={styles.form} onSubmit={handleSubmitDecision}>
            <div className={styles.formGroup}>
              <label className={styles.label}>Trigger *</label>
              <input
                className={styles.input}
                type="text"
                placeholder="What triggered this decision point..."
                value={decisionTrigger}
                onChange={e => setDecisionTrigger(e.target.value)}
                required
              />
            </div>

            <div className={styles.formGroup}>
              <label className={styles.label}>Decision *</label>
              <input
                className={styles.input}
                type="text"
                placeholder="The decision made..."
                value={decisionValue}
                onChange={e => setDecisionValue(e.target.value)}
                required
              />
            </div>

            <div className={styles.formGroup}>
              <label className={styles.label}>Rationale *</label>
              <textarea
                className={styles.textarea}
                placeholder="Explain the reasoning behind this decision..."
                value={decisionRationale}
                onChange={e => setDecisionRationale(e.target.value)}
                required
              />
            </div>

            <div className={styles.formGroup}>
              <label className={styles.label} style={{ display: 'flex', alignItems: 'center', gap: 'var(--space2)' }}>
                <input
                  type="checkbox"
                  checked={decisionIsPrecedent}
                  onChange={e => setDecisionIsPrecedent(e.target.checked)}
                />
                This decision sets precedent
              </label>
            </div>

            {decisionIsPrecedent && (
              <div className={styles.formGroup}>
                <label className={styles.label}>Applicability Clause</label>
                <textarea
                  className={styles.textarea}
                  placeholder="When does this precedent apply..."
                  value={decisionApplicability}
                  onChange={e => setDecisionApplicability(e.target.value)}
                />
              </div>
            )}

            <div className={styles.buttonRow}>
              <Button variant="secondary" type="button" onClick={resetDecisionForm}>
                Cancel
              </Button>
              <Button variant="primary" type="submit">
                Record Decision
              </Button>
            </div>
          </form>
        )}
      </Card>

      <Card title="Recorded Decisions">
        {decisions.length === 0 ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>No decisions recorded.</p>
            <p className={styles.placeholderHint}>
              Decisions record binding human judgments and may set precedent for future governance.
            </p>
          </div>
        ) : (
          <table className={styles.table}>
            <thead>
              <tr>
                <th className={styles.th}>ID</th>
                <th className={styles.th}>Trigger</th>
                <th className={styles.th}>Decision</th>
                <th className={styles.th}>Precedent</th>
                <th className={styles.th}>Decided By</th>
                <th className={styles.th}>Date</th>
              </tr>
            </thead>
            <tbody>
              {decisions.map(decision => (
                <tr key={decision.decision_id}>
                  <td className={styles.td}>
                    <code className={styles.mono}>
                      {decision.decision_id.substring(0, 12)}...
                    </code>
                  </td>
                  <td className={styles.td}>{truncate(decision.trigger, 30)}</td>
                  <td className={styles.td}>{truncate(decision.decision, 30)}</td>
                  <td className={styles.td}>
                    {decision.is_precedent ? (
                      <Pill tone="success">Yes</Pill>
                    ) : (
                      '-'
                    )}
                  </td>
                  <td className={styles.td}>
                    <span style={{ fontSize: '0.75rem', color: 'var(--muted)' }}>
                      [{decision.decided_by.kind}]
                    </span>{' '}
                    {decision.decided_by.id.substring(0, 12)}...
                  </td>
                  <td className={styles.td}>{new Date(decision.decided_at).toLocaleString()}</td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </Card>
    </>
  );

  // ============================================================================
  // Main Render
  // ============================================================================

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <h1 className={styles.title}>Portal Workflows</h1>
      </div>

      <div className={styles.note}>
        <strong>Governance Note:</strong> Per SR-CONTRACT, all portal actions require HUMAN actor
        kind. Approvals (C-TB-3), exceptions (1.8), and decisions (C-DEC-1) are binding records.
        Waivers cannot target integrity conditions.
      </div>

      {error && <div className={styles.error}>{error}</div>}
      {success && <div className={styles.success}>{success}</div>}

      <div className={styles.tabs}>
        <button
          className={`${styles.tab} ${activeTab === 'approvals' ? styles.tabActive : ''}`}
          onClick={() => {
            setActiveTab('approvals');
            clearMessages();
          }}
        >
          Approvals ({approvals.length})
        </button>
        <button
          className={`${styles.tab} ${activeTab === 'exceptions' ? styles.tabActive : ''}`}
          onClick={() => {
            setActiveTab('exceptions');
            clearMessages();
          }}
        >
          Exceptions ({exceptions.length})
        </button>
        <button
          className={`${styles.tab} ${activeTab === 'decisions' ? styles.tabActive : ''}`}
          onClick={() => {
            setActiveTab('decisions');
            clearMessages();
          }}
        >
          Decisions ({decisions.length})
        </button>
      </div>

      {loading ? (
        <Card>
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>Loading portal data...</p>
          </div>
        </Card>
      ) : (
        <>
          {activeTab === 'approvals' && renderApprovalsTab()}
          {activeTab === 'exceptions' && renderExceptionsTab()}
          {activeTab === 'decisions' && renderDecisionsTab()}
        </>
      )}
    </div>
  );
}

export default Approvals;

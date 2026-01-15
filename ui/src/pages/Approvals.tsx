/**
 * Portal Workflows Page (D-30)
 *
 * Full portal workflows UI for human governance decisions:
 * - Approvals: binding decisions at portal touchpoints (HUMAN-only per SR-CONTRACT C-TB-3)
 * - Exceptions: deviations, deferrals, waivers (HUMAN-only per SR-SPEC §1.8)
 * - Decisions: precedent-setting judgments (HUMAN-only per SR-CONTRACT C-DEC-1)
 */

import { useState, useEffect, useCallback, FormEvent } from 'react';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';

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
// Styles
// ============================================================================

const styles = {
  container: {
    maxWidth: '1200px',
    margin: '0 auto',
  },
  header: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: '1.5rem',
  },
  title: {
    margin: 0,
    fontSize: '1.5rem',
    color: '#1a1a2e',
  },
  tabs: {
    display: 'flex',
    gap: '0.5rem',
    marginBottom: '1.5rem',
  },
  tab: {
    padding: '0.5rem 1rem',
    border: 'none',
    borderRadius: '4px',
    cursor: 'pointer',
    fontSize: '0.875rem',
    transition: 'background-color 0.2s',
  },
  tabActive: {
    backgroundColor: '#1a1a2e',
    color: 'white',
  },
  tabInactive: {
    backgroundColor: '#e5e5e5',
    color: '#333',
  },
  card: {
    backgroundColor: 'white',
    borderRadius: '8px',
    padding: '1.5rem',
    boxShadow: '0 1px 3px rgba(0, 0, 0, 0.1)',
    marginBottom: '1.5rem',
  },
  cardTitle: {
    margin: '0 0 1rem 0',
    fontSize: '1rem',
    color: '#1a1a2e',
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
  },
  table: {
    width: '100%',
    borderCollapse: 'collapse' as const,
  },
  th: {
    textAlign: 'left' as const,
    padding: '0.75rem',
    borderBottom: '2px solid #e5e5e5',
    color: '#666',
    fontSize: '0.75rem',
    textTransform: 'uppercase' as const,
  },
  td: {
    padding: '0.75rem',
    borderBottom: '1px solid #e5e5e5',
    fontSize: '0.875rem',
  },
  statusBadge: {
    display: 'inline-block',
    padding: '0.25rem 0.5rem',
    borderRadius: '4px',
    fontSize: '0.75rem',
    fontWeight: 500,
  },
  placeholder: {
    textAlign: 'center' as const,
    padding: '3rem',
    color: '#666',
  },
  note: {
    backgroundColor: '#fff3cd',
    padding: '1rem',
    borderRadius: '4px',
    marginBottom: '1.5rem',
    fontSize: '0.875rem',
    color: '#856404',
  },
  form: {
    display: 'flex',
    flexDirection: 'column' as const,
    gap: '1rem',
  },
  formGroup: {
    display: 'flex',
    flexDirection: 'column' as const,
    gap: '0.5rem',
  },
  formRow: {
    display: 'grid',
    gridTemplateColumns: '1fr 1fr',
    gap: '1rem',
  },
  label: {
    fontSize: '0.875rem',
    fontWeight: 500,
    color: '#333',
  },
  input: {
    padding: '0.5rem 0.75rem',
    border: '1px solid #ccc',
    borderRadius: '4px',
    fontSize: '0.875rem',
  },
  select: {
    padding: '0.5rem 0.75rem',
    border: '1px solid #ccc',
    borderRadius: '4px',
    fontSize: '0.875rem',
    backgroundColor: 'white',
  },
  textarea: {
    padding: '0.5rem 0.75rem',
    border: '1px solid #ccc',
    borderRadius: '4px',
    fontSize: '0.875rem',
    minHeight: '100px',
    resize: 'vertical' as const,
    fontFamily: 'inherit',
  },
  button: {
    padding: '0.75rem 1.5rem',
    border: 'none',
    borderRadius: '4px',
    fontSize: '0.875rem',
    cursor: 'pointer',
    transition: 'background-color 0.2s',
  },
  buttonPrimary: {
    backgroundColor: '#1a1a2e',
    color: 'white',
  },
  buttonSecondary: {
    backgroundColor: '#e5e5e5',
    color: '#333',
  },
  buttonDanger: {
    backgroundColor: '#dc3545',
    color: 'white',
  },
  buttonSmall: {
    padding: '0.25rem 0.5rem',
    fontSize: '0.75rem',
  },
  buttonRow: {
    display: 'flex',
    gap: '0.5rem',
    justifyContent: 'flex-end',
  },
  error: {
    color: '#dc3545',
    fontSize: '0.875rem',
    padding: '0.5rem',
    backgroundColor: '#f8d7da',
    borderRadius: '4px',
  },
  success: {
    color: '#155724',
    fontSize: '0.875rem',
    padding: '0.5rem',
    backgroundColor: '#d4edda',
    borderRadius: '4px',
  },
  monospace: {
    fontFamily: 'monospace',
    fontSize: '0.75rem',
    backgroundColor: '#f5f5f5',
    padding: '0.25rem 0.5rem',
    borderRadius: '4px',
  },
  actionLink: {
    color: '#0066cc',
    background: 'none',
    border: 'none',
    cursor: 'pointer',
    fontSize: '0.75rem',
    padding: '0.25rem 0.5rem',
    textDecoration: 'underline',
  },
};

const statusColors: Record<string, { bg: string; color: string }> = {
  APPROVED: { bg: '#d4edda', color: '#155724' },
  REJECTED: { bg: '#f8d7da', color: '#721c24' },
  DEFERRED: { bg: '#fff3cd', color: '#856404' },
  CREATED: { bg: '#e2e3e5', color: '#383d41' },
  ACTIVE: { bg: '#cce5ff', color: '#004085' },
  RESOLVED: { bg: '#d4edda', color: '#155724' },
  EXPIRED: { bg: '#e2e3e5', color: '#383d41' },
  DEVIATION: { bg: '#fff3cd', color: '#856404' },
  DEFERRAL: { bg: '#cce5ff', color: '#004085' },
  WAIVER: { bg: '#f8d7da', color: '#721c24' },
};

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

  const renderStatusBadge = (status: string) => {
    const style = statusColors[status] || { bg: '#e2e3e5', color: '#383d41' };
    return (
      <span style={{ ...styles.statusBadge, backgroundColor: style.bg, color: style.color }}>
        {status}
      </span>
    );
  };

  const renderApprovalsTab = () => (
    <>
      <div style={styles.card}>
        <div style={styles.cardTitle}>
          <span>Record Approval</span>
          <button
            style={{ ...styles.button, ...styles.buttonSecondary, ...styles.buttonSmall }}
            onClick={() => setShowApprovalForm(!showApprovalForm)}
          >
            {showApprovalForm ? 'Cancel' : 'New Approval'}
          </button>
        </div>

        {showApprovalForm && (
          <form style={styles.form} onSubmit={handleSubmitApproval}>
            <div style={styles.formRow}>
              <div style={styles.formGroup}>
                <label style={styles.label}>Portal ID *</label>
                <input
                  style={styles.input}
                  type="text"
                  placeholder="e.g., portal:intake-acceptance"
                  value={approvalPortalId}
                  onChange={e => setApprovalPortalId(e.target.value)}
                  required
                />
              </div>
              <div style={styles.formGroup}>
                <label style={styles.label}>Decision *</label>
                <select
                  style={styles.select}
                  value={approvalDecision}
                  onChange={e => setApprovalDecision(e.target.value)}
                >
                  <option value="APPROVED">APPROVED</option>
                  <option value="REJECTED">REJECTED</option>
                  <option value="DEFERRED">DEFERRED</option>
                </select>
              </div>
            </div>

            <div style={styles.formRow}>
              <div style={styles.formGroup}>
                <label style={styles.label}>Subject Type</label>
                <input
                  style={styles.input}
                  type="text"
                  placeholder="e.g., Candidate, Intake"
                  value={approvalSubjectKind}
                  onChange={e => setApprovalSubjectKind(e.target.value)}
                />
              </div>
              <div style={styles.formGroup}>
                <label style={styles.label}>Subject ID</label>
                <input
                  style={styles.input}
                  type="text"
                  placeholder="e.g., cand_abc123"
                  value={approvalSubjectId}
                  onChange={e => setApprovalSubjectId(e.target.value)}
                />
              </div>
            </div>

            <div style={styles.formGroup}>
              <label style={styles.label}>Artifact Refs (comma-separated hashes)</label>
              <input
                style={styles.input}
                type="text"
                placeholder="sha256:abc123, sha256:def456"
                value={approvalEvidenceRefs}
                onChange={e => setApprovalEvidenceRefs(e.target.value)}
              />
            </div>

            <div style={styles.formGroup}>
              <label style={styles.label}>Rationale *</label>
              <textarea
                style={styles.textarea}
                placeholder="Explain the decision rationale..."
                value={approvalRationale}
                onChange={e => setApprovalRationale(e.target.value)}
                required
              />
            </div>

            <div style={styles.buttonRow}>
              <button
                type="button"
                style={{ ...styles.button, ...styles.buttonSecondary }}
                onClick={resetApprovalForm}
              >
                Cancel
              </button>
              <button type="submit" style={{ ...styles.button, ...styles.buttonPrimary }}>
                Record Approval
              </button>
            </div>
          </form>
        )}
      </div>

      <div style={styles.card}>
        <h2 style={styles.cardTitle}>Recorded Approvals</h2>
        {approvals.length === 0 ? (
          <div style={styles.placeholder}>
            <p>No approvals recorded.</p>
            <p style={{ fontSize: '0.875rem', color: '#999' }}>
              Approvals bind candidates to freeze baselines at portal touchpoints.
            </p>
          </div>
        ) : (
          <table style={styles.table}>
            <thead>
              <tr>
                <th style={styles.th}>ID</th>
                <th style={styles.th}>Portal</th>
                <th style={styles.th}>Decision</th>
                <th style={styles.th}>Actor</th>
                <th style={styles.th}>Rationale</th>
                <th style={styles.th}>Recorded</th>
              </tr>
            </thead>
            <tbody>
              {approvals.map(approval => (
                <tr key={approval.approval_id}>
                  <td style={styles.td}>
                    <code style={styles.monospace}>{approval.approval_id.substring(0, 12)}...</code>
                  </td>
                  <td style={styles.td}>{approval.portal_id}</td>
                  <td style={styles.td}>{renderStatusBadge(approval.decision)}</td>
                  <td style={styles.td}>
                    <span style={{ fontSize: '0.75rem', color: '#666' }}>
                      [{approval.approved_by.kind}]
                    </span>{' '}
                    {approval.approved_by.id.substring(0, 12)}...
                  </td>
                  <td style={styles.td}>
                    {approval.rationale
                      ? approval.rationale.length > 50
                        ? `${approval.rationale.substring(0, 50)}...`
                        : approval.rationale
                      : '-'}
                  </td>
                  <td style={styles.td}>{new Date(approval.approved_at).toLocaleString()}</td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>
    </>
  );

  const renderExceptionsTab = () => (
    <>
      <div style={styles.card}>
        <div style={styles.cardTitle}>
          <span>Create Exception</span>
          <button
            style={{ ...styles.button, ...styles.buttonSecondary, ...styles.buttonSmall }}
            onClick={() => setShowExceptionForm(!showExceptionForm)}
          >
            {showExceptionForm ? 'Cancel' : 'New Exception'}
          </button>
        </div>

        {showExceptionForm && (
          <form style={styles.form} onSubmit={handleSubmitException}>
            <div style={styles.formRow}>
              <div style={styles.formGroup}>
                <label style={styles.label}>Exception Kind *</label>
                <select
                  style={styles.select}
                  value={exceptionKind}
                  onChange={e => setExceptionKind(e.target.value)}
                >
                  <option value="DEVIATION">DEVIATION - Documented departure from spec</option>
                  <option value="DEFERRAL">DEFERRAL - Postponed requirement</option>
                  <option value="WAIVER">WAIVER - Bypassed oracle check (non-integrity)</option>
                </select>
              </div>
              <div style={styles.formGroup}>
                <label style={styles.label}>Target Description</label>
                <input
                  style={styles.input}
                  type="text"
                  placeholder="What is being excepted..."
                  value={exceptionTarget}
                  onChange={e => setExceptionTarget(e.target.value)}
                />
              </div>
            </div>

            <div style={styles.formRow}>
              <div style={styles.formGroup}>
                <label style={styles.label}>Loop ID (optional)</label>
                <input
                  style={styles.input}
                  type="text"
                  placeholder="loop_abc123"
                  value={exceptionLoopId}
                  onChange={e => setExceptionLoopId(e.target.value)}
                />
              </div>
              <div style={styles.formGroup}>
                <label style={styles.label}>Candidate ID (optional)</label>
                <input
                  style={styles.input}
                  type="text"
                  placeholder="cand_abc123"
                  value={exceptionCandidateId}
                  onChange={e => setExceptionCandidateId(e.target.value)}
                />
              </div>
            </div>

            <div style={styles.formRow}>
              <div style={styles.formGroup}>
                <label style={styles.label}>Oracle ID (optional)</label>
                <input
                  style={styles.input}
                  type="text"
                  placeholder="oracle:schema_compliance"
                  value={exceptionOracleId}
                  onChange={e => setExceptionOracleId(e.target.value)}
                />
              </div>
              <div style={styles.formGroup}>
                <label style={styles.label}>Expires At (optional)</label>
                <input
                  style={styles.input}
                  type="datetime-local"
                  value={exceptionExpiresAt}
                  onChange={e => setExceptionExpiresAt(e.target.value)}
                />
              </div>
            </div>

            <div style={styles.formGroup}>
              <label style={styles.label}>Rationale *</label>
              <textarea
                style={styles.textarea}
                placeholder="Explain why this exception is necessary..."
                value={exceptionRationale}
                onChange={e => setExceptionRationale(e.target.value)}
                required
              />
            </div>

            <div style={{ ...styles.note, marginTop: '0.5rem' }}>
              <strong>Warning:</strong> Waivers cannot target integrity conditions (ORACLE_TAMPER,
              ORACLE_GAP, ORACLE_ENV_MISMATCH, ORACLE_FLAKE, EVIDENCE_MISSING).
            </div>

            <div style={styles.buttonRow}>
              <button
                type="button"
                style={{ ...styles.button, ...styles.buttonSecondary }}
                onClick={resetExceptionForm}
              >
                Cancel
              </button>
              <button type="submit" style={{ ...styles.button, ...styles.buttonPrimary }}>
                Create Exception
              </button>
            </div>
          </form>
        )}
      </div>

      <div style={styles.card}>
        <h2 style={styles.cardTitle}>Exceptions</h2>
        {exceptions.length === 0 ? (
          <div style={styles.placeholder}>
            <p>No exceptions recorded.</p>
            <p style={{ fontSize: '0.875rem', color: '#999' }}>
              Exceptions are narrowly scoped permissions to deviate from governing context.
            </p>
          </div>
        ) : (
          <table style={styles.table}>
            <thead>
              <tr>
                <th style={styles.th}>ID</th>
                <th style={styles.th}>Kind</th>
                <th style={styles.th}>Status</th>
                <th style={styles.th}>Target</th>
                <th style={styles.th}>Created By</th>
                <th style={styles.th}>Created</th>
                <th style={styles.th}>Actions</th>
              </tr>
            </thead>
            <tbody>
              {exceptions.map(exception => (
                <tr key={exception.exception_id}>
                  <td style={styles.td}>
                    <code style={styles.monospace}>
                      {exception.exception_id.substring(0, 12)}...
                    </code>
                  </td>
                  <td style={styles.td}>{renderStatusBadge(exception.kind)}</td>
                  <td style={styles.td}>{renderStatusBadge(exception.status)}</td>
                  <td style={styles.td}>
                    {exception.target_description || '-'}
                  </td>
                  <td style={styles.td}>
                    <span style={{ fontSize: '0.75rem', color: '#666' }}>
                      [{exception.created_by.kind}]
                    </span>{' '}
                    {exception.created_by.id.substring(0, 12)}...
                  </td>
                  <td style={styles.td}>{new Date(exception.created_at).toLocaleString()}</td>
                  <td style={styles.td}>
                    {exception.status === 'CREATED' && (
                      <button
                        style={styles.actionLink}
                        onClick={() => handleActivateException(exception.exception_id)}
                      >
                        Activate
                      </button>
                    )}
                    {exception.status === 'ACTIVE' && (
                      <button
                        style={styles.actionLink}
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
      </div>
    </>
  );

  const renderDecisionsTab = () => (
    <>
      <div style={styles.card}>
        <div style={styles.cardTitle}>
          <span>Record Decision</span>
          <button
            style={{ ...styles.button, ...styles.buttonSecondary, ...styles.buttonSmall }}
            onClick={() => setShowDecisionForm(!showDecisionForm)}
          >
            {showDecisionForm ? 'Cancel' : 'New Decision'}
          </button>
        </div>

        {showDecisionForm && (
          <form style={styles.form} onSubmit={handleSubmitDecision}>
            <div style={styles.formGroup}>
              <label style={styles.label}>Trigger *</label>
              <input
                style={styles.input}
                type="text"
                placeholder="What triggered this decision point..."
                value={decisionTrigger}
                onChange={e => setDecisionTrigger(e.target.value)}
                required
              />
            </div>

            <div style={styles.formGroup}>
              <label style={styles.label}>Decision *</label>
              <input
                style={styles.input}
                type="text"
                placeholder="The decision made..."
                value={decisionValue}
                onChange={e => setDecisionValue(e.target.value)}
                required
              />
            </div>

            <div style={styles.formGroup}>
              <label style={styles.label}>Rationale *</label>
              <textarea
                style={styles.textarea}
                placeholder="Explain the reasoning behind this decision..."
                value={decisionRationale}
                onChange={e => setDecisionRationale(e.target.value)}
                required
              />
            </div>

            <div style={styles.formRow}>
              <div style={styles.formGroup}>
                <label style={styles.label}>
                  <input
                    type="checkbox"
                    checked={decisionIsPrecedent}
                    onChange={e => setDecisionIsPrecedent(e.target.checked)}
                    style={{ marginRight: '0.5rem' }}
                  />
                  This decision sets precedent
                </label>
              </div>
            </div>

            {decisionIsPrecedent && (
              <div style={styles.formGroup}>
                <label style={styles.label}>Applicability Clause</label>
                <textarea
                  style={styles.textarea}
                  placeholder="When does this precedent apply..."
                  value={decisionApplicability}
                  onChange={e => setDecisionApplicability(e.target.value)}
                />
              </div>
            )}

            <div style={styles.buttonRow}>
              <button
                type="button"
                style={{ ...styles.button, ...styles.buttonSecondary }}
                onClick={resetDecisionForm}
              >
                Cancel
              </button>
              <button type="submit" style={{ ...styles.button, ...styles.buttonPrimary }}>
                Record Decision
              </button>
            </div>
          </form>
        )}
      </div>

      <div style={styles.card}>
        <h2 style={styles.cardTitle}>Recorded Decisions</h2>
        {decisions.length === 0 ? (
          <div style={styles.placeholder}>
            <p>No decisions recorded.</p>
            <p style={{ fontSize: '0.875rem', color: '#999' }}>
              Decisions record binding human judgments and may set precedent for future governance.
            </p>
          </div>
        ) : (
          <table style={styles.table}>
            <thead>
              <tr>
                <th style={styles.th}>ID</th>
                <th style={styles.th}>Trigger</th>
                <th style={styles.th}>Decision</th>
                <th style={styles.th}>Precedent</th>
                <th style={styles.th}>Decided By</th>
                <th style={styles.th}>Date</th>
              </tr>
            </thead>
            <tbody>
              {decisions.map(decision => (
                <tr key={decision.decision_id}>
                  <td style={styles.td}>
                    <code style={styles.monospace}>
                      {decision.decision_id.substring(0, 12)}...
                    </code>
                  </td>
                  <td style={styles.td}>
                    {decision.trigger.length > 30
                      ? `${decision.trigger.substring(0, 30)}...`
                      : decision.trigger}
                  </td>
                  <td style={styles.td}>
                    {decision.decision.length > 30
                      ? `${decision.decision.substring(0, 30)}...`
                      : decision.decision}
                  </td>
                  <td style={styles.td}>
                    {decision.is_precedent ? (
                      <span
                        style={{
                          ...styles.statusBadge,
                          backgroundColor: '#cce5ff',
                          color: '#004085',
                        }}
                      >
                        Yes
                      </span>
                    ) : (
                      '-'
                    )}
                  </td>
                  <td style={styles.td}>
                    <span style={{ fontSize: '0.75rem', color: '#666' }}>
                      [{decision.decided_by.kind}]
                    </span>{' '}
                    {decision.decided_by.id.substring(0, 12)}...
                  </td>
                  <td style={styles.td}>{new Date(decision.decided_at).toLocaleString()}</td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>
    </>
  );

  // ============================================================================
  // Main Render
  // ============================================================================

  return (
    <div style={styles.container}>
      <div style={styles.header}>
        <h1 style={styles.title}>Portal Workflows</h1>
      </div>

      <div style={styles.note}>
        <strong>Governance Note:</strong> Per SR-CONTRACT, all portal actions require HUMAN actor
        kind. Approvals (C-TB-3), exceptions (§1.8), and decisions (C-DEC-1) are binding records.
        Waivers cannot target integrity conditions.
      </div>

      {error && <div style={styles.error}>{error}</div>}
      {success && <div style={styles.success}>{success}</div>}

      <div style={styles.tabs}>
        <button
          style={{
            ...styles.tab,
            ...(activeTab === 'approvals' ? styles.tabActive : styles.tabInactive),
          }}
          onClick={() => {
            setActiveTab('approvals');
            clearMessages();
          }}
        >
          Approvals ({approvals.length})
        </button>
        <button
          style={{
            ...styles.tab,
            ...(activeTab === 'exceptions' ? styles.tabActive : styles.tabInactive),
          }}
          onClick={() => {
            setActiveTab('exceptions');
            clearMessages();
          }}
        >
          Exceptions ({exceptions.length})
        </button>
        <button
          style={{
            ...styles.tab,
            ...(activeTab === 'decisions' ? styles.tabActive : styles.tabInactive),
          }}
          onClick={() => {
            setActiveTab('decisions');
            clearMessages();
          }}
        >
          Decisions ({decisions.length})
        </button>
      </div>

      {loading ? (
        <div style={styles.card}>
          <div style={styles.placeholder}>
            <p>Loading portal data...</p>
          </div>
        </div>
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

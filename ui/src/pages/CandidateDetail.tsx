/**
 * Candidate Detail Page (D-29, D-30)
 *
 * Displays a single candidate with its oracle runs, evidence bundles, and freeze records.
 * Per SR-SPEC, candidates are content-addressed artifacts produced by iterations.
 * D-30 adds freeze record creation from candidate context.
 */

import { useState, useEffect, FormEvent } from 'react';
import { useParams, Link } from 'react-router-dom';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Pill, Button, getStatusTone, truncateHash } from '../ui';
import styles from '../styles/pages.module.css';

interface TypedRef {
  type_key: string;
  id: string;
  rel: string;
}

interface Candidate {
  id: string;
  content_hash: string;
  git_sha: string | null;
  state: string;
  produced_by_iteration_id: string | null;
  materialized_at: string;
  refs: TypedRef[];
}

interface Run {
  id: string;
  candidate_id: string;
  oracle_suite_id: string;
  oracle_suite_hash: string;
  state: string;
  started_at: string;
  completed_at: string | null;
  outcome: string | null;
  evidence_bundle_hash: string | null;
}

interface EvidenceBundle {
  content_hash: string;
  manifest: {
    artifact_type: string;
    suite_id: string;
    verdict: string;
    oracle_results: OracleResult[];
    created_at: string;
  };
}

interface OracleResult {
  oracle_id: string;
  status: string;
  message: string | null;
}

interface FreezeRecord {
  id: string;
  candidate_id: string;
  created_at: string;
  verification_mode: string;
  artifact_manifest_hash: string;
}

export function CandidateDetail(): JSX.Element {
  const { candidateId } = useParams<{ candidateId: string }>();
  const auth = useAuth();
  const [candidate, setCandidate] = useState<Candidate | null>(null);
  const [runs, setRuns] = useState<Run[]>([]);
  const [evidence, setEvidence] = useState<EvidenceBundle[]>([]);
  const [freezeRecords, setFreezeRecords] = useState<FreezeRecord[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [activeTab, setActiveTab] = useState<'runs' | 'evidence' | 'freeze'>('runs');

  // Freeze record form states (D-30)
  const [showFreezeForm, setShowFreezeForm] = useState(false);
  const [freezeFormError, setFreezeFormError] = useState<string | null>(null);
  const [freezeFormSuccess, setFreezeFormSuccess] = useState<string | null>(null);
  const [freezeBaselineId, setFreezeBaselineId] = useState('');
  const [freezeVerificationMode, setFreezeVerificationMode] = useState('STRICT');
  const [freezeOracleSuiteId, setFreezeOracleSuiteId] = useState('');
  const [freezeOracleSuiteHash, setFreezeOracleSuiteHash] = useState('');
  const [freezeEvidenceBundleRefs, setFreezeEvidenceBundleRefs] = useState('');
  const [freezeApprovalId, setFreezeApprovalId] = useState('');

  const getHeaders = (): Record<string, string> => {
    const h: Record<string, string> = { 'Content-Type': 'application/json' };
    if (auth.user?.access_token) {
      h.Authorization = `Bearer ${auth.user.access_token}`;
    }
    return h;
  };

  const fetchFreezeRecords = async () => {
    if (!auth.user?.access_token || !candidateId) return;
    try {
      const res = await fetch(
        `${config.apiUrl}/api/v1/candidates/${candidateId}/freeze-records`,
        { headers: getHeaders() }
      );
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      const data = await res.json();
      setFreezeRecords(data.freeze_records || []);
    } catch (err) {
      console.error('Failed to fetch freeze records:', err);
    }
  };

  const resetFreezeForm = () => {
    setFreezeBaselineId('');
    setFreezeVerificationMode('STRICT');
    setFreezeOracleSuiteId('');
    setFreezeOracleSuiteHash('');
    setFreezeEvidenceBundleRefs('');
    setFreezeApprovalId('');
    setShowFreezeForm(false);
    setFreezeFormError(null);
    setFreezeFormSuccess(null);
  };

  const handleSubmitFreeze = async (e: FormEvent) => {
    e.preventDefault();
    setFreezeFormError(null);
    setFreezeFormSuccess(null);

    if (!freezeBaselineId || !freezeOracleSuiteId || !freezeApprovalId) {
      setFreezeFormError('Baseline ID, Oracle Suite ID, and Approval ID are required');
      return;
    }

    const evidenceBundleRefs = freezeEvidenceBundleRefs
      .split(',')
      .map(s => s.trim())
      .filter(Boolean);

    try {
      const res = await fetch(`${config.apiUrl}/api/v1/freeze-records`, {
        method: 'POST',
        headers: getHeaders(),
        body: JSON.stringify({
          baseline_id: freezeBaselineId,
          candidate_id: candidateId,
          verification_mode: freezeVerificationMode,
          oracle_suite_id: freezeOracleSuiteId,
          oracle_suite_hash: freezeOracleSuiteHash || 'sha256:pending',
          evidence_bundle_refs: evidenceBundleRefs,
          waiver_refs: [],
          release_approval_id: freezeApprovalId,
          artifact_manifest: [],
          active_exceptions: [],
        }),
      });

      if (!res.ok) {
        const errData = await res.json();
        throw new Error(errData.message || `HTTP ${res.status}`);
      }

      const data = await res.json();
      setFreezeFormSuccess(`Freeze record ${data.freeze_id} created successfully`);
      resetFreezeForm();
      fetchFreezeRecords();
    } catch (err) {
      setFreezeFormError(err instanceof Error ? err.message : 'Failed to create freeze record');
    }
  };

  useEffect(() => {
    if (!auth.user?.access_token || !candidateId) return;

    Promise.all([
      fetch(`${config.apiUrl}/api/v1/candidates/${candidateId}`, { headers: getHeaders() }).then(res => {
        if (!res.ok) throw new Error(`Candidate fetch failed: HTTP ${res.status}`);
        return res.json();
      }),
      fetch(`${config.apiUrl}/api/v1/candidates/${candidateId}/runs`, { headers: getHeaders() }).then(res => {
        if (!res.ok) throw new Error(`Runs fetch failed: HTTP ${res.status}`);
        return res.json();
      }),
      fetch(`${config.apiUrl}/api/v1/candidates/${candidateId}/evidence`, { headers: getHeaders() }).then(res => {
        if (!res.ok) throw new Error(`Evidence fetch failed: HTTP ${res.status}`);
        return res.json();
      }),
      fetch(`${config.apiUrl}/api/v1/candidates/${candidateId}/freeze-records`, { headers: getHeaders() }).then(res => {
        if (!res.ok) throw new Error(`Freeze records fetch failed: HTTP ${res.status}`);
        return res.json();
      }),
    ])
      .then(([candidateData, runsData, evidenceData, freezeData]) => {
        setCandidate(candidateData);
        setRuns(runsData.runs || []);
        setEvidence(evidenceData.evidence || []);
        setFreezeRecords(freezeData.freeze_records || []);
        setLoading(false);
      })
      .catch(err => {
        setError(err.message);
        setLoading(false);
      });
  }, [auth.user?.access_token, candidateId]);

  if (loading) {
    return (
      <div className={styles.container}>
        <div className={styles.placeholder}>
          <p className={styles.placeholderText}>Loading candidate details...</p>
        </div>
      </div>
    );
  }

  if (error || !candidate) {
    return (
      <div className={styles.container}>
        <div className={styles.placeholder}>
          <p className={styles.error}>Error: {error || 'Candidate not found'}</p>
          <Link to="/loops" className={styles.link}>Back to Workflows</Link>
        </div>
      </div>
    );
  }

  return (
    <div className={styles.container}>
      {/* Breadcrumb */}
      <div className={styles.breadcrumb}>
        <Link to="/loops" className={styles.breadcrumbLink}>Workflows</Link>
        {candidate.produced_by_iteration_id && (
          <>
            <span className={styles.breadcrumbSeparator}>/</span>
            <Link
              to={`/iterations/${candidate.produced_by_iteration_id}`}
              className={styles.breadcrumbLink}
            >
              Iteration
            </Link>
          </>
        )}
        <span className={styles.breadcrumbSeparator}>/</span>
        <span>Candidate</span>
      </div>

      {/* Header */}
      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>Candidate</h1>
          <p className={styles.subtitle}>{candidate.id}</p>
        </div>
        <Pill tone={getStatusTone(candidate.state)}>{candidate.state}</Pill>
      </div>

      {/* Overview Card */}
      <Card title="Overview" className={styles.cardSpacing}>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Content Hash</span>
          <code className={styles.mono}>{candidate.content_hash}</code>
        </div>
        {candidate.git_sha && (
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Git SHA</span>
            <code className={styles.mono}>{candidate.git_sha}</code>
          </div>
        )}
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Materialized</span>
          <span className={styles.infoValue}>
            {new Date(candidate.materialized_at).toLocaleString()}
          </span>
        </div>
        {candidate.produced_by_iteration_id && (
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Iteration</span>
            <Link
              to={`/iterations/${candidate.produced_by_iteration_id}`}
              className={styles.link}
            >
              {candidate.produced_by_iteration_id}
            </Link>
          </div>
        )}
      </Card>

      {/* Refs Card */}
      {candidate.refs && candidate.refs.length > 0 && (
        <Card title={`References (${candidate.refs.length})`} className={styles.cardSpacing}>
          <ul className={styles.refList}>
            {candidate.refs.map((ref, idx) => (
              <li key={idx} className={styles.refItem}>
                <span>{ref.type_key}:{ref.id}</span>
                <span className={styles.refRel}>({ref.rel})</span>
              </li>
            ))}
          </ul>
        </Card>
      )}

      {/* Tabbed Content Card */}
      <Card>
        <div className={styles.tabs}>
          <button
            className={`${styles.tab} ${activeTab === 'runs' ? styles.tabActive : ''}`}
            onClick={() => setActiveTab('runs')}
          >
            Oracle Runs ({runs.length})
          </button>
          <button
            className={`${styles.tab} ${activeTab === 'evidence' ? styles.tabActive : ''}`}
            onClick={() => setActiveTab('evidence')}
          >
            Artifacts ({evidence.length})
          </button>
          <button
            className={`${styles.tab} ${activeTab === 'freeze' ? styles.tabActive : ''}`}
            onClick={() => setActiveTab('freeze')}
          >
            Freeze Records ({freezeRecords.length})
          </button>
        </div>

        {/* Runs Tab */}
        {activeTab === 'runs' && (
          <>
            {runs.length === 0 ? (
              <div className={styles.placeholder}>
                <p className={styles.placeholderText}>No oracle runs for this candidate.</p>
              </div>
            ) : (
              <table className={styles.table}>
                <thead>
                  <tr>
                    <th className={styles.th}>Run ID</th>
                    <th className={styles.th}>Suite</th>
                    <th className={styles.th}>State</th>
                    <th className={styles.th}>Outcome</th>
                    <th className={styles.th}>Artifacts</th>
                    <th className={styles.th}>Started</th>
                  </tr>
                </thead>
                <tbody>
                  {runs.map(run => (
                    <tr key={run.id}>
                      <td className={styles.td}>{run.id}</td>
                      <td className={styles.td}>
                        <code className={styles.mono}>{run.oracle_suite_id}</code>
                      </td>
                      <td className={styles.td}>
                        <Pill tone={getStatusTone(run.state)}>{run.state}</Pill>
                      </td>
                      <td className={styles.td}>
                        {run.outcome ? (
                          <Pill tone={getStatusTone(run.outcome)}>{run.outcome}</Pill>
                        ) : (
                          <span style={{ color: 'var(--muted)' }}>-</span>
                        )}
                      </td>
                      <td className={styles.td}>
                        {run.evidence_bundle_hash ? (
                          <Link
                            to={`/evidence/${run.evidence_bundle_hash}`}
                            className={styles.link}
                          >
                            <code className={styles.mono}>
                              {truncateHash(run.evidence_bundle_hash, 12)}
                            </code>
                          </Link>
                        ) : (
                          <span style={{ color: 'var(--muted)' }}>-</span>
                        )}
                      </td>
                      <td className={styles.td}>
                        {new Date(run.started_at).toLocaleString()}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            )}
          </>
        )}

        {/* Artifacts Tab */}
        {activeTab === 'evidence' && (
          <>
            {evidence.length === 0 ? (
              <div className={styles.placeholder}>
                <p className={styles.placeholderText}>No artifact bundles associated with this candidate.</p>
              </div>
            ) : (
              <table className={styles.table}>
                <thead>
                  <tr>
                    <th className={styles.th}>Content Hash</th>
                    <th className={styles.th}>Type</th>
                    <th className={styles.th}>Suite</th>
                    <th className={styles.th}>Verdict</th>
                    <th className={styles.th}>Created</th>
                  </tr>
                </thead>
                <tbody>
                  {evidence.map(bundle => (
                    <tr key={bundle.content_hash}>
                      <td className={styles.td}>
                        <Link
                          to={`/evidence/${bundle.content_hash}`}
                          className={styles.link}
                        >
                          <code className={styles.mono}>
                            {truncateHash(bundle.content_hash, 16)}
                          </code>
                        </Link>
                      </td>
                      <td className={styles.td}>{bundle.manifest.artifact_type}</td>
                      <td className={styles.td}>
                        <code className={styles.mono}>{bundle.manifest.suite_id}</code>
                      </td>
                      <td className={styles.td}>
                        <Pill tone={getStatusTone(bundle.manifest.verdict)}>
                          {bundle.manifest.verdict}
                        </Pill>
                      </td>
                      <td className={styles.td}>
                        {new Date(bundle.manifest.created_at).toLocaleString()}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            )}
          </>
        )}

        {/* Freeze Records Tab */}
        {activeTab === 'freeze' && (
          <>
            {/* Create Freeze Form Header (D-30) */}
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 'var(--space4)' }}>
              <span style={{ fontSize: '0.875rem', fontWeight: 500, color: 'var(--ink)' }}>Freeze Records</span>
              <Button
                variant="secondary"
                onClick={() => setShowFreezeForm(!showFreezeForm)}
              >
                {showFreezeForm ? 'Cancel' : 'Create Freeze'}
              </Button>
            </div>

            {freezeFormError && <div className={styles.error}>{freezeFormError}</div>}
            {freezeFormSuccess && <div className={styles.success}>{freezeFormSuccess}</div>}

            {showFreezeForm && (
              <form className={styles.form} onSubmit={handleSubmitFreeze}>
                <div className={styles.formRow}>
                  <div className={styles.formGroup}>
                    <label className={styles.label}>Baseline ID *</label>
                    <input
                      className={styles.input}
                      type="text"
                      placeholder="e.g., v1.0.0"
                      value={freezeBaselineId}
                      onChange={e => setFreezeBaselineId(e.target.value)}
                      required
                    />
                  </div>
                  <div className={styles.formGroup}>
                    <label className={styles.label}>Verification Mode *</label>
                    <select
                      className={styles.select}
                      value={freezeVerificationMode}
                      onChange={e => setFreezeVerificationMode(e.target.value)}
                    >
                      <option value="STRICT">STRICT</option>
                      <option value="WITH_EXCEPTIONS">WITH_EXCEPTIONS</option>
                    </select>
                  </div>
                </div>

                <div className={styles.formRow}>
                  <div className={styles.formGroup}>
                    <label className={styles.label}>Oracle Suite ID *</label>
                    <input
                      className={styles.input}
                      type="text"
                      placeholder="e.g., suite:SR-SUITE-CORE"
                      value={freezeOracleSuiteId}
                      onChange={e => setFreezeOracleSuiteId(e.target.value)}
                      required
                    />
                  </div>
                  <div className={styles.formGroup}>
                    <label className={styles.label}>Oracle Suite Hash</label>
                    <input
                      className={styles.input}
                      type="text"
                      placeholder="sha256:..."
                      value={freezeOracleSuiteHash}
                      onChange={e => setFreezeOracleSuiteHash(e.target.value)}
                    />
                  </div>
                </div>

                <div className={styles.formGroup}>
                  <label className={styles.label}>Release Approval ID *</label>
                  <input
                    className={styles.input}
                    type="text"
                    placeholder="appr_..."
                    value={freezeApprovalId}
                    onChange={e => setFreezeApprovalId(e.target.value)}
                    required
                  />
                </div>

                <div className={styles.formGroup}>
                  <label className={styles.label}>Artifact Bundle Refs (comma-separated)</label>
                  <input
                    className={styles.input}
                    type="text"
                    placeholder="sha256:abc123, sha256:def456"
                    value={freezeEvidenceBundleRefs}
                    onChange={e => setFreezeEvidenceBundleRefs(e.target.value)}
                  />
                </div>

                <div className={styles.buttonRow}>
                  <Button variant="secondary" type="button" onClick={resetFreezeForm}>
                    Cancel
                  </Button>
                  <Button variant="primary" type="submit">
                    Create Freeze Record
                  </Button>
                </div>
              </form>
            )}

            {freezeRecords.length === 0 && !showFreezeForm ? (
              <div className={styles.placeholder}>
                <p className={styles.placeholderText}>No freeze records for this candidate.</p>
                <p className={styles.placeholderHint}>
                  Freeze records are created when a human authority approves a candidate baseline.
                </p>
              </div>
            ) : freezeRecords.length > 0 ? (
              <table className={styles.table}>
                <thead>
                  <tr>
                    <th className={styles.th}>ID</th>
                    <th className={styles.th}>Mode</th>
                    <th className={styles.th}>Artifact Manifest</th>
                    <th className={styles.th}>Created</th>
                  </tr>
                </thead>
                <tbody>
                  {freezeRecords.map(freeze => (
                    <tr key={freeze.id}>
                      <td className={styles.td}>{freeze.id}</td>
                      <td className={styles.td}>
                        <Pill tone={getStatusTone(freeze.verification_mode)}>
                          {freeze.verification_mode}
                        </Pill>
                      </td>
                      <td className={styles.td}>
                        <code className={styles.mono}>
                          {truncateHash(freeze.artifact_manifest_hash, 16)}
                        </code>
                      </td>
                      <td className={styles.td}>
                        {new Date(freeze.created_at).toLocaleString()}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            ) : null}
          </>
        )}
      </Card>
    </div>
  );
}

export default CandidateDetail;

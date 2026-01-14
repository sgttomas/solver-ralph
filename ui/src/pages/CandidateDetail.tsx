/**
 * Candidate Detail Page (D-29)
 *
 * Displays a single candidate with its oracle runs, evidence bundles, and freeze records.
 * Per SR-SPEC, candidates are content-addressed artifacts produced by iterations.
 */

import { useState, useEffect } from 'react';
import { useParams, Link } from 'react-router-dom';
import { useAuth } from 'react-oidc-context';
import config from '../config';

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

const styles = {
  container: {
    maxWidth: '1200px',
    margin: '0 auto',
  },
  breadcrumb: {
    marginBottom: '1rem',
    fontSize: '0.875rem',
  },
  breadcrumbLink: {
    color: '#0066cc',
    textDecoration: 'none',
  },
  header: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'flex-start',
    marginBottom: '1.5rem',
  },
  title: {
    margin: 0,
    fontSize: '1.5rem',
    color: '#1a1a2e',
  },
  subtitle: {
    margin: '0.5rem 0 0 0',
    fontSize: '0.875rem',
    color: '#666',
    fontFamily: 'monospace',
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
  link: {
    color: '#0066cc',
    textDecoration: 'none',
  },
  placeholder: {
    textAlign: 'center' as const,
    padding: '2rem',
    color: '#666',
  },
  infoRow: {
    display: 'flex',
    marginBottom: '0.5rem',
  },
  infoLabel: {
    width: '140px',
    fontSize: '0.75rem',
    color: '#666',
    textTransform: 'uppercase' as const,
  },
  infoValue: {
    flex: 1,
    fontSize: '0.875rem',
  },
  monospace: {
    fontFamily: 'monospace',
    fontSize: '0.75rem',
    backgroundColor: '#f5f5f5',
    padding: '0.25rem 0.5rem',
    borderRadius: '4px',
  },
  refList: {
    listStyle: 'none',
    margin: 0,
    padding: 0,
  },
  refItem: {
    padding: '0.5rem',
    backgroundColor: '#f8f9fa',
    borderRadius: '4px',
    marginBottom: '0.5rem',
    fontFamily: 'monospace',
    fontSize: '0.75rem',
  },
  refRel: {
    color: '#666',
    marginLeft: '0.5rem',
  },
  tabs: {
    display: 'flex',
    borderBottom: '2px solid #e5e5e5',
    marginBottom: '1rem',
  },
  tab: {
    padding: '0.75rem 1rem',
    border: 'none',
    background: 'none',
    cursor: 'pointer',
    fontSize: '0.875rem',
    color: '#666',
    borderBottom: '2px solid transparent',
    marginBottom: '-2px',
  },
  tabActive: {
    color: '#0066cc',
    borderBottomColor: '#0066cc',
  },
};

const stateColors: Record<string, { bg: string; color: string }> = {
  PENDING: { bg: '#fff3cd', color: '#856404' },
  RUNNING: { bg: '#d4edda', color: '#155724' },
  COMPLETED: { bg: '#cce5ff', color: '#004085' },
  SUCCESS: { bg: '#d4edda', color: '#155724' },
  FAILURE: { bg: '#f8d7da', color: '#721c24' },
  VERIFIED: { bg: '#d4edda', color: '#155724' },
  REJECTED: { bg: '#f8d7da', color: '#721c24' },
  PASS: { bg: '#d4edda', color: '#155724' },
  FAIL: { bg: '#f8d7da', color: '#721c24' },
  ERROR: { bg: '#f5c6cb', color: '#721c24' },
  SKIPPED: { bg: '#e2e3e5', color: '#383d41' },
  STRICT: { bg: '#cce5ff', color: '#004085' },
  WITH_EXCEPTIONS: { bg: '#fff3cd', color: '#856404' },
};

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

  useEffect(() => {
    if (!auth.user?.access_token || !candidateId) return;

    const headers = { Authorization: `Bearer ${auth.user.access_token}` };

    Promise.all([
      fetch(`${config.apiUrl}/api/v1/candidates/${candidateId}`, { headers }).then(res => {
        if (!res.ok) throw new Error(`Candidate fetch failed: HTTP ${res.status}`);
        return res.json();
      }),
      fetch(`${config.apiUrl}/api/v1/candidates/${candidateId}/runs`, { headers }).then(res => {
        if (!res.ok) throw new Error(`Runs fetch failed: HTTP ${res.status}`);
        return res.json();
      }),
      fetch(`${config.apiUrl}/api/v1/candidates/${candidateId}/evidence`, { headers }).then(res => {
        if (!res.ok) throw new Error(`Evidence fetch failed: HTTP ${res.status}`);
        return res.json();
      }),
      fetch(`${config.apiUrl}/api/v1/candidates/${candidateId}/freeze-records`, { headers }).then(res => {
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
      <div style={styles.container}>
        <div style={styles.placeholder}>
          <p>Loading candidate details...</p>
        </div>
      </div>
    );
  }

  if (error || !candidate) {
    return (
      <div style={styles.container}>
        <div style={styles.placeholder}>
          <p style={{ color: '#dc3545' }}>Error: {error || 'Candidate not found'}</p>
          <Link to="/loops" style={styles.link}>Back to Loops</Link>
        </div>
      </div>
    );
  }

  const stateStyle = stateColors[candidate.state] || stateColors.PENDING;

  return (
    <div style={styles.container}>
      {/* Breadcrumb */}
      <div style={styles.breadcrumb}>
        <Link to="/loops" style={styles.breadcrumbLink}>Loops</Link>
        {candidate.produced_by_iteration_id && (
          <>
            <span style={{ color: '#666' }}> / </span>
            <Link
              to={`/iterations/${candidate.produced_by_iteration_id}`}
              style={styles.breadcrumbLink}
            >
              Iteration
            </Link>
          </>
        )}
        <span style={{ color: '#666' }}> / </span>
        <span>Candidate</span>
      </div>

      {/* Header */}
      <div style={styles.header}>
        <div>
          <h1 style={styles.title}>Candidate</h1>
          <p style={styles.subtitle}>{candidate.id}</p>
        </div>
        <span
          style={{
            ...styles.statusBadge,
            backgroundColor: stateStyle.bg,
            color: stateStyle.color,
          }}
        >
          {candidate.state}
        </span>
      </div>

      {/* Overview Card */}
      <div style={styles.card}>
        <h2 style={styles.cardTitle}>Overview</h2>
        <div style={styles.infoRow}>
          <span style={styles.infoLabel}>Content Hash</span>
          <code style={styles.monospace}>{candidate.content_hash}</code>
        </div>
        {candidate.git_sha && (
          <div style={styles.infoRow}>
            <span style={styles.infoLabel}>Git SHA</span>
            <code style={styles.monospace}>{candidate.git_sha}</code>
          </div>
        )}
        <div style={styles.infoRow}>
          <span style={styles.infoLabel}>Materialized</span>
          <span style={styles.infoValue}>
            {new Date(candidate.materialized_at).toLocaleString()}
          </span>
        </div>
        {candidate.produced_by_iteration_id && (
          <div style={styles.infoRow}>
            <span style={styles.infoLabel}>Iteration</span>
            <Link
              to={`/iterations/${candidate.produced_by_iteration_id}`}
              style={styles.link}
            >
              {candidate.produced_by_iteration_id}
            </Link>
          </div>
        )}
      </div>

      {/* Refs Card */}
      {candidate.refs && candidate.refs.length > 0 && (
        <div style={styles.card}>
          <h2 style={styles.cardTitle}>References ({candidate.refs.length})</h2>
          <ul style={styles.refList}>
            {candidate.refs.map((ref, idx) => (
              <li key={idx} style={styles.refItem}>
                <span>{ref.type_key}:{ref.id}</span>
                <span style={styles.refRel}>({ref.rel})</span>
              </li>
            ))}
          </ul>
        </div>
      )}

      {/* Tabbed Content Card */}
      <div style={styles.card}>
        <div style={styles.tabs}>
          <button
            style={{ ...styles.tab, ...(activeTab === 'runs' ? styles.tabActive : {}) }}
            onClick={() => setActiveTab('runs')}
          >
            Oracle Runs ({runs.length})
          </button>
          <button
            style={{ ...styles.tab, ...(activeTab === 'evidence' ? styles.tabActive : {}) }}
            onClick={() => setActiveTab('evidence')}
          >
            Evidence ({evidence.length})
          </button>
          <button
            style={{ ...styles.tab, ...(activeTab === 'freeze' ? styles.tabActive : {}) }}
            onClick={() => setActiveTab('freeze')}
          >
            Freeze Records ({freezeRecords.length})
          </button>
        </div>

        {/* Runs Tab */}
        {activeTab === 'runs' && (
          <>
            {runs.length === 0 ? (
              <div style={styles.placeholder}>
                <p>No oracle runs for this candidate.</p>
              </div>
            ) : (
              <table style={styles.table}>
                <thead>
                  <tr>
                    <th style={styles.th}>Run ID</th>
                    <th style={styles.th}>Suite</th>
                    <th style={styles.th}>State</th>
                    <th style={styles.th}>Outcome</th>
                    <th style={styles.th}>Evidence</th>
                    <th style={styles.th}>Started</th>
                  </tr>
                </thead>
                <tbody>
                  {runs.map(run => {
                    const runStateStyle = stateColors[run.state] || stateColors.PENDING;
                    const outcomeStyle = run.outcome
                      ? (stateColors[run.outcome] || stateColors.PENDING)
                      : null;
                    return (
                      <tr key={run.id}>
                        <td style={styles.td}>{run.id}</td>
                        <td style={styles.td}>
                          <code style={styles.monospace}>{run.oracle_suite_id}</code>
                        </td>
                        <td style={styles.td}>
                          <span
                            style={{
                              ...styles.statusBadge,
                              backgroundColor: runStateStyle.bg,
                              color: runStateStyle.color,
                            }}
                          >
                            {run.state}
                          </span>
                        </td>
                        <td style={styles.td}>
                          {outcomeStyle ? (
                            <span
                              style={{
                                ...styles.statusBadge,
                                backgroundColor: outcomeStyle.bg,
                                color: outcomeStyle.color,
                              }}
                            >
                              {run.outcome}
                            </span>
                          ) : (
                            <span style={{ color: '#999' }}>-</span>
                          )}
                        </td>
                        <td style={styles.td}>
                          {run.evidence_bundle_hash ? (
                            <Link
                              to={`/evidence/${run.evidence_bundle_hash}`}
                              style={styles.link}
                            >
                              <code style={styles.monospace}>
                                {run.evidence_bundle_hash.substring(0, 12)}...
                              </code>
                            </Link>
                          ) : (
                            <span style={{ color: '#999' }}>-</span>
                          )}
                        </td>
                        <td style={styles.td}>
                          {new Date(run.started_at).toLocaleString()}
                        </td>
                      </tr>
                    );
                  })}
                </tbody>
              </table>
            )}
          </>
        )}

        {/* Evidence Tab */}
        {activeTab === 'evidence' && (
          <>
            {evidence.length === 0 ? (
              <div style={styles.placeholder}>
                <p>No evidence bundles associated with this candidate.</p>
              </div>
            ) : (
              <table style={styles.table}>
                <thead>
                  <tr>
                    <th style={styles.th}>Content Hash</th>
                    <th style={styles.th}>Type</th>
                    <th style={styles.th}>Suite</th>
                    <th style={styles.th}>Verdict</th>
                    <th style={styles.th}>Created</th>
                  </tr>
                </thead>
                <tbody>
                  {evidence.map(bundle => {
                    const verdictStyle = stateColors[bundle.manifest.verdict] || stateColors.PENDING;
                    return (
                      <tr key={bundle.content_hash}>
                        <td style={styles.td}>
                          <Link
                            to={`/evidence/${bundle.content_hash}`}
                            style={styles.link}
                          >
                            <code style={styles.monospace}>
                              {bundle.content_hash.substring(0, 16)}...
                            </code>
                          </Link>
                        </td>
                        <td style={styles.td}>{bundle.manifest.artifact_type}</td>
                        <td style={styles.td}>
                          <code style={styles.monospace}>{bundle.manifest.suite_id}</code>
                        </td>
                        <td style={styles.td}>
                          <span
                            style={{
                              ...styles.statusBadge,
                              backgroundColor: verdictStyle.bg,
                              color: verdictStyle.color,
                            }}
                          >
                            {bundle.manifest.verdict}
                          </span>
                        </td>
                        <td style={styles.td}>
                          {new Date(bundle.manifest.created_at).toLocaleString()}
                        </td>
                      </tr>
                    );
                  })}
                </tbody>
              </table>
            )}
          </>
        )}

        {/* Freeze Records Tab */}
        {activeTab === 'freeze' && (
          <>
            {freezeRecords.length === 0 ? (
              <div style={styles.placeholder}>
                <p>No freeze records for this candidate.</p>
                <p style={{ fontSize: '0.875rem', color: '#999' }}>
                  Freeze records are created when a human authority approves a candidate baseline.
                </p>
              </div>
            ) : (
              <table style={styles.table}>
                <thead>
                  <tr>
                    <th style={styles.th}>ID</th>
                    <th style={styles.th}>Mode</th>
                    <th style={styles.th}>Artifact Manifest</th>
                    <th style={styles.th}>Created</th>
                  </tr>
                </thead>
                <tbody>
                  {freezeRecords.map(freeze => {
                    const modeStyle = stateColors[freeze.verification_mode] || stateColors.PENDING;
                    return (
                      <tr key={freeze.id}>
                        <td style={styles.td}>{freeze.id}</td>
                        <td style={styles.td}>
                          <span
                            style={{
                              ...styles.statusBadge,
                              backgroundColor: modeStyle.bg,
                              color: modeStyle.color,
                            }}
                          >
                            {freeze.verification_mode}
                          </span>
                        </td>
                        <td style={styles.td}>
                          <code style={styles.monospace}>
                            {freeze.artifact_manifest_hash.substring(0, 16)}...
                          </code>
                        </td>
                        <td style={styles.td}>
                          {new Date(freeze.created_at).toLocaleString()}
                        </td>
                      </tr>
                    );
                  })}
                </tbody>
              </table>
            )}
          </>
        )}
      </div>
    </div>
  );
}

export default CandidateDetail;

/**
 * Iteration Detail Page (D-29)
 *
 * Displays a single iteration with its candidates, context refs, and summary.
 * Per SR-SPEC, iterations are fresh-context execution cycles within a loop.
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

interface IterationSummary {
  rationale: string;
  actions_taken: string[];
  blockers: string[];
  next_steps: string[];
}

interface Iteration {
  id: string;
  loop_id: string;
  sequence_number: number;
  state: string;
  started_at: string;
  completed_at: string | null;
  outcome: string | null;
  refs: TypedRef[];
  summary: IterationSummary | null;
}

interface Candidate {
  id: string;
  content_hash: string;
  git_sha: string | null;
  state: string;
  produced_by_iteration_id: string | null;
  materialized_at: string;
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
  summarySection: {
    marginBottom: '1rem',
  },
  summaryLabel: {
    fontSize: '0.75rem',
    color: '#666',
    textTransform: 'uppercase' as const,
    marginBottom: '0.5rem',
  },
  summaryText: {
    fontSize: '0.875rem',
    lineHeight: 1.6,
    margin: 0,
  },
  bulletList: {
    margin: '0.5rem 0 0 1.5rem',
    padding: 0,
    fontSize: '0.875rem',
  },
};

const stateColors: Record<string, { bg: string; color: string }> = {
  CREATED: { bg: '#fff3cd', color: '#856404' },
  ACTIVE: { bg: '#d4edda', color: '#155724' },
  RUNNING: { bg: '#d4edda', color: '#155724' },
  COMPLETED: { bg: '#cce5ff', color: '#004085' },
  SUCCESS: { bg: '#d4edda', color: '#155724' },
  FAILURE: { bg: '#f8d7da', color: '#721c24' },
  PENDING: { bg: '#fff3cd', color: '#856404' },
  VERIFIED: { bg: '#d4edda', color: '#155724' },
  REJECTED: { bg: '#f8d7da', color: '#721c24' },
};

export function IterationDetail(): JSX.Element {
  const { iterationId } = useParams<{ iterationId: string }>();
  const auth = useAuth();
  const [iteration, setIteration] = useState<Iteration | null>(null);
  const [candidates, setCandidates] = useState<Candidate[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!auth.user?.access_token || !iterationId) return;

    const headers = { Authorization: `Bearer ${auth.user.access_token}` };

    Promise.all([
      fetch(`${config.apiUrl}/api/v1/iterations/${iterationId}`, { headers }).then(res => {
        if (!res.ok) throw new Error(`Iteration fetch failed: HTTP ${res.status}`);
        return res.json();
      }),
      fetch(`${config.apiUrl}/api/v1/iterations/${iterationId}/candidates`, { headers }).then(res => {
        if (!res.ok) throw new Error(`Candidates fetch failed: HTTP ${res.status}`);
        return res.json();
      }),
    ])
      .then(([iterationData, candidatesData]) => {
        setIteration(iterationData);
        setCandidates(candidatesData.candidates || []);
        setLoading(false);
      })
      .catch(err => {
        setError(err.message);
        setLoading(false);
      });
  }, [auth.user?.access_token, iterationId]);

  if (loading) {
    return (
      <div style={styles.container}>
        <div style={styles.placeholder}>
          <p>Loading iteration details...</p>
        </div>
      </div>
    );
  }

  if (error || !iteration) {
    return (
      <div style={styles.container}>
        <div style={styles.placeholder}>
          <p style={{ color: '#dc3545' }}>Error: {error || 'Iteration not found'}</p>
          <Link to="/loops" style={styles.link}>Back to Loops</Link>
        </div>
      </div>
    );
  }

  const stateStyle = stateColors[iteration.state] || stateColors.CREATED;
  const outcomeStyle = iteration.outcome
    ? (stateColors[iteration.outcome] || stateColors.CREATED)
    : null;

  return (
    <div style={styles.container}>
      {/* Breadcrumb */}
      <div style={styles.breadcrumb}>
        <Link to="/loops" style={styles.breadcrumbLink}>Loops</Link>
        <span style={{ color: '#666' }}> / </span>
        <Link to={`/loops/${iteration.loop_id}`} style={styles.breadcrumbLink}>
          {iteration.loop_id}
        </Link>
        <span style={{ color: '#666' }}> / </span>
        <span>Iteration #{iteration.sequence_number}</span>
      </div>

      {/* Header */}
      <div style={styles.header}>
        <div>
          <h1 style={styles.title}>Iteration #{iteration.sequence_number}</h1>
          <p style={styles.subtitle}>{iteration.id}</p>
        </div>
        <div style={{ display: 'flex', gap: '0.5rem' }}>
          <span
            style={{
              ...styles.statusBadge,
              backgroundColor: stateStyle.bg,
              color: stateStyle.color,
            }}
          >
            {iteration.state}
          </span>
          {outcomeStyle && (
            <span
              style={{
                ...styles.statusBadge,
                backgroundColor: outcomeStyle.bg,
                color: outcomeStyle.color,
              }}
            >
              {iteration.outcome}
            </span>
          )}
        </div>
      </div>

      {/* Overview Card */}
      <div style={styles.card}>
        <h2 style={styles.cardTitle}>Overview</h2>
        <div style={styles.infoRow}>
          <span style={styles.infoLabel}>Loop</span>
          <Link to={`/loops/${iteration.loop_id}`} style={styles.link}>
            {iteration.loop_id}
          </Link>
        </div>
        <div style={styles.infoRow}>
          <span style={styles.infoLabel}>Started</span>
          <span style={styles.infoValue}>{new Date(iteration.started_at).toLocaleString()}</span>
        </div>
        {iteration.completed_at && (
          <div style={styles.infoRow}>
            <span style={styles.infoLabel}>Completed</span>
            <span style={styles.infoValue}>
              {new Date(iteration.completed_at).toLocaleString()}
            </span>
          </div>
        )}
      </div>

      {/* Context Refs Card */}
      {iteration.refs && iteration.refs.length > 0 && (
        <div style={styles.card}>
          <h2 style={styles.cardTitle}>Context References ({iteration.refs.length})</h2>
          <ul style={styles.refList}>
            {iteration.refs.map((ref, idx) => (
              <li key={idx} style={styles.refItem}>
                <span>{ref.type_key}:{ref.id}</span>
                <span style={styles.refRel}>({ref.rel})</span>
              </li>
            ))}
          </ul>
        </div>
      )}

      {/* Summary Card */}
      {iteration.summary && (
        <div style={styles.card}>
          <h2 style={styles.cardTitle}>Iteration Summary</h2>

          <div style={styles.summarySection}>
            <div style={styles.summaryLabel}>Rationale</div>
            <p style={styles.summaryText}>{iteration.summary.rationale}</p>
          </div>

          {iteration.summary.actions_taken.length > 0 && (
            <div style={styles.summarySection}>
              <div style={styles.summaryLabel}>Actions Taken</div>
              <ul style={styles.bulletList}>
                {iteration.summary.actions_taken.map((action, idx) => (
                  <li key={idx}>{action}</li>
                ))}
              </ul>
            </div>
          )}

          {iteration.summary.blockers.length > 0 && (
            <div style={styles.summarySection}>
              <div style={styles.summaryLabel}>Blockers</div>
              <ul style={styles.bulletList}>
                {iteration.summary.blockers.map((blocker, idx) => (
                  <li key={idx}>{blocker}</li>
                ))}
              </ul>
            </div>
          )}

          {iteration.summary.next_steps.length > 0 && (
            <div style={styles.summarySection}>
              <div style={styles.summaryLabel}>Next Steps</div>
              <ul style={styles.bulletList}>
                {iteration.summary.next_steps.map((step, idx) => (
                  <li key={idx}>{step}</li>
                ))}
              </ul>
            </div>
          )}
        </div>
      )}

      {/* Candidates Card */}
      <div style={styles.card}>
        <h2 style={styles.cardTitle}>Candidates ({candidates.length})</h2>
        {candidates.length === 0 ? (
          <div style={styles.placeholder}>
            <p>No candidates produced in this iteration yet.</p>
          </div>
        ) : (
          <table style={styles.table}>
            <thead>
              <tr>
                <th style={styles.th}>ID</th>
                <th style={styles.th}>Content Hash</th>
                <th style={styles.th}>Git SHA</th>
                <th style={styles.th}>State</th>
                <th style={styles.th}>Materialized</th>
              </tr>
            </thead>
            <tbody>
              {candidates.map(candidate => {
                const candStateStyle = stateColors[candidate.state] || stateColors.PENDING;
                return (
                  <tr key={candidate.id}>
                    <td style={styles.td}>
                      <Link to={`/candidates/${candidate.id}`} style={styles.link}>
                        {candidate.id}
                      </Link>
                    </td>
                    <td style={styles.td}>
                      <code style={styles.monospace}>
                        {candidate.content_hash.substring(0, 16)}...
                      </code>
                    </td>
                    <td style={styles.td}>
                      {candidate.git_sha ? (
                        <code style={styles.monospace}>
                          {candidate.git_sha.substring(0, 8)}
                        </code>
                      ) : (
                        <span style={{ color: '#999' }}>-</span>
                      )}
                    </td>
                    <td style={styles.td}>
                      <span
                        style={{
                          ...styles.statusBadge,
                          backgroundColor: candStateStyle.bg,
                          color: candStateStyle.color,
                        }}
                      >
                        {candidate.state}
                      </span>
                    </td>
                    <td style={styles.td}>
                      {new Date(candidate.materialized_at).toLocaleString()}
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        )}
      </div>
    </div>
  );
}

export default IterationDetail;

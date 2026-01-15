/**
 * Iteration Detail Page (D-29)
 *
 * Displays a single iteration with its candidates, context refs, and summary.
 * Per SR-SPEC, iterations are fresh-context execution cycles within a loop.
 */

import { useState, useEffect } from 'react';
import { useParams, Link } from 'react-router-dom';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Pill, getStatusTone, truncateHash } from '../ui';
import styles from '../styles/pages.module.css';

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
      <div className={styles.container}>
        <div className={styles.placeholder}>
          <p className={styles.placeholderText}>Loading iteration details...</p>
        </div>
      </div>
    );
  }

  if (error || !iteration) {
    return (
      <div className={styles.container}>
        <div className={styles.placeholder}>
          <p className={styles.error}>Error: {error || 'Iteration not found'}</p>
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
        <span className={styles.breadcrumbSeparator}>/</span>
        <Link to={`/loops/${iteration.loop_id}`} className={styles.breadcrumbLink}>
          {iteration.loop_id}
        </Link>
        <span className={styles.breadcrumbSeparator}>/</span>
        <span>Iteration #{iteration.sequence_number}</span>
      </div>

      {/* Header */}
      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>Iteration #{iteration.sequence_number}</h1>
          <p className={styles.subtitle}>{iteration.id}</p>
        </div>
        <div className={styles.badgeGroup}>
          <Pill tone={getStatusTone(iteration.state)}>{iteration.state}</Pill>
          {iteration.outcome && (
            <Pill tone={getStatusTone(iteration.outcome)}>{iteration.outcome}</Pill>
          )}
        </div>
      </div>

      {/* Overview Card */}
      <Card title="Overview" className={styles.cardSpacing}>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Workflow</span>
          <Link to={`/loops/${iteration.loop_id}`} className={styles.link}>
            {iteration.loop_id}
          </Link>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Started</span>
          <span className={styles.infoValue}>{new Date(iteration.started_at).toLocaleString()}</span>
        </div>
        {iteration.completed_at && (
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Completed</span>
            <span className={styles.infoValue}>
              {new Date(iteration.completed_at).toLocaleString()}
            </span>
          </div>
        )}
      </Card>

      {/* Context Refs Card */}
      {iteration.refs && iteration.refs.length > 0 && (
        <Card title={`Context References (${iteration.refs.length})`} className={styles.cardSpacing}>
          <ul className={styles.refList}>
            {iteration.refs.map((ref, idx) => (
              <li key={idx} className={styles.refItem}>
                <span>{ref.type_key}:{ref.id}</span>
                <span className={styles.refRel}>({ref.rel})</span>
              </li>
            ))}
          </ul>
        </Card>
      )}

      {/* Summary Card */}
      {iteration.summary && (
        <Card title="Iteration Summary" className={styles.cardSpacing}>
          <div className={styles.summarySection}>
            <div className={styles.summaryLabel}>Rationale</div>
            <p className={styles.summaryText}>{iteration.summary.rationale}</p>
          </div>

          {iteration.summary.actions_taken.length > 0 && (
            <div className={styles.summarySection}>
              <div className={styles.summaryLabel}>Actions Taken</div>
              <ul className={styles.bulletList}>
                {iteration.summary.actions_taken.map((action, idx) => (
                  <li key={idx}>{action}</li>
                ))}
              </ul>
            </div>
          )}

          {iteration.summary.blockers.length > 0 && (
            <div className={styles.summarySection}>
              <div className={styles.summaryLabel}>Blockers</div>
              <ul className={styles.bulletList}>
                {iteration.summary.blockers.map((blocker, idx) => (
                  <li key={idx}>{blocker}</li>
                ))}
              </ul>
            </div>
          )}

          {iteration.summary.next_steps.length > 0 && (
            <div className={styles.summarySection}>
              <div className={styles.summaryLabel}>Next Steps</div>
              <ul className={styles.bulletList}>
                {iteration.summary.next_steps.map((step, idx) => (
                  <li key={idx}>{step}</li>
                ))}
              </ul>
            </div>
          )}
        </Card>
      )}

      {/* Candidates Card */}
      <Card title={`Candidates (${candidates.length})`}>
        {candidates.length === 0 ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>No candidates produced in this iteration yet.</p>
          </div>
        ) : (
          <table className={styles.table}>
            <thead>
              <tr>
                <th className={styles.th}>ID</th>
                <th className={styles.th}>Content Hash</th>
                <th className={styles.th}>Git SHA</th>
                <th className={styles.th}>State</th>
                <th className={styles.th}>Materialized</th>
              </tr>
            </thead>
            <tbody>
              {candidates.map(candidate => (
                <tr key={candidate.id}>
                  <td className={styles.td}>
                    <Link to={`/candidates/${candidate.id}`} className={styles.link}>
                      {candidate.id}
                    </Link>
                  </td>
                  <td className={styles.td}>
                    <code className={styles.mono}>
                      {truncateHash(candidate.content_hash, 16)}
                    </code>
                  </td>
                  <td className={styles.td}>
                    {candidate.git_sha ? (
                      <code className={styles.mono}>
                        {candidate.git_sha.substring(0, 8)}
                      </code>
                    ) : (
                      <span style={{ color: 'var(--muted)' }}>-</span>
                    )}
                  </td>
                  <td className={styles.td}>
                    <Pill tone={getStatusTone(candidate.state)}>{candidate.state}</Pill>
                  </td>
                  <td className={styles.td}>
                    {new Date(candidate.materialized_at).toLocaleString()}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </Card>
    </div>
  );
}

export default IterationDetail;

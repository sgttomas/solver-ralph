/**
 * Loop Detail Page (D-29)
 *
 * Displays a single loop with its iterations, status, and metadata.
 * Per SR-SPEC, loops are bounded work units containing iterations.
 */

import { useState, useEffect } from 'react';
import { useParams, Link } from 'react-router-dom';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Pill, getStatusTone } from '../ui';
import styles from '../styles/pages.module.css';

interface TypedRef {
  type_key: string;
  id: string;
  rel: string;
}

interface Loop {
  id: string;
  goal: string;
  work_unit: string | null;
  state: string;
  created_at: string;
  directive_ref: TypedRef;
  budgets: {
    max_iterations: number;
    max_oracle_runs: number;
    max_wallclock_hours: number;
  };
  iteration_count: number;
  oracle_run_count: number;
}

interface Iteration {
  id: string;
  loop_id: string;
  sequence_number: number;
  state: string;
  started_at: string;
  completed_at: string | null;
  outcome: string | null;
}

export function LoopDetail(): JSX.Element {
  const { loopId } = useParams<{ loopId: string }>();
  const auth = useAuth();
  const [loop, setLoop] = useState<Loop | null>(null);
  const [iterations, setIterations] = useState<Iteration[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!auth.user?.access_token || !loopId) return;

    const headers = { Authorization: `Bearer ${auth.user.access_token}` };

    Promise.all([
      fetch(`${config.apiUrl}/api/v1/loops/${loopId}`, { headers }).then(res => {
        if (!res.ok) throw new Error(`Loop fetch failed: HTTP ${res.status}`);
        return res.json();
      }),
      fetch(`${config.apiUrl}/api/v1/loops/${loopId}/iterations`, { headers }).then(res => {
        if (!res.ok) throw new Error(`Iterations fetch failed: HTTP ${res.status}`);
        return res.json();
      }),
    ])
      .then(([loopData, iterationsData]) => {
        setLoop(loopData);
        setIterations(iterationsData.iterations || []);
        setLoading(false);
      })
      .catch(err => {
        setError(err.message);
        setLoading(false);
      });
  }, [auth.user?.access_token, loopId]);

  if (loading) {
    return (
      <div className={styles.container}>
        <div className={styles.placeholder}>
          <p className={styles.placeholderText}>Loading workflow details...</p>
        </div>
      </div>
    );
  }

  if (error || !loop) {
    return (
      <div className={styles.container}>
        <div className={styles.placeholder}>
          <p className={styles.error}>Error: {error || 'Workflow not found'}</p>
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
        <span>{loop.id}</span>
      </div>

      {/* Header */}
      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>{loop.work_unit || loop.goal}</h1>
          <p className={styles.subtitle}>{loop.id}</p>
        </div>
        <Pill tone={getStatusTone(loop.state)}>{loop.state}</Pill>
      </div>

      {/* Overview Card */}
      <Card title="Overview" className={styles.cardSpacing}>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Goal</span>
          <span className={styles.infoValue}>{loop.goal}</span>
        </div>
        {loop.work_unit && (
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Work Unit</span>
            <span className={styles.infoValue}>{loop.work_unit}</span>
          </div>
        )}
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Created</span>
          <span className={styles.infoValue}>{new Date(loop.created_at).toLocaleString()}</span>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Directive Ref</span>
          <code className={styles.mono}>
            {loop.directive_ref.type_key}:{loop.directive_ref.id}
          </code>
        </div>
      </Card>

      {/* Budgets Card */}
      <Card title="Budgets & Usage" className={styles.cardSpacing}>
        <div className={styles.statsGrid}>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Iterations</div>
            <div className={styles.statValue}>
              {loop.iteration_count} / {loop.budgets.max_iterations}
            </div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Oracle Runs</div>
            <div className={styles.statValue}>
              {loop.oracle_run_count} / {loop.budgets.max_oracle_runs}
            </div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Max Hours</div>
            <div className={styles.statValue}>
              {loop.budgets.max_wallclock_hours}h
            </div>
          </div>
        </div>
      </Card>

      {/* Iterations Card */}
      <Card title={`Iterations (${iterations.length})`}>
        {iterations.length === 0 ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>No iterations yet.</p>
            <p className={styles.placeholderHint}>
              Iterations are started by the SYSTEM actor per SR-SPEC.
            </p>
          </div>
        ) : (
          <table className={styles.table}>
            <thead>
              <tr>
                <th className={styles.th}>#</th>
                <th className={styles.th}>ID</th>
                <th className={styles.th}>State</th>
                <th className={styles.th}>Outcome</th>
                <th className={styles.th}>Started</th>
                <th className={styles.th}>Completed</th>
              </tr>
            </thead>
            <tbody>
              {iterations.map(iteration => (
                <tr key={iteration.id}>
                  <td className={styles.td}>{iteration.sequence_number}</td>
                  <td className={styles.td}>
                    <Link to={`/iterations/${iteration.id}`} className={styles.link}>
                      {iteration.id}
                    </Link>
                  </td>
                  <td className={styles.td}>
                    <Pill tone={getStatusTone(iteration.state)}>{iteration.state}</Pill>
                  </td>
                  <td className={styles.td}>
                    {iteration.outcome ? (
                      <Pill tone={getStatusTone(iteration.outcome)}>{iteration.outcome}</Pill>
                    ) : (
                      <span style={{ color: 'var(--muted)' }}>-</span>
                    )}
                  </td>
                  <td className={styles.td}>{new Date(iteration.started_at).toLocaleString()}</td>
                  <td className={styles.td}>
                    {iteration.completed_at
                      ? new Date(iteration.completed_at).toLocaleString()
                      : '-'}
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

export default LoopDetail;

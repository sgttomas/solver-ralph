/**
 * Loop Detail Page (D-29)
 *
 * Displays a single loop with its iterations, status, and metadata.
 * Per SR-SPEC, loops are bounded work units containing iterations.
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
  grid: {
    display: 'grid',
    gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))',
    gap: '1rem',
  },
  stat: {
    padding: '1rem',
    backgroundColor: '#f8f9fa',
    borderRadius: '4px',
  },
  statLabel: {
    fontSize: '0.75rem',
    color: '#666',
    textTransform: 'uppercase' as const,
    marginBottom: '0.25rem',
  },
  statValue: {
    fontSize: '1.25rem',
    fontWeight: 600,
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
};

const stateColors: Record<string, { bg: string; color: string }> = {
  CREATED: { bg: '#fff3cd', color: '#856404' },
  ACTIVE: { bg: '#d4edda', color: '#155724' },
  PAUSED: { bg: '#e2e3e5', color: '#383d41' },
  CLOSED: { bg: '#f8d7da', color: '#721c24' },
  RUNNING: { bg: '#d4edda', color: '#155724' },
  COMPLETED: { bg: '#cce5ff', color: '#004085' },
  SUCCESS: { bg: '#d4edda', color: '#155724' },
  FAILURE: { bg: '#f8d7da', color: '#721c24' },
};

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
      <div style={styles.container}>
        <div style={styles.placeholder}>
          <p>Loading loop details...</p>
        </div>
      </div>
    );
  }

  if (error || !loop) {
    return (
      <div style={styles.container}>
        <div style={styles.placeholder}>
          <p style={{ color: '#dc3545' }}>Error: {error || 'Loop not found'}</p>
          <Link to="/loops" style={styles.link}>Back to Loops</Link>
        </div>
      </div>
    );
  }

  const stateStyle = stateColors[loop.state] || stateColors.CREATED;

  return (
    <div style={styles.container}>
      {/* Breadcrumb */}
      <div style={styles.breadcrumb}>
        <Link to="/loops" style={styles.breadcrumbLink}>Loops</Link>
        <span style={{ color: '#666' }}> / </span>
        <span>{loop.id}</span>
      </div>

      {/* Header */}
      <div style={styles.header}>
        <div>
          <h1 style={styles.title}>
            {loop.work_unit || loop.goal}
          </h1>
          <p style={styles.subtitle}>{loop.id}</p>
        </div>
        <span
          style={{
            ...styles.statusBadge,
            backgroundColor: stateStyle.bg,
            color: stateStyle.color,
          }}
        >
          {loop.state}
        </span>
      </div>

      {/* Overview Card */}
      <div style={styles.card}>
        <h2 style={styles.cardTitle}>Overview</h2>
        <div style={styles.infoRow}>
          <span style={styles.infoLabel}>Goal</span>
          <span style={styles.infoValue}>{loop.goal}</span>
        </div>
        {loop.work_unit && (
          <div style={styles.infoRow}>
            <span style={styles.infoLabel}>Work Unit</span>
            <span style={styles.infoValue}>{loop.work_unit}</span>
          </div>
        )}
        <div style={styles.infoRow}>
          <span style={styles.infoLabel}>Created</span>
          <span style={styles.infoValue}>{new Date(loop.created_at).toLocaleString()}</span>
        </div>
        <div style={styles.infoRow}>
          <span style={styles.infoLabel}>Directive Ref</span>
          <code style={styles.monospace}>
            {loop.directive_ref.type_key}:{loop.directive_ref.id}
          </code>
        </div>
      </div>

      {/* Budgets Card */}
      <div style={styles.card}>
        <h2 style={styles.cardTitle}>Budgets &amp; Usage</h2>
        <div style={styles.grid}>
          <div style={styles.stat}>
            <div style={styles.statLabel}>Iterations</div>
            <div style={styles.statValue}>
              {loop.iteration_count} / {loop.budgets.max_iterations}
            </div>
          </div>
          <div style={styles.stat}>
            <div style={styles.statLabel}>Oracle Runs</div>
            <div style={styles.statValue}>
              {loop.oracle_run_count} / {loop.budgets.max_oracle_runs}
            </div>
          </div>
          <div style={styles.stat}>
            <div style={styles.statLabel}>Max Hours</div>
            <div style={styles.statValue}>
              {loop.budgets.max_wallclock_hours}h
            </div>
          </div>
        </div>
      </div>

      {/* Iterations Card */}
      <div style={styles.card}>
        <h2 style={styles.cardTitle}>Iterations ({iterations.length})</h2>
        {iterations.length === 0 ? (
          <div style={styles.placeholder}>
            <p>No iterations yet.</p>
            <p style={{ fontSize: '0.875rem', color: '#999' }}>
              Iterations are started by the SYSTEM actor per SR-SPEC.
            </p>
          </div>
        ) : (
          <table style={styles.table}>
            <thead>
              <tr>
                <th style={styles.th}>#</th>
                <th style={styles.th}>ID</th>
                <th style={styles.th}>State</th>
                <th style={styles.th}>Outcome</th>
                <th style={styles.th}>Started</th>
                <th style={styles.th}>Completed</th>
              </tr>
            </thead>
            <tbody>
              {iterations.map(iteration => {
                const iterStateStyle = stateColors[iteration.state] || stateColors.CREATED;
                const outcomeStyle = iteration.outcome
                  ? (stateColors[iteration.outcome] || stateColors.CREATED)
                  : { bg: 'transparent', color: '#999' };
                return (
                  <tr key={iteration.id}>
                    <td style={styles.td}>{iteration.sequence_number}</td>
                    <td style={styles.td}>
                      <Link to={`/iterations/${iteration.id}`} style={styles.link}>
                        {iteration.id}
                      </Link>
                    </td>
                    <td style={styles.td}>
                      <span
                        style={{
                          ...styles.statusBadge,
                          backgroundColor: iterStateStyle.bg,
                          color: iterStateStyle.color,
                        }}
                      >
                        {iteration.state}
                      </span>
                    </td>
                    <td style={styles.td}>
                      {iteration.outcome ? (
                        <span
                          style={{
                            ...styles.statusBadge,
                            backgroundColor: outcomeStyle.bg,
                            color: outcomeStyle.color,
                          }}
                        >
                          {iteration.outcome}
                        </span>
                      ) : (
                        <span style={{ color: '#999' }}>-</span>
                      )}
                    </td>
                    <td style={styles.td}>{new Date(iteration.started_at).toLocaleString()}</td>
                    <td style={styles.td}>
                      {iteration.completed_at
                        ? new Date(iteration.completed_at).toLocaleString()
                        : '-'}
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

export default LoopDetail;

/**
 * Workflows Page
 *
 * Displays workflow definitions and templates.
 * Workflows are higher-level orchestrations that may contain multiple loops.
 */

import { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Pill, getStatusTone } from '../ui';
import styles from '../styles/pages.module.css';

interface Workflow {
  id: string;
  name: string;
  description: string | null;
  status: 'active' | 'draft' | 'archived';
  loops_count: number;
  created_at: string;
  updated_at: string;
}

interface WorkflowsResponse {
  workflows: Workflow[];
}

export function Workflows(): JSX.Element {
  const auth = useAuth();
  const [workflows, setWorkflows] = useState<Workflow[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!auth.user?.access_token) return;

    fetch(`${config.apiUrl}/api/v1/workflows`, {
      headers: {
        Authorization: `Bearer ${auth.user.access_token}`,
      },
    })
      .then(res => {
        // Treat 404 as "no data yet" rather than an error
        if (res.status === 404) {
          return { workflows: [] };
        }
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .then((data: WorkflowsResponse) => {
        setWorkflows(data.workflows || []);
        setLoading(false);
      })
      .catch(err => {
        setError(err.message);
        setLoading(false);
      });
  }, [auth.user?.access_token]);

  const getWorkflowStatusTone = (status: string) => {
    switch (status) {
      case 'active':
        return 'success';
      case 'draft':
        return 'warning';
      case 'archived':
        return 'neutral';
      default:
        return getStatusTone(status);
    }
  };

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>Workflows</h1>
          <p className={styles.subtitle}>Orchestrated sequences of loops and procedures</p>
        </div>
      </div>

      {/* Overview Stats */}
      <Card>
        <div className={styles.statsGrid}>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Total Workflows</div>
            <div className={styles.statValue}>{workflows.length}</div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Active</div>
            <div className={styles.statValue}>
              {workflows.filter(w => w.status === 'active').length}
            </div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Draft</div>
            <div className={styles.statValue}>
              {workflows.filter(w => w.status === 'draft').length}
            </div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Archived</div>
            <div className={styles.statValue}>
              {workflows.filter(w => w.status === 'archived').length}
            </div>
          </div>
        </div>
      </Card>

      {/* Info Note */}
      <div className={styles.note}>
        Workflows define higher-level orchestrations that coordinate multiple loops,
        procedure templates, and governance checkpoints. Each workflow may spawn
        one or more loops to accomplish its objectives.
      </div>

      {/* Workflows Table */}
      <Card>
        {loading ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>Loading workflows...</p>
          </div>
        ) : error ? (
          <div className={styles.placeholder}>
            <p className={styles.error}>Error: {error}</p>
          </div>
        ) : workflows.length === 0 ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>No workflows defined.</p>
            <p className={styles.placeholderHint}>
              Workflows orchestrate multiple loops and procedures.
              They are defined via workflow definition artifacts.
            </p>
          </div>
        ) : (
          <table className={styles.table}>
            <thead>
              <tr>
                <th className={styles.th}>Name</th>
                <th className={styles.th}>Description</th>
                <th className={styles.th}>Status</th>
                <th className={styles.th}>Loops</th>
                <th className={styles.th}>Updated</th>
              </tr>
            </thead>
            <tbody>
              {workflows.map(workflow => (
                <tr key={workflow.id}>
                  <td className={styles.td}>
                    <Link to={`/workflows/${workflow.id}`} className={styles.link}>
                      {workflow.name}
                    </Link>
                    <div style={{ fontSize: '0.75rem', color: 'var(--muted)' }}>
                      {workflow.id}
                    </div>
                  </td>
                  <td className={styles.td}>
                    {workflow.description || <span style={{ color: 'var(--muted)' }}>—</span>}
                  </td>
                  <td className={styles.td}>
                    <Pill tone={getWorkflowStatusTone(workflow.status)}>
                      {workflow.status}
                    </Pill>
                  </td>
                  <td className={styles.td}>{workflow.loops_count}</td>
                  <td className={styles.td}>
                    {new Date(workflow.updated_at).toLocaleDateString()}
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

export default Workflows;

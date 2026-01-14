/**
 * Loops Page (D-29)
 *
 * Displays active work units (loops) and their status.
 * Links to individual loop detail views with iterations.
 */

import { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { useAuth } from 'react-oidc-context';
import config from '../config';

interface Loop {
  id: string;
  name: string;
  status: string;
  created_at: string;
}

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
  card: {
    backgroundColor: 'white',
    borderRadius: '8px',
    padding: '1.5rem',
    boxShadow: '0 1px 3px rgba(0, 0, 0, 0.1)',
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
  link: {
    color: '#0066cc',
    textDecoration: 'none',
  },
};

const statusColors: Record<string, { bg: string; color: string }> = {
  Active: { bg: '#d4edda', color: '#155724' },
  Pending: { bg: '#fff3cd', color: '#856404' },
  Paused: { bg: '#e2e3e5', color: '#383d41' },
  Completed: { bg: '#cce5ff', color: '#004085' },
  Closed: { bg: '#f8d7da', color: '#721c24' },
};

export function Loops(): JSX.Element {
  const auth = useAuth();
  const [loops, setLoops] = useState<Loop[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!auth.user?.access_token) return;

    fetch(`${config.apiUrl}/api/v1/loops`, {
      headers: {
        Authorization: `Bearer ${auth.user.access_token}`,
      },
    })
      .then(res => {
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .then(data => {
        setLoops(data.loops || []);
        setLoading(false);
      })
      .catch(err => {
        setError(err.message);
        setLoading(false);
      });
  }, [auth.user?.access_token]);

  return (
    <div style={styles.container}>
      <div style={styles.header}>
        <h1 style={styles.title}>Loops</h1>
      </div>

      <div style={styles.card}>
        {loading ? (
          <div style={styles.placeholder}>
            <p>Loading loops...</p>
          </div>
        ) : error ? (
          <div style={styles.placeholder}>
            <p style={{ color: '#dc3545' }}>Error: {error}</p>
          </div>
        ) : loops.length === 0 ? (
          <div style={styles.placeholder}>
            <p>No loops found.</p>
            <p style={{ fontSize: '0.875rem', color: '#999' }}>
              Loops represent bounded work units. Create one to get started.
            </p>
          </div>
        ) : (
          <table style={styles.table}>
            <thead>
              <tr>
                <th style={styles.th}>ID</th>
                <th style={styles.th}>Name</th>
                <th style={styles.th}>Status</th>
                <th style={styles.th}>Created</th>
              </tr>
            </thead>
            <tbody>
              {loops.map(loop => {
                const statusStyle = statusColors[loop.status] || statusColors.Pending;
                return (
                  <tr key={loop.id}>
                    <td style={styles.td}>
                      <Link to={`/loops/${loop.id}`} style={styles.link}>
                        {loop.id}
                      </Link>
                    </td>
                    <td style={styles.td}>{loop.name}</td>
                    <td style={styles.td}>
                      <span
                        style={{
                          ...styles.statusBadge,
                          backgroundColor: statusStyle.bg,
                          color: statusStyle.color,
                        }}
                      >
                        {loop.status}
                      </span>
                    </td>
                    <td style={styles.td}>{new Date(loop.created_at).toLocaleString()}</td>
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

export default Loops;

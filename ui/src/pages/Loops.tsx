/**
 * Loops Page (D-29)
 *
 * Displays active work units (loops) and their status.
 * Links to individual loop detail views with iterations.
 */

import { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Pill, getStatusTone } from '../ui';
import styles from '../styles/pages.module.css';

interface Loop {
  id: string;
  name: string;
  status: string;
  created_at: string;
}

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
    <div className={styles.container}>
      <div className={styles.header}>
        <h1 className={styles.title}>Workflows</h1>
      </div>

      <Card>
        {loading ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>Loading workflows...</p>
          </div>
        ) : error ? (
          <div className={styles.placeholder}>
            <p className={styles.error}>Error: {error}</p>
          </div>
        ) : loops.length === 0 ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>No workflows found.</p>
            <p className={styles.placeholderHint}>
              Workflows represent bounded work units. Create one to get started.
            </p>
          </div>
        ) : (
          <table className={styles.table}>
            <thead>
              <tr>
                <th className={styles.th}>ID</th>
                <th className={styles.th}>Name</th>
                <th className={styles.th}>Status</th>
                <th className={styles.th}>Created</th>
              </tr>
            </thead>
            <tbody>
              {loops.map(loop => (
                <tr key={loop.id}>
                  <td className={styles.td}>
                    <Link to={`/loops/${loop.id}`} className={styles.link}>
                      {loop.id}
                    </Link>
                  </td>
                  <td className={styles.td}>{loop.name}</td>
                  <td className={styles.td}>
                    <Pill tone={getStatusTone(loop.status)}>{loop.status}</Pill>
                  </td>
                  <td className={styles.td}>{new Date(loop.created_at).toLocaleString()}</td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </Card>
    </div>
  );
}

export default Loops;

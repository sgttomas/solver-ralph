/**
 * Evidence Page (D-29)
 *
 * Displays evidence bundles (oracle outputs and verification artifacts).
 * Links to individual evidence detail views with manifest inspection.
 */

import { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';

interface EvidenceBundle {
  content_hash: string;
  manifest: {
    artifact_type: string;
    created_at: string;
  };
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
  hash: {
    fontFamily: 'monospace',
    fontSize: '0.75rem',
    backgroundColor: '#f5f5f5',
    padding: '0.25rem 0.5rem',
    borderRadius: '4px',
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

export function Evidence(): JSX.Element {
  const auth = useAuth();
  const [evidence, setEvidence] = useState<EvidenceBundle[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!auth.user?.access_token) return;

    fetch(`${config.apiUrl}/api/v1/evidence`, {
      headers: {
        Authorization: `Bearer ${auth.user.access_token}`,
      },
    })
      .then(res => {
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .then(data => {
        setEvidence(data.evidence || []);
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
        <h1 style={styles.title}>Evidence</h1>
      </div>

      <div style={styles.card}>
        {loading ? (
          <div style={styles.placeholder}>
            <p>Loading evidence bundles...</p>
          </div>
        ) : error ? (
          <div style={styles.placeholder}>
            <p style={{ color: '#dc3545' }}>Error: {error}</p>
          </div>
        ) : evidence.length === 0 ? (
          <div style={styles.placeholder}>
            <p>No evidence bundles found.</p>
            <p style={{ fontSize: '0.875rem', color: '#999' }}>
              Evidence bundles are produced by oracle runs. They contain
              verification artifacts and structured measurements.
            </p>
          </div>
        ) : (
          <table style={styles.table}>
            <thead>
              <tr>
                <th style={styles.th}>Content Hash</th>
                <th style={styles.th}>Type</th>
                <th style={styles.th}>Created</th>
              </tr>
            </thead>
            <tbody>
              {evidence.map(bundle => (
                <tr key={bundle.content_hash}>
                  <td style={styles.td}>
                    <Link to={`/evidence/${bundle.content_hash}`} style={styles.link}>
                      <code style={styles.hash}>
                        {bundle.content_hash.substring(0, 16)}...
                      </code>
                    </Link>
                  </td>
                  <td style={styles.td}>{bundle.manifest.artifact_type}</td>
                  <td style={styles.td}>
                    {new Date(bundle.manifest.created_at).toLocaleString()}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>
    </div>
  );
}

export default Evidence;

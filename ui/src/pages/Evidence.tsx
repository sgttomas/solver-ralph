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
import { Card, truncateHash } from '../ui';
import styles from '../styles/pages.module.css';

interface EvidenceBundle {
  content_hash: string;
  manifest?: {
    artifact_type?: string;
    created_at?: string;
  };
}

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
    <div className={styles.container}>
      <div className={styles.header}>
        <h1 className={styles.title}>Artifacts</h1>
      </div>

      <Card>
        {loading ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>Loading artifact bundles...</p>
          </div>
        ) : error ? (
          <div className={styles.placeholder}>
            <p className={styles.error}>Error: {error}</p>
          </div>
        ) : evidence.length === 0 ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>No artifact bundles found.</p>
            <p className={styles.placeholderHint}>
              Artifact bundles are produced by oracle runs. They contain
              verification outputs and structured measurements.
            </p>
          </div>
        ) : (
          <table className={styles.table}>
            <thead>
              <tr>
                <th className={styles.th}>Content Hash</th>
                <th className={styles.th}>Type</th>
                <th className={styles.th}>Created</th>
              </tr>
            </thead>
            <tbody>
              {evidence.map(bundle => (
                <tr key={bundle.content_hash}>
                  <td className={styles.td}>
                    <Link to={`/artifacts/${bundle.content_hash}`} className={styles.link}>
                      <code className={styles.mono}>
                        {truncateHash(bundle.content_hash, 16)}
                      </code>
                    </Link>
                  </td>
                  <td className={styles.td}>{bundle.manifest?.artifact_type || '—'}</td>
                  <td className={styles.td}>
                    {bundle.manifest?.created_at
                      ? new Date(bundle.manifest.created_at).toLocaleString()
                      : '—'}
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

export default Evidence;

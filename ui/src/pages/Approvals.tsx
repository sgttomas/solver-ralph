/**
 * Approvals Page (D-28 scaffold)
 *
 * Placeholder for Portal workflows UI (D-30).
 * Will display approvals, exceptions, and decisions.
 */

import { useState, useEffect } from 'react';
import { useAuth } from 'react-oidc-context';
import config from '../config';

interface Approval {
  id: string;
  portal_id: string;
  decision: string;
  rationale: string;
  actor_id: string;
  recorded_at: string;
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
  tabs: {
    display: 'flex',
    gap: '0.5rem',
    marginBottom: '1.5rem',
  },
  tab: {
    padding: '0.5rem 1rem',
    border: 'none',
    borderRadius: '4px',
    cursor: 'pointer',
    fontSize: '0.875rem',
    transition: 'background-color 0.2s',
  },
  tabActive: {
    backgroundColor: '#1a1a2e',
    color: 'white',
  },
  tabInactive: {
    backgroundColor: '#e5e5e5',
    color: '#333',
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
  approved: {
    backgroundColor: '#d4edda',
    color: '#155724',
  },
  denied: {
    backgroundColor: '#f8d7da',
    color: '#721c24',
  },
  placeholder: {
    textAlign: 'center' as const,
    padding: '3rem',
    color: '#666',
  },
  note: {
    backgroundColor: '#fff3cd',
    padding: '1rem',
    borderRadius: '4px',
    marginBottom: '1.5rem',
    fontSize: '0.875rem',
    color: '#856404',
  },
};

type TabType = 'approvals' | 'exceptions' | 'decisions';

export function Approvals(): JSX.Element {
  const auth = useAuth();
  const [activeTab, setActiveTab] = useState<TabType>('approvals');
  const [approvals, setApprovals] = useState<Approval[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!auth.user?.access_token) return;

    setLoading(true);
    fetch(`${config.apiUrl}/api/v1/approvals`, {
      headers: {
        Authorization: `Bearer ${auth.user.access_token}`,
      },
    })
      .then(res => {
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .then(data => {
        setApprovals(data.approvals || []);
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
        <h1 style={styles.title}>Portal Workflows</h1>
      </div>

      <div style={styles.note}>
        <strong>Note:</strong> Per SR-CONTRACT C-TB-3, approvals require HUMAN actor kind.
        Only authenticated human users can record binding decisions.
      </div>

      <div style={styles.tabs}>
        <button
          style={{
            ...styles.tab,
            ...(activeTab === 'approvals' ? styles.tabActive : styles.tabInactive),
          }}
          onClick={() => setActiveTab('approvals')}
        >
          Approvals
        </button>
        <button
          style={{
            ...styles.tab,
            ...(activeTab === 'exceptions' ? styles.tabActive : styles.tabInactive),
          }}
          onClick={() => setActiveTab('exceptions')}
        >
          Exceptions
        </button>
        <button
          style={{
            ...styles.tab,
            ...(activeTab === 'decisions' ? styles.tabActive : styles.tabInactive),
          }}
          onClick={() => setActiveTab('decisions')}
        >
          Decisions
        </button>
      </div>

      <div style={styles.card}>
        {activeTab === 'approvals' && (
          <>
            {loading ? (
              <div style={styles.placeholder}>
                <p>Loading approvals...</p>
              </div>
            ) : error ? (
              <div style={styles.placeholder}>
                <p style={{ color: '#dc3545' }}>Error: {error}</p>
              </div>
            ) : approvals.length === 0 ? (
              <div style={styles.placeholder}>
                <p>No approvals recorded.</p>
                <p style={{ fontSize: '0.875rem', color: '#999' }}>
                  Approvals bind candidates to freeze baselines.
                </p>
              </div>
            ) : (
              <table style={styles.table}>
                <thead>
                  <tr>
                    <th style={styles.th}>ID</th>
                    <th style={styles.th}>Portal</th>
                    <th style={styles.th}>Decision</th>
                    <th style={styles.th}>Actor</th>
                    <th style={styles.th}>Recorded</th>
                  </tr>
                </thead>
                <tbody>
                  {approvals.map(approval => (
                    <tr key={approval.id}>
                      <td style={styles.td}>{approval.id}</td>
                      <td style={styles.td}>{approval.portal_id}</td>
                      <td style={styles.td}>
                        <span
                          style={{
                            ...styles.statusBadge,
                            ...(approval.decision === 'Approved'
                              ? styles.approved
                              : styles.denied),
                          }}
                        >
                          {approval.decision}
                        </span>
                      </td>
                      <td style={styles.td}>{approval.actor_id}</td>
                      <td style={styles.td}>
                        {new Date(approval.recorded_at).toLocaleString()}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            )}
          </>
        )}

        {activeTab === 'exceptions' && (
          <div style={styles.placeholder}>
            <p>Exceptions view coming in D-30</p>
            <p style={{ fontSize: '0.875rem', color: '#999' }}>
              Exceptions are narrowly scoped permissions to deviate from
              governing documents or work instructions.
            </p>
          </div>
        )}

        {activeTab === 'decisions' && (
          <div style={styles.placeholder}>
            <p>Decisions view coming in D-30</p>
            <p style={{ fontSize: '0.875rem', color: '#999' }}>
              Decisions set precedent for future governance choices.
            </p>
          </div>
        )}
      </div>
    </div>
  );
}

export default Approvals;

/**
 * EvidenceBundleSelector Component
 *
 * Dropdown for selecting evidence bundles from the API.
 * Per SR-PLAN-V5 §3.4: Fetches recent evidence and allows manual hash entry.
 */

import { useState, useEffect } from 'react';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import styles from '../styles/pages.module.css';

interface EvidenceSummary {
  content_hash: string;
  bundle_id: string;
  run_id: string;
  candidate_id: string;
  oracle_suite_id: string;
  verdict: string;
  run_completed_at: string;
  artifact_count: number;
}

interface EvidenceBundleSelectorProps {
  value: string;
  onChange: (value: string) => void;
  disabled?: boolean;
}

function formatRelativeTime(isoDate: string): string {
  const date = new Date(isoDate);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffHours = Math.floor(diffMs / (1000 * 60 * 60));
  const diffDays = Math.floor(diffHours / 24);

  if (diffHours < 1) return 'just now';
  if (diffHours < 24) return `${diffHours}h ago`;
  if (diffDays === 1) return 'yesterday';
  return `${diffDays} days ago`;
}

function truncateHash(hash: string, length: number = 16): string {
  if (hash.length <= length) return hash;
  return hash.slice(0, length) + '...';
}

export function EvidenceBundleSelector({
  value,
  onChange,
  disabled = false,
}: EvidenceBundleSelectorProps): JSX.Element {
  const auth = useAuth();
  const [evidence, setEvidence] = useState<EvidenceSummary[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [showManualInput, setShowManualInput] = useState(false);

  useEffect(() => {
    const fetchEvidence = async () => {
      if (!auth.user?.access_token) return;

      setLoading(true);
      setError(null);

      try {
        const res = await fetch(`${config.apiUrl}/api/v1/evidence?limit=20`, {
          headers: { Authorization: `Bearer ${auth.user.access_token}` },
        });

        if (!res.ok) {
          throw new Error(`HTTP ${res.status}`);
        }

        const data = await res.json();
        setEvidence(data.evidence || []);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to load evidence');
      } finally {
        setLoading(false);
      }
    };

    fetchEvidence();
  }, [auth.user?.access_token]);

  const handleSelectChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const selected = e.target.value;
    if (selected === '__manual__') {
      setShowManualInput(true);
      onChange('');
    } else {
      setShowManualInput(false);
      onChange(selected);
    }
  };

  const handleManualInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    onChange(e.target.value);
  };

  if (loading) {
    return (
      <div className={styles.formGroup}>
        <label className={styles.label}>Evidence Bundle *</label>
        <select className={styles.select} disabled>
          <option>Loading evidence bundles...</option>
        </select>
      </div>
    );
  }

  if (error) {
    return (
      <div className={styles.formGroup}>
        <label className={styles.label}>Evidence Bundle *</label>
        <input
          type="text"
          className={styles.input}
          placeholder="Enter evidence bundle hash (e.g., sha256:...)"
          value={value}
          onChange={handleManualInputChange}
          disabled={disabled}
        />
        <span style={{ fontSize: '0.75rem', color: 'var(--muted)' }}>
          Could not load evidence list: {error}
        </span>
      </div>
    );
  }

  return (
    <div className={styles.formGroup}>
      <label className={styles.label}>Evidence Bundle *</label>
      {!showManualInput ? (
        <select
          className={styles.select}
          value={value || ''}
          onChange={handleSelectChange}
          disabled={disabled}
        >
          <option value="">Select an evidence bundle...</option>
          {evidence.map((ev) => (
            <option key={ev.content_hash} value={ev.content_hash}>
              {truncateHash(ev.content_hash)} ({ev.verdict}, {formatRelativeTime(ev.run_completed_at)})
            </option>
          ))}
          <option value="__manual__">Enter hash manually...</option>
        </select>
      ) : (
        <div style={{ display: 'flex', gap: 'var(--space2)' }}>
          <input
            type="text"
            className={styles.input}
            placeholder="sha256:..."
            value={value}
            onChange={handleManualInputChange}
            disabled={disabled}
            style={{ flex: 1 }}
          />
          <button
            type="button"
            className={styles.actionLink}
            onClick={() => setShowManualInput(false)}
            disabled={disabled}
          >
            Show list
          </button>
        </div>
      )}
      <span style={{ fontSize: '0.75rem', color: 'var(--muted)' }}>
        Select from recent evidence bundles or enter a content hash manually
      </span>
    </div>
  );
}

export default EvidenceBundleSelector;

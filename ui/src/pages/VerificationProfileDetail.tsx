/**
 * Verification Profile Detail Page
 *
 * Displays detailed information about a verification profile including
 * required suites, waivable failures, and applicable deliverables.
 */

import { useState, useEffect } from 'react';
import { Link, useParams } from 'react-router-dom';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Pill } from '../ui';
import styles from '../styles/pages.module.css';

// ============================================================================
// Types
// ============================================================================

interface VerificationProfileDetail {
  profile_id: string;
  name: string;
  description: string;
  required_suites: string[];
  optional_suites: string[];
  waivable_failures: string[];
  integrity_conditions: string[];
  applicable_deliverables: string[];
  metadata: Record<string, unknown>;
}

// ============================================================================
// Component
// ============================================================================

export function VerificationProfileDetail(): JSX.Element {
  const auth = useAuth();
  const { profileId } = useParams<{ profileId: string }>();
  const [profile, setProfile] = useState<VerificationProfileDetail | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!auth.user?.access_token || !profileId) return;

    fetch(`${config.apiUrl}/api/v1/oracles/profiles/${encodeURIComponent(profileId)}`, {
      headers: {
        Authorization: `Bearer ${auth.user.access_token}`,
      },
    })
      .then(res => {
        if (res.status === 404) {
          throw new Error('Verification profile not found');
        }
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .then((data: VerificationProfileDetail) => {
        setProfile(data);
        setLoading(false);
      })
      .catch(err => {
        setError(err.message);
        setLoading(false);
      });
  }, [auth.user?.access_token, profileId]);

  const getWaiverDescription = (waiver: string): string => {
    const descriptions: Record<string, string> = {
      'BuildFail': 'Build failures can be waived with proper justification',
      'UnitFail': 'Unit test failures can be waived',
      'LintFail': 'Lint/format failures can be waived',
      'SchemaFail': 'Schema validation failures can be waived',
      'IntegrationFail': 'Integration test failures can be waived',
      'E2eFail': 'End-to-end test failures can be waived',
    };
    return descriptions[waiver] || waiver;
  };

  const getIntegrityDescription = (condition: string): string => {
    const descriptions: Record<string, string> = {
      'OracleTamper': 'Oracle execution was tampered with (non-waivable)',
      'OracleGap': 'Gap in oracle coverage (non-waivable)',
      'OracleEnvMismatch': 'Environment mismatch detected (non-waivable)',
      'OracleFlake': 'Oracle flake detected (non-waivable)',
      'EvidenceMissing': 'Evidence missing or incomplete (non-waivable)',
      'ManifestInvalid': 'Manifest validation failed (non-waivable)',
    };
    return descriptions[condition] || condition;
  };

  if (loading) {
    return (
      <div className={styles.container}>
        <div className={styles.placeholder}>
          <p className={styles.placeholderText}>Loading verification profile...</p>
        </div>
      </div>
    );
  }

  if (error || !profile) {
    return (
      <div className={styles.container}>
        <nav className={styles.breadcrumb}>
          <Link to="/oracles" className={styles.link}>Oracles</Link>
          <span> / </span>
          <span>Profiles</span>
          <span> / </span>
          <span>{profileId}</span>
        </nav>
        <div className={styles.placeholder}>
          <p className={styles.error}>Error: {error || 'Profile not found'}</p>
        </div>
      </div>
    );
  }

  return (
    <div className={styles.container}>
      {/* Breadcrumb */}
      <nav className={styles.breadcrumb}>
        <Link to="/oracles" className={styles.link}>Oracles</Link>
        <span> / </span>
        <span>Profiles</span>
        <span> / </span>
        <span>{profile.profile_id}</span>
      </nav>

      {/* Header */}
      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>{profile.name}</h1>
          <p className={styles.subtitle}>{profile.profile_id}</p>
        </div>
      </div>

      {/* Profile Info */}
      <Card>
        <div className={styles.statsGrid}>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Required Suites</div>
            <div className={styles.statValue}>{profile.required_suites.length}</div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Optional Suites</div>
            <div className={styles.statValue}>{profile.optional_suites.length}</div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Waivable Conditions</div>
            <div className={styles.statValue}>{profile.waivable_failures.length}</div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Applicable Deliverables</div>
            <div className={styles.statValue}>{profile.applicable_deliverables.length}</div>
          </div>
        </div>
      </Card>

      {/* Description */}
      <Card>
        <h3 style={{ margin: '0 0 1rem 0', fontSize: '0.875rem', color: 'var(--ink)' }}>
          Description
        </h3>
        <p style={{ margin: 0, color: 'var(--muted)' }}>{profile.description}</p>
      </Card>

      {/* Required Suites */}
      <Card>
        <h3 style={{ margin: '0 0 1rem 0', fontSize: '0.875rem', color: 'var(--ink)' }}>
          Required Suites
        </h3>
        {profile.required_suites.length === 0 ? (
          <p style={{ margin: 0, color: 'var(--muted)' }}>No required suites.</p>
        ) : (
          <ul style={{ margin: 0, padding: '0 0 0 1.5rem' }}>
            {profile.required_suites.map(suite => (
              <li key={suite} style={{ marginBottom: '0.5rem' }}>
                <Link
                  to={`/oracles/suites/${encodeURIComponent(suite)}`}
                  className={styles.link}
                  style={{ fontFamily: 'var(--mono)' }}
                >
                  {suite}
                </Link>
              </li>
            ))}
          </ul>
        )}
      </Card>

      {/* Optional Suites */}
      {profile.optional_suites.length > 0 && (
        <Card>
          <h3 style={{ margin: '0 0 1rem 0', fontSize: '0.875rem', color: 'var(--ink)' }}>
            Optional Suites
          </h3>
          <ul style={{ margin: 0, padding: '0 0 0 1.5rem' }}>
            {profile.optional_suites.map(suite => (
              <li key={suite} style={{ marginBottom: '0.5rem' }}>
                <Link
                  to={`/oracles/suites/${encodeURIComponent(suite)}`}
                  className={styles.link}
                  style={{ fontFamily: 'var(--mono)' }}
                >
                  {suite}
                </Link>
              </li>
            ))}
          </ul>
        </Card>
      )}

      {/* Waivable Failures */}
      <Card>
        <h3 style={{ margin: '0 0 1rem 0', fontSize: '0.875rem', color: 'var(--ink)' }}>
          Waivable Failures
        </h3>
        <p style={{ margin: '0 0 1rem 0', fontSize: '0.8rem', color: 'var(--muted)' }}>
          These conditions can be waived via the Approvals page with proper justification.
        </p>
        {profile.waivable_failures.length === 0 ? (
          <p style={{ margin: 0, color: 'var(--muted)' }}>No waivable conditions.</p>
        ) : (
          <table className={styles.table}>
            <thead>
              <tr>
                <th className={styles.th}>Condition</th>
                <th className={styles.th}>Description</th>
              </tr>
            </thead>
            <tbody>
              {profile.waivable_failures.map(waiver => (
                <tr key={waiver}>
                  <td className={styles.td}>
                    <Pill tone="warning">{waiver}</Pill>
                  </td>
                  <td className={styles.td} style={{ color: 'var(--muted)' }}>
                    {getWaiverDescription(waiver)}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </Card>

      {/* Integrity Conditions (Non-waivable) */}
      <Card>
        <h3 style={{ margin: '0 0 1rem 0', fontSize: '0.875rem', color: 'var(--ink)' }}>
          Integrity Conditions (Non-waivable)
        </h3>
        <p style={{ margin: '0 0 1rem 0', fontSize: '0.8rem', color: 'var(--muted)' }}>
          These conditions always block and cannot be waived. They indicate oracle integrity issues.
        </p>
        {profile.integrity_conditions.length === 0 ? (
          <p style={{ margin: 0, color: 'var(--muted)' }}>No integrity conditions defined.</p>
        ) : (
          <table className={styles.table}>
            <thead>
              <tr>
                <th className={styles.th}>Condition</th>
                <th className={styles.th}>Description</th>
              </tr>
            </thead>
            <tbody>
              {profile.integrity_conditions.map(condition => (
                <tr key={condition}>
                  <td className={styles.td}>
                    <Pill tone="danger">{condition}</Pill>
                  </td>
                  <td className={styles.td} style={{ color: 'var(--muted)' }}>
                    {getIntegrityDescription(condition)}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </Card>

      {/* Applicable Deliverables */}
      <Card>
        <h3 style={{ margin: '0 0 1rem 0', fontSize: '0.875rem', color: 'var(--ink)' }}>
          Applicable Deliverables
        </h3>
        <p style={{ margin: '0 0 1rem 0', fontSize: '0.8rem', color: 'var(--muted)' }}>
          This profile applies to the following deliverable IDs.
        </p>
        <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.5rem' }}>
          {profile.applicable_deliverables.length === 0 ? (
            <span style={{ color: 'var(--muted)' }}>No deliverables specified.</span>
          ) : (
            profile.applicable_deliverables.map(deliverable => (
              <Pill key={deliverable} tone="neutral">
                {deliverable}
              </Pill>
            ))
          )}
        </div>
      </Card>
    </div>
  );
}

export default VerificationProfileDetail;

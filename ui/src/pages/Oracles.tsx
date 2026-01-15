/**
 * Oracles Page
 *
 * Displays registered Oracle Suites and Verification Profiles.
 * Per SR-SEMANTIC-ORACLE-SPEC: Oracle suites contain oracle definitions
 * that emit structured evidence for stage gates.
 *
 * Key concepts:
 * - Oracle Suite: Collection of oracles with a deterministic hash
 * - Verification Profile: Maps deliverables to required oracle suites
 * - Semantic Sets: Meaning-matrices for semantic oracles
 */

import { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Pill } from '../ui';
import styles from '../styles/pages.module.css';

// ============================================================================
// Types
// ============================================================================

interface OracleSuiteSummary {
  suite_id: string;
  suite_hash: string;
  oracle_count: number;
  oci_image: string;
  network_mode: string;
  runtime: string;
  semantic_set_id?: string;
}

interface VerificationProfileSummary {
  profile_id: string;
  name: string;
  description: string;
  required_suite_count: number;
  applicable_deliverable_count: number;
}

interface ListSuitesResponse {
  suites: OracleSuiteSummary[];
  total: number;
}

interface ListProfilesResponse {
  profiles: VerificationProfileSummary[];
  total: number;
}

// ============================================================================
// Component
// ============================================================================

export function Oracles(): JSX.Element {
  const auth = useAuth();
  const [activeTab, setActiveTab] = useState<'suites' | 'profiles'>('suites');
  const [suites, setSuites] = useState<OracleSuiteSummary[]>([]);
  const [profiles, setProfiles] = useState<VerificationProfileSummary[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!auth.user?.access_token) return;

    const fetchSuites = fetch(`${config.apiUrl}/api/v1/oracles/suites`, {
      headers: {
        Authorization: `Bearer ${auth.user.access_token}`,
      },
    })
      .then(res => {
        if (res.status === 404) return { suites: [], total: 0 };
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .then((data: ListSuitesResponse) => {
        setSuites(data.suites || []);
      });

    const fetchProfiles = fetch(`${config.apiUrl}/api/v1/oracles/profiles`, {
      headers: {
        Authorization: `Bearer ${auth.user.access_token}`,
      },
    })
      .then(res => {
        if (res.status === 404) return { profiles: [], total: 0 };
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .then((data: ListProfilesResponse) => {
        setProfiles(data.profiles || []);
      });

    Promise.all([fetchSuites, fetchProfiles])
      .then(() => setLoading(false))
      .catch(err => {
        setError(err.message);
        setLoading(false);
      });
  }, [auth.user?.access_token]);

  const truncateHash = (hash: string) => {
    if (hash.startsWith('sha256:')) {
      return `sha256:${hash.slice(7, 15)}...`;
    }
    return hash.length > 16 ? `${hash.slice(0, 16)}...` : hash;
  };

  const getNetworkTone = (mode: string): 'success' | 'warning' | 'danger' | 'neutral' => {
    switch (mode.toLowerCase()) {
      case 'disabled':
        return 'success';
      case 'private':
        return 'warning';
      case 'host':
        return 'danger';
      default:
        return 'neutral';
    }
  };

  const semanticSuiteCount = suites.filter(s => s.semantic_set_id).length;

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>Oracles</h1>
          <p className={styles.subtitle}>Oracle suites and verification profiles for evidence gates</p>
        </div>
      </div>

      {/* Overview Stats */}
      <Card>
        <div className={styles.statsGrid}>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Oracle Suites</div>
            <div className={styles.statValue}>{suites.length}</div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Verification Profiles</div>
            <div className={styles.statValue}>{profiles.length}</div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Semantic Suites</div>
            <div className={styles.statValue}>{semanticSuiteCount}</div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Type</div>
            <div className={styles.statValue}>config.oracle_definition</div>
          </div>
        </div>
      </Card>

      {/* Info Note */}
      <div className={styles.note}>
        Per SR-SEMANTIC-ORACLE-SPEC: Oracle suites emit structured semantic measurements
        about Candidates. Suite hashes incorporate all oracle definitions for deterministic
        evidence binding. Suites may be Required, Advisory, or Optional.
      </div>

      {/* Tabs */}
      <div className={styles.tabs}>
        <button
          className={`${styles.tab} ${activeTab === 'suites' ? styles.tabActive : ''}`}
          onClick={() => setActiveTab('suites')}
        >
          Oracle Suites ({suites.length})
        </button>
        <button
          className={`${styles.tab} ${activeTab === 'profiles' ? styles.tabActive : ''}`}
          onClick={() => setActiveTab('profiles')}
        >
          Verification Profiles ({profiles.length})
        </button>
      </div>

      {/* Content */}
      <Card>
        {loading ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>Loading oracles...</p>
          </div>
        ) : error ? (
          <div className={styles.placeholder}>
            <p className={styles.error}>Error: {error}</p>
          </div>
        ) : activeTab === 'suites' ? (
          /* Suites Tab */
          suites.length === 0 ? (
            <div className={styles.placeholder}>
              <p className={styles.placeholderText}>No oracle suites registered.</p>
              <p className={styles.placeholderHint}>
                Oracle suites define verification oracles for stage gates.
              </p>
            </div>
          ) : (
            <table className={styles.table}>
              <thead>
                <tr>
                  <th className={styles.th}>Suite ID</th>
                  <th className={styles.th}>Hash</th>
                  <th className={styles.th}>Oracles</th>
                  <th className={styles.th}>OCI Image</th>
                  <th className={styles.th}>Network</th>
                  <th className={styles.th}>Type</th>
                </tr>
              </thead>
              <tbody>
                {suites.map(suite => (
                  <tr key={suite.suite_id}>
                    <td className={styles.td}>
                      <Link
                        to={`/oracles/suites/${encodeURIComponent(suite.suite_id)}`}
                        className={styles.link}
                      >
                        {suite.suite_id}
                      </Link>
                    </td>
                    <td className={styles.tdMono} title={suite.suite_hash}>
                      {truncateHash(suite.suite_hash)}
                    </td>
                    <td className={styles.td}>{suite.oracle_count}</td>
                    <td className={styles.tdMono} style={{ maxWidth: '200px', overflow: 'hidden', textOverflow: 'ellipsis' }}>
                      {suite.oci_image}
                    </td>
                    <td className={styles.td}>
                      <Pill tone={getNetworkTone(suite.network_mode)}>
                        {suite.network_mode}
                      </Pill>
                    </td>
                    <td className={styles.td}>
                      {suite.semantic_set_id ? (
                        <Pill tone="warning">Semantic</Pill>
                      ) : (
                        <Pill tone="neutral">Standard</Pill>
                      )}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          )
        ) : (
          /* Profiles Tab */
          profiles.length === 0 ? (
            <div className={styles.placeholder}>
              <p className={styles.placeholderText}>No verification profiles registered.</p>
              <p className={styles.placeholderHint}>
                Verification profiles map deliverables to required oracle suites.
              </p>
            </div>
          ) : (
            <table className={styles.table}>
              <thead>
                <tr>
                  <th className={styles.th}>Profile ID</th>
                  <th className={styles.th}>Name</th>
                  <th className={styles.th}>Description</th>
                  <th className={styles.th}>Required Suites</th>
                  <th className={styles.th}>Deliverables</th>
                </tr>
              </thead>
              <tbody>
                {profiles.map(profile => (
                  <tr key={profile.profile_id}>
                    <td className={styles.td}>
                      <Link
                        to={`/oracles/profiles/${encodeURIComponent(profile.profile_id)}`}
                        className={styles.link}
                      >
                        {profile.profile_id}
                      </Link>
                    </td>
                    <td className={styles.td}>{profile.name}</td>
                    <td className={styles.td} style={{ maxWidth: '300px' }}>
                      {profile.description}
                    </td>
                    <td className={styles.td}>{profile.required_suite_count}</td>
                    <td className={styles.td}>{profile.applicable_deliverable_count}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          )
        )}
      </Card>

      {/* Core Suites Reference */}
      <Card>
        <h3 style={{ margin: '0 0 1rem 0', fontSize: '0.875rem', color: 'var(--ink)' }}>
          Core Oracle Suites (SR-DIRECTIVE)
        </h3>
        <table className={styles.table}>
          <thead>
            <tr>
              <th className={styles.th}>Suite</th>
              <th className={styles.th}>Purpose</th>
              <th className={styles.th}>Oracles</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td className={styles.tdMono}>suite:SR-SUITE-GOV</td>
              <td className={styles.td}>Governance-only (meta_validate, refs_validate)</td>
              <td className={styles.td}>2</td>
            </tr>
            <tr>
              <td className={styles.tdMono}>suite:SR-SUITE-CORE</td>
              <td className={styles.td}>Default for code (build, test, lint, schema, integrity)</td>
              <td className={styles.td}>6</td>
            </tr>
            <tr>
              <td className={styles.tdMono}>suite:SR-SUITE-FULL</td>
              <td className={styles.td}>Full stack (integration, e2e, replay, sbom)</td>
              <td className={styles.td}>10</td>
            </tr>
            <tr>
              <td className={styles.tdMono}>oracle.suite.intake_admissibility.v1</td>
              <td className={styles.td}>Semantic intake validation (6 axes)</td>
              <td className={styles.td}>6</td>
            </tr>
          </tbody>
        </table>
      </Card>
    </div>
  );
}

export default Oracles;

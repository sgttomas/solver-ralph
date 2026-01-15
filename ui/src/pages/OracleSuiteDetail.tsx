/**
 * Oracle Suite Detail Page
 *
 * Displays detailed information about a single oracle suite including
 * its oracles, environment constraints, and metadata.
 */

import { useState, useEffect } from 'react';
import { Link, useParams } from 'react-router-dom';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Pill, Button } from '../ui';
import styles from '../styles/pages.module.css';

// ============================================================================
// Types
// ============================================================================

interface ExpectedOutput {
  path: string;
  content_type: string;
  required: boolean;
}

interface OracleDefinition {
  oracle_id: string;
  oracle_name: string;
  command: string;
  timeout_seconds: number;
  expected_outputs: ExpectedOutput[];
  classification: string;
  working_dir: string | null;
  env: Record<string, string>;
}

interface EnvironmentConstraints {
  runtime: string;
  network: string;
  cpu_arch: string;
  os: string;
  workspace_readonly: boolean;
  additional_constraints: string[];
}

interface OracleSuiteDetail {
  suite_id: string;
  suite_hash: string;
  oci_image: string;
  oci_image_digest: string;
  environment_constraints: EnvironmentConstraints;
  oracles: OracleDefinition[];
  metadata: Record<string, unknown>;
}

// ============================================================================
// Component
// ============================================================================

export function OracleSuiteDetail(): JSX.Element {
  const auth = useAuth();
  const { suiteId } = useParams<{ suiteId: string }>();
  const [suite, setSuite] = useState<OracleSuiteDetail | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [showRawJson, setShowRawJson] = useState(false);
  const [expandedOracle, setExpandedOracle] = useState<string | null>(null);

  useEffect(() => {
    if (!auth.user?.access_token || !suiteId) return;

    fetch(`${config.apiUrl}/api/v1/oracles/suites/${encodeURIComponent(suiteId)}`, {
      headers: {
        Authorization: `Bearer ${auth.user.access_token}`,
      },
    })
      .then(res => {
        if (res.status === 404) {
          throw new Error('Oracle suite not found');
        }
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .then((data: OracleSuiteDetail) => {
        setSuite(data);
        setLoading(false);
      })
      .catch(err => {
        setError(err.message);
        setLoading(false);
      });
  }, [auth.user?.access_token, suiteId]);

  const getClassificationTone = (classification: string): 'success' | 'warning' | 'neutral' => {
    switch (classification.toLowerCase()) {
      case 'required':
        return 'success';
      case 'advisory':
        return 'warning';
      default:
        return 'neutral';
    }
  };

  const getNetworkTone = (mode: string): 'success' | 'warning' | 'danger' => {
    switch (mode.toLowerCase()) {
      case 'disabled':
        return 'success';
      case 'private':
        return 'warning';
      default:
        return 'danger';
    }
  };

  if (loading) {
    return (
      <div className={styles.container}>
        <div className={styles.placeholder}>
          <p className={styles.placeholderText}>Loading oracle suite...</p>
        </div>
      </div>
    );
  }

  if (error || !suite) {
    return (
      <div className={styles.container}>
        <nav className={styles.breadcrumb}>
          <Link to="/oracles" className={styles.link}>Oracles</Link>
          <span> / </span>
          <span>{suiteId}</span>
        </nav>
        <div className={styles.placeholder}>
          <p className={styles.error}>Error: {error || 'Suite not found'}</p>
        </div>
      </div>
    );
  }

  const semanticSetId = suite.metadata?.semantic_set_id as string | undefined;

  return (
    <div className={styles.container}>
      {/* Breadcrumb */}
      <nav className={styles.breadcrumb}>
        <Link to="/oracles" className={styles.link}>Oracles</Link>
        <span> / </span>
        <span>{suite.suite_id}</span>
      </nav>

      {/* Header */}
      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>{suite.suite_id}</h1>
          <p className={styles.subtitle}>Oracle Suite with {suite.oracles.length} oracles</p>
        </div>
        <div className={styles.headerEnd}>
          <Button variant="secondary" onClick={() => setShowRawJson(!showRawJson)}>
            {showRawJson ? 'Hide' : 'Show'} Raw JSON
          </Button>
        </div>
      </div>

      {/* Suite Info */}
      <Card>
        <div className={styles.statsGrid}>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Suite Hash</div>
            <div className={styles.statValue} style={{ fontSize: '0.75rem', fontFamily: 'var(--mono)' }}>
              {suite.suite_hash}
            </div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Oracle Count</div>
            <div className={styles.statValue}>{suite.oracles.length}</div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Type</div>
            <div className={styles.statValue}>
              {semanticSetId ? 'Semantic' : 'Standard'}
            </div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statLabel}>Runtime</div>
            <div className={styles.statValue}>{suite.environment_constraints.runtime}</div>
          </div>
        </div>
      </Card>

      {/* Environment Constraints */}
      <Card>
        <h3 style={{ margin: '0 0 1rem 0', fontSize: '0.875rem', color: 'var(--ink)' }}>
          Environment Constraints
        </h3>
        <div className={styles.infoRows}>
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Runtime</span>
            <span className={styles.infoValue}>{suite.environment_constraints.runtime}</span>
          </div>
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Network Mode</span>
            <span className={styles.infoValue}>
              <Pill tone={getNetworkTone(suite.environment_constraints.network)}>
                {suite.environment_constraints.network}
              </Pill>
            </span>
          </div>
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Architecture</span>
            <span className={styles.infoValue}>{suite.environment_constraints.cpu_arch}</span>
          </div>
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>OS</span>
            <span className={styles.infoValue}>{suite.environment_constraints.os}</span>
          </div>
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Workspace</span>
            <span className={styles.infoValue}>
              {suite.environment_constraints.workspace_readonly ? (
                <Pill tone="success">Read-only</Pill>
              ) : (
                <Pill tone="warning">Read-write</Pill>
              )}
            </span>
          </div>
          {suite.environment_constraints.additional_constraints.length > 0 && (
            <div className={styles.infoRow}>
              <span className={styles.infoLabel}>Additional</span>
              <span className={styles.infoValue}>
                {suite.environment_constraints.additional_constraints.join(', ')}
              </span>
            </div>
          )}
        </div>
      </Card>

      {/* OCI Image */}
      <Card>
        <h3 style={{ margin: '0 0 1rem 0', fontSize: '0.875rem', color: 'var(--ink)' }}>
          OCI Image
        </h3>
        <div className={styles.infoRows}>
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Image</span>
            <span className={styles.infoValue} style={{ fontFamily: 'var(--mono)', fontSize: '0.8rem' }}>
              {suite.oci_image}
            </span>
          </div>
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Digest</span>
            <span className={styles.infoValue} style={{ fontFamily: 'var(--mono)', fontSize: '0.8rem' }}>
              {suite.oci_image_digest}
            </span>
          </div>
        </div>
      </Card>

      {/* Semantic Set (if applicable) */}
      {semanticSetId && (
        <Card>
          <h3 style={{ margin: '0 0 1rem 0', fontSize: '0.875rem', color: 'var(--ink)' }}>
            Semantic Set
          </h3>
          <div className={styles.infoRows}>
            <div className={styles.infoRow}>
              <span className={styles.infoLabel}>Semantic Set ID</span>
              <span className={styles.infoValue} style={{ fontFamily: 'var(--mono)' }}>
                {semanticSetId}
              </span>
            </div>
            {suite.metadata?.semantic_set_hash !== undefined && (
              <div className={styles.infoRow}>
                <span className={styles.infoLabel}>Semantic Set Hash</span>
                <span className={styles.infoValue} style={{ fontFamily: 'var(--mono)', fontSize: '0.75rem' }}>
                  {String(suite.metadata.semantic_set_hash)}
                </span>
              </div>
            )}
          </div>
        </Card>
      )}

      {/* Oracles Table */}
      <Card>
        <h3 style={{ margin: '0 0 1rem 0', fontSize: '0.875rem', color: 'var(--ink)' }}>
          Oracles ({suite.oracles.length})
        </h3>
        <table className={styles.table}>
          <thead>
            <tr>
              <th className={styles.th}>Oracle ID</th>
              <th className={styles.th}>Name</th>
              <th className={styles.th}>Timeout</th>
              <th className={styles.th}>Classification</th>
              <th className={styles.th}>Outputs</th>
              <th className={styles.th}>Actions</th>
            </tr>
          </thead>
          <tbody>
            {suite.oracles.map(oracle => (
              <>
                <tr key={oracle.oracle_id}>
                  <td className={styles.tdMono}>{oracle.oracle_id}</td>
                  <td className={styles.td}>{oracle.oracle_name}</td>
                  <td className={styles.td}>{oracle.timeout_seconds}s</td>
                  <td className={styles.td}>
                    <Pill tone={getClassificationTone(oracle.classification)}>
                      {oracle.classification}
                    </Pill>
                  </td>
                  <td className={styles.td}>{oracle.expected_outputs.length}</td>
                  <td className={styles.td}>
                    <button
                      className={styles.actionLink}
                      onClick={() => setExpandedOracle(
                        expandedOracle === oracle.oracle_id ? null : oracle.oracle_id
                      )}
                    >
                      {expandedOracle === oracle.oracle_id ? 'Collapse' : 'Expand'}
                    </button>
                  </td>
                </tr>
                {expandedOracle === oracle.oracle_id && (
                  <tr key={`${oracle.oracle_id}-detail`}>
                    <td colSpan={6} style={{ padding: '1rem', background: 'var(--paper2)' }}>
                      <div style={{ display: 'flex', flexDirection: 'column', gap: '0.75rem' }}>
                        <div>
                          <strong style={{ fontSize: '0.75rem', color: 'var(--muted)' }}>Command:</strong>
                          <pre style={{
                            margin: '0.25rem 0 0 0',
                            padding: '0.5rem',
                            background: 'var(--ink)',
                            color: 'var(--paper)',
                            borderRadius: '4px',
                            fontSize: '0.75rem',
                            overflow: 'auto',
                            maxHeight: '100px'
                          }}>
                            {oracle.command}
                          </pre>
                        </div>
                        {oracle.working_dir && (
                          <div>
                            <strong style={{ fontSize: '0.75rem', color: 'var(--muted)' }}>Working Dir:</strong>
                            <code style={{ marginLeft: '0.5rem', fontFamily: 'var(--mono)' }}>{oracle.working_dir}</code>
                          </div>
                        )}
                        <div>
                          <strong style={{ fontSize: '0.75rem', color: 'var(--muted)' }}>Expected Outputs:</strong>
                          <ul style={{ margin: '0.25rem 0 0 1rem', padding: 0 }}>
                            {oracle.expected_outputs.map((output, idx) => (
                              <li key={idx} style={{ fontSize: '0.8rem' }}>
                                <code style={{ fontFamily: 'var(--mono)' }}>{output.path}</code>
                                {' '}
                                <span style={{ color: 'var(--muted)' }}>({output.content_type})</span>
                                {' '}
                                {output.required && <Pill tone="success">required</Pill>}
                              </li>
                            ))}
                          </ul>
                        </div>
                        {Object.keys(oracle.env).length > 0 && (
                          <div>
                            <strong style={{ fontSize: '0.75rem', color: 'var(--muted)' }}>Environment:</strong>
                            <ul style={{ margin: '0.25rem 0 0 1rem', padding: 0 }}>
                              {Object.entries(oracle.env).map(([key, value]) => (
                                <li key={key} style={{ fontSize: '0.8rem', fontFamily: 'var(--mono)' }}>
                                  {key}={value}
                                </li>
                              ))}
                            </ul>
                          </div>
                        )}
                      </div>
                    </td>
                  </tr>
                )}
              </>
            ))}
          </tbody>
        </table>
      </Card>

      {/* Raw JSON */}
      {showRawJson && (
        <Card>
          <h3 style={{ margin: '0 0 1rem 0', fontSize: '0.875rem', color: 'var(--ink)' }}>
            Raw JSON
          </h3>
          <pre style={{
            padding: '1rem',
            background: 'var(--ink)',
            color: 'var(--paper)',
            borderRadius: '4px',
            fontSize: '0.75rem',
            overflow: 'auto',
            maxHeight: '400px'
          }}>
            {JSON.stringify(suite, null, 2)}
          </pre>
        </Card>
      )}
    </div>
  );
}

export default OracleSuiteDetail;

/**
 * Evidence Detail Page (D-29)
 *
 * Displays a single evidence bundle with its manifest, oracle results, and artifacts.
 * Per SR-SPEC, evidence bundles are content-addressed oracle outputs supporting verification.
 */

import { useState, useEffect } from 'react';
import { useParams, Link } from 'react-router-dom';
import { useAuth } from '../auth/AuthProvider';
import config from '../config';
import { Card, Pill, Button, getStatusTone, truncateHash } from '../ui';
import styles from '../styles/pages.module.css';

interface OracleResult {
  oracle_id: string;
  status: string;
  message: string | null;
  duration_ms: number | null;
  artifacts: string[];
}

interface EvidenceArtifact {
  name: string;
  content_hash: string;
  media_type: string;
  size_bytes: number;
}

interface Attribution {
  actor_kind: string;
  actor_id: string;
  run_id: string | null;
  candidate_id: string | null;
}

interface EvidenceManifest {
  schema_version: string;
  artifact_type: string;
  suite_id: string;
  suite_hash: string;
  verdict: string;
  oracle_results: OracleResult[];
  artifacts: EvidenceArtifact[];
  attribution: Attribution;
  environment_fingerprint: string | null;
  created_at: string;
}

interface EvidenceBundle {
  content_hash: string;
  manifest: EvidenceManifest;
}

function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

export function EvidenceDetail(): JSX.Element {
  const { contentHash } = useParams<{ contentHash: string }>();
  const auth = useAuth();
  const [bundle, setBundle] = useState<EvidenceBundle | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [showRawManifest, setShowRawManifest] = useState(false);

  useEffect(() => {
    if (!auth.user?.access_token || !contentHash) return;

    fetch(`${config.apiUrl}/api/v1/evidence/${contentHash}`, {
      headers: { Authorization: `Bearer ${auth.user.access_token}` },
    })
      .then(res => {
        if (!res.ok) throw new Error(`Evidence fetch failed: HTTP ${res.status}`);
        return res.json();
      })
      .then(data => {
        setBundle(data);
        setLoading(false);
      })
      .catch(err => {
        setError(err.message);
        setLoading(false);
      });
  }, [auth.user?.access_token, contentHash]);

  if (loading) {
    return (
      <div className={styles.container}>
        <div className={styles.placeholder}>
          <p className={styles.placeholderText}>Loading artifact details...</p>
        </div>
      </div>
    );
  }

  if (error || !bundle) {
    return (
      <div className={styles.container}>
        <div className={styles.placeholder}>
          <p className={styles.error}>Error: {error || 'Artifact not found'}</p>
          <Link to="/evidence" className={styles.link}>Back to Artifacts</Link>
        </div>
      </div>
    );
  }

  const { manifest } = bundle;

  // Count oracle results by status
  const resultCounts = manifest.oracle_results.reduce(
    (acc, r) => {
      acc[r.status] = (acc[r.status] || 0) + 1;
      return acc;
    },
    {} as Record<string, number>
  );

  return (
    <div className={styles.container}>
      {/* Breadcrumb */}
      <div className={styles.breadcrumb}>
        <Link to="/evidence" className={styles.breadcrumbLink}>Artifacts</Link>
        <span className={styles.breadcrumbSeparator}>/</span>
        <span>{truncateHash(bundle.content_hash, 16)}</span>
      </div>

      {/* Header */}
      <div className={styles.header}>
        <div className={styles.headerStart}>
          <h1 className={styles.title}>Artifact Bundle</h1>
          <p className={styles.subtitle} style={{ wordBreak: 'break-all' }}>{bundle.content_hash}</p>
        </div>
        <Pill tone={getStatusTone(manifest.verdict)}>{manifest.verdict}</Pill>
      </div>

      {/* Overview Card */}
      <Card title="Overview" className={styles.cardSpacing}>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Artifact Type</span>
          <span className={styles.infoValue}>{manifest.artifact_type}</span>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Schema Version</span>
          <span className={styles.infoValue}>{manifest.schema_version}</span>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Suite ID</span>
          <code className={styles.mono}>{manifest.suite_id}</code>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Suite Hash</span>
          <code className={styles.mono}>{manifest.suite_hash}</code>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Created</span>
          <span className={styles.infoValue}>
            {new Date(manifest.created_at).toLocaleString()}
          </span>
        </div>
        {manifest.environment_fingerprint && (
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Environment</span>
            <code className={styles.mono}>{manifest.environment_fingerprint}</code>
          </div>
        )}
      </Card>

      {/* Attribution Card */}
      <Card title="Attribution" className={styles.cardSpacing}>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Actor Kind</span>
          <Pill tone={getStatusTone(manifest.attribution.actor_kind)}>
            {manifest.attribution.actor_kind}
          </Pill>
        </div>
        <div className={styles.infoRow}>
          <span className={styles.infoLabel}>Actor ID</span>
          <span className={styles.infoValue}>{manifest.attribution.actor_id}</span>
        </div>
        {manifest.attribution.run_id && (
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Run ID</span>
            <span className={styles.infoValue}>{manifest.attribution.run_id}</span>
          </div>
        )}
        {manifest.attribution.candidate_id && (
          <div className={styles.infoRow}>
            <span className={styles.infoLabel}>Candidate</span>
            <Link
              to={`/candidates/${manifest.attribution.candidate_id}`}
              className={styles.link}
            >
              {manifest.attribution.candidate_id}
            </Link>
          </div>
        )}
      </Card>

      {/* Oracle Results Card */}
      <Card
        title={`Oracle Results (${manifest.oracle_results.length})`}
        right={
          <div className={styles.badgeGroup}>
            {Object.entries(resultCounts).map(([status, count]) => (
              <Pill key={status} tone={getStatusTone(status)}>
                {count} {status}
              </Pill>
            ))}
          </div>
        }
        className={styles.cardSpacing}
      >
        {manifest.oracle_results.length === 0 ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>No oracle results in this bundle.</p>
          </div>
        ) : (
          manifest.oracle_results.map((result, idx) => (
            <div key={idx} className={styles.refItem} style={{ flexDirection: 'column', alignItems: 'stretch', gap: 'var(--space2)' }}>
              <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                <span style={{ fontWeight: 500, fontSize: '0.875rem' }}>{result.oracle_id}</span>
                <div className={styles.badgeGroup}>
                  {result.duration_ms !== null && (
                    <span style={{ fontSize: '0.75rem', color: 'var(--muted)' }}>
                      {result.duration_ms}ms
                    </span>
                  )}
                  <Pill tone={getStatusTone(result.status)}>{result.status}</Pill>
                </div>
              </div>
              {result.message && (
                <p style={{ fontSize: '0.75rem', color: 'var(--muted)', margin: 0, fontStyle: 'italic' }}>
                  {result.message}
                </p>
              )}
              {result.artifacts.length > 0 && (
                <div style={{ fontSize: '0.75rem', color: 'var(--muted)' }}>
                  Artifacts: {result.artifacts.join(', ')}
                </div>
              )}
            </div>
          ))
        )}
      </Card>

      {/* Artifacts Card */}
      <Card title={`Artifacts (${manifest.artifacts.length})`} className={styles.cardSpacing}>
        {manifest.artifacts.length === 0 ? (
          <div className={styles.placeholder}>
            <p className={styles.placeholderText}>No artifacts in this bundle.</p>
          </div>
        ) : (
          manifest.artifacts.map((artifact, idx) => (
            <div key={idx} className={styles.refItem} style={{ justifyContent: 'space-between' }}>
              <div style={{ display: 'flex', flexDirection: 'column', gap: 'var(--space1)' }}>
                <span style={{ fontWeight: 500, fontSize: '0.875rem' }}>{artifact.name}</span>
                <span style={{ fontSize: '0.75rem', color: 'var(--muted)' }}>
                  {artifact.media_type} &bull; {formatBytes(artifact.size_bytes)}
                </span>
                <code className={styles.mono} style={{ fontSize: '0.7rem' }}>
                  {artifact.content_hash}
                </code>
              </div>
              <a
                href={`${config.apiUrl}/api/v1/evidence/${bundle.content_hash}/blobs/${artifact.name}`}
                target="_blank"
                rel="noopener noreferrer"
                style={{ textDecoration: 'none' }}
              >
                <Button variant="primary">Download</Button>
              </a>
            </div>
          ))
        )}
      </Card>

      {/* Raw Manifest Card */}
      <Card
        title="Raw Manifest"
        right={
          <Button
            variant={showRawManifest ? 'ghost' : 'secondary'}
            onClick={() => setShowRawManifest(!showRawManifest)}
          >
            {showRawManifest ? 'Hide' : 'Show'}
          </Button>
        }
      >
        {showRawManifest && (
          <pre style={{
            backgroundColor: 'var(--ink)',
            color: '#e0e0e0',
            padding: 'var(--space4)',
            borderRadius: 'var(--radiusSm)',
            fontFamily: 'var(--mono)',
            fontSize: '0.75rem',
            overflow: 'auto',
            maxHeight: '400px',
            whiteSpace: 'pre-wrap',
            wordBreak: 'break-all',
            margin: 0,
          }}>
            {JSON.stringify(manifest, null, 2)}
          </pre>
        )}
      </Card>
    </div>
  );
}

export default EvidenceDetail;

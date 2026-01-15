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

const styles = {
  container: {
    maxWidth: '1200px',
    margin: '0 auto',
  },
  breadcrumb: {
    marginBottom: '1rem',
    fontSize: '0.875rem',
  },
  breadcrumbLink: {
    color: '#0066cc',
    textDecoration: 'none',
  },
  header: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'flex-start',
    marginBottom: '1.5rem',
  },
  title: {
    margin: 0,
    fontSize: '1.5rem',
    color: '#1a1a2e',
  },
  subtitle: {
    margin: '0.5rem 0 0 0',
    fontSize: '0.75rem',
    color: '#666',
    fontFamily: 'monospace',
    wordBreak: 'break-all' as const,
  },
  card: {
    backgroundColor: 'white',
    borderRadius: '8px',
    padding: '1.5rem',
    boxShadow: '0 1px 3px rgba(0, 0, 0, 0.1)',
    marginBottom: '1.5rem',
  },
  cardTitle: {
    margin: '0 0 1rem 0',
    fontSize: '1rem',
    color: '#1a1a2e',
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
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
  link: {
    color: '#0066cc',
    textDecoration: 'none',
  },
  placeholder: {
    textAlign: 'center' as const,
    padding: '2rem',
    color: '#666',
  },
  infoRow: {
    display: 'flex',
    marginBottom: '0.5rem',
  },
  infoLabel: {
    width: '160px',
    fontSize: '0.75rem',
    color: '#666',
    textTransform: 'uppercase' as const,
    flexShrink: 0,
  },
  infoValue: {
    flex: 1,
    fontSize: '0.875rem',
    wordBreak: 'break-all' as const,
  },
  monospace: {
    fontFamily: 'monospace',
    fontSize: '0.75rem',
    backgroundColor: '#f5f5f5',
    padding: '0.25rem 0.5rem',
    borderRadius: '4px',
  },
  downloadButton: {
    display: 'inline-flex',
    alignItems: 'center',
    padding: '0.375rem 0.75rem',
    fontSize: '0.75rem',
    backgroundColor: '#0066cc',
    color: 'white',
    border: 'none',
    borderRadius: '4px',
    cursor: 'pointer',
    textDecoration: 'none',
  },
  oracleCard: {
    backgroundColor: '#f8f9fa',
    borderRadius: '4px',
    padding: '1rem',
    marginBottom: '0.75rem',
  },
  oracleHeader: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: '0.5rem',
  },
  oracleId: {
    fontFamily: 'monospace',
    fontSize: '0.875rem',
    fontWeight: 500,
  },
  oracleMessage: {
    fontSize: '0.75rem',
    color: '#666',
    margin: '0.5rem 0 0 0',
    fontStyle: 'italic',
  },
  artifactRow: {
    display: 'flex',
    alignItems: 'center',
    padding: '0.5rem',
    backgroundColor: '#f8f9fa',
    borderRadius: '4px',
    marginBottom: '0.5rem',
  },
  artifactInfo: {
    flex: 1,
  },
  artifactName: {
    fontFamily: 'monospace',
    fontSize: '0.875rem',
    fontWeight: 500,
  },
  artifactMeta: {
    fontSize: '0.75rem',
    color: '#666',
  },
  jsonViewer: {
    backgroundColor: '#1a1a2e',
    color: '#e0e0e0',
    padding: '1rem',
    borderRadius: '4px',
    fontFamily: 'monospace',
    fontSize: '0.75rem',
    overflow: 'auto',
    maxHeight: '400px',
    whiteSpace: 'pre-wrap' as const,
    wordBreak: 'break-all' as const,
  },
};

const statusColors: Record<string, { bg: string; color: string }> = {
  PASS: { bg: '#d4edda', color: '#155724' },
  FAIL: { bg: '#f8d7da', color: '#721c24' },
  ERROR: { bg: '#f5c6cb', color: '#721c24' },
  SKIPPED: { bg: '#e2e3e5', color: '#383d41' },
  HUMAN: { bg: '#cce5ff', color: '#004085' },
  AGENT: { bg: '#fff3cd', color: '#856404' },
  SYSTEM: { bg: '#e2e3e5', color: '#383d41' },
};

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
      <div style={styles.container}>
        <div style={styles.placeholder}>
          <p>Loading artifact details...</p>
        </div>
      </div>
    );
  }

  if (error || !bundle) {
    return (
      <div style={styles.container}>
        <div style={styles.placeholder}>
          <p style={{ color: '#dc3545' }}>Error: {error || 'Artifact not found'}</p>
          <Link to="/evidence" style={styles.link}>Back to Artifacts</Link>
        </div>
      </div>
    );
  }

  const { manifest } = bundle;
  const verdictStyle = statusColors[manifest.verdict] || statusColors.SKIPPED;
  const actorStyle = statusColors[manifest.attribution.actor_kind] || statusColors.SYSTEM;

  // Count oracle results by status
  const resultCounts = manifest.oracle_results.reduce(
    (acc, r) => {
      acc[r.status] = (acc[r.status] || 0) + 1;
      return acc;
    },
    {} as Record<string, number>
  );

  return (
    <div style={styles.container}>
      {/* Breadcrumb */}
      <div style={styles.breadcrumb}>
        <Link to="/evidence" style={styles.breadcrumbLink}>Artifacts</Link>
        <span style={{ color: '#666' }}> / </span>
        <span>{bundle.content_hash.substring(0, 16)}...</span>
      </div>

      {/* Header */}
      <div style={styles.header}>
        <div>
          <h1 style={styles.title}>Artifact Bundle</h1>
          <p style={styles.subtitle}>{bundle.content_hash}</p>
        </div>
        <span
          style={{
            ...styles.statusBadge,
            backgroundColor: verdictStyle.bg,
            color: verdictStyle.color,
            fontSize: '0.875rem',
            padding: '0.5rem 1rem',
          }}
        >
          {manifest.verdict}
        </span>
      </div>

      {/* Overview Card */}
      <div style={styles.card}>
        <h2 style={styles.cardTitle}>Overview</h2>
        <div style={styles.infoRow}>
          <span style={styles.infoLabel}>Artifact Type</span>
          <span style={styles.infoValue}>{manifest.artifact_type}</span>
        </div>
        <div style={styles.infoRow}>
          <span style={styles.infoLabel}>Schema Version</span>
          <span style={styles.infoValue}>{manifest.schema_version}</span>
        </div>
        <div style={styles.infoRow}>
          <span style={styles.infoLabel}>Suite ID</span>
          <code style={styles.monospace}>{manifest.suite_id}</code>
        </div>
        <div style={styles.infoRow}>
          <span style={styles.infoLabel}>Suite Hash</span>
          <code style={styles.monospace}>{manifest.suite_hash}</code>
        </div>
        <div style={styles.infoRow}>
          <span style={styles.infoLabel}>Created</span>
          <span style={styles.infoValue}>
            {new Date(manifest.created_at).toLocaleString()}
          </span>
        </div>
        {manifest.environment_fingerprint && (
          <div style={styles.infoRow}>
            <span style={styles.infoLabel}>Environment</span>
            <code style={styles.monospace}>{manifest.environment_fingerprint}</code>
          </div>
        )}
      </div>

      {/* Attribution Card */}
      <div style={styles.card}>
        <h2 style={styles.cardTitle}>Attribution</h2>
        <div style={styles.infoRow}>
          <span style={styles.infoLabel}>Actor Kind</span>
          <span
            style={{
              ...styles.statusBadge,
              backgroundColor: actorStyle.bg,
              color: actorStyle.color,
            }}
          >
            {manifest.attribution.actor_kind}
          </span>
        </div>
        <div style={styles.infoRow}>
          <span style={styles.infoLabel}>Actor ID</span>
          <span style={styles.infoValue}>{manifest.attribution.actor_id}</span>
        </div>
        {manifest.attribution.run_id && (
          <div style={styles.infoRow}>
            <span style={styles.infoLabel}>Run ID</span>
            <span style={styles.infoValue}>{manifest.attribution.run_id}</span>
          </div>
        )}
        {manifest.attribution.candidate_id && (
          <div style={styles.infoRow}>
            <span style={styles.infoLabel}>Candidate</span>
            <Link
              to={`/candidates/${manifest.attribution.candidate_id}`}
              style={styles.link}
            >
              {manifest.attribution.candidate_id}
            </Link>
          </div>
        )}
      </div>

      {/* Oracle Results Card */}
      <div style={styles.card}>
        <h2 style={styles.cardTitle}>
          <span>Oracle Results ({manifest.oracle_results.length})</span>
          <span style={{ fontSize: '0.75rem', color: '#666', fontWeight: 'normal' }}>
            {Object.entries(resultCounts).map(([status, count]) => (
              <span
                key={status}
                style={{
                  ...styles.statusBadge,
                  backgroundColor: (statusColors[status] || statusColors.SKIPPED).bg,
                  color: (statusColors[status] || statusColors.SKIPPED).color,
                  marginLeft: '0.5rem',
                }}
              >
                {count} {status}
              </span>
            ))}
          </span>
        </h2>
        {manifest.oracle_results.length === 0 ? (
          <div style={styles.placeholder}>
            <p>No oracle results in this bundle.</p>
          </div>
        ) : (
          manifest.oracle_results.map((result, idx) => {
            const resultStyle = statusColors[result.status] || statusColors.SKIPPED;
            return (
              <div key={idx} style={styles.oracleCard}>
                <div style={styles.oracleHeader}>
                  <span style={styles.oracleId}>{result.oracle_id}</span>
                  <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
                    {result.duration_ms !== null && (
                      <span style={{ fontSize: '0.75rem', color: '#666' }}>
                        {result.duration_ms}ms
                      </span>
                    )}
                    <span
                      style={{
                        ...styles.statusBadge,
                        backgroundColor: resultStyle.bg,
                        color: resultStyle.color,
                      }}
                    >
                      {result.status}
                    </span>
                  </div>
                </div>
                {result.message && (
                  <p style={styles.oracleMessage}>{result.message}</p>
                )}
                {result.artifacts.length > 0 && (
                  <div style={{ marginTop: '0.5rem', fontSize: '0.75rem', color: '#666' }}>
                    Artifacts: {result.artifacts.join(', ')}
                  </div>
                )}
              </div>
            );
          })
        )}
      </div>

      {/* Artifacts Card */}
      <div style={styles.card}>
        <h2 style={styles.cardTitle}>Artifacts ({manifest.artifacts.length})</h2>
        {manifest.artifacts.length === 0 ? (
          <div style={styles.placeholder}>
            <p>No artifacts in this bundle.</p>
          </div>
        ) : (
          manifest.artifacts.map((artifact, idx) => (
            <div key={idx} style={styles.artifactRow}>
              <div style={styles.artifactInfo}>
                <div style={styles.artifactName}>{artifact.name}</div>
                <div style={styles.artifactMeta}>
                  {artifact.media_type} &bull; {formatBytes(artifact.size_bytes)}
                </div>
                <div style={{ ...styles.artifactMeta, fontFamily: 'monospace' }}>
                  {artifact.content_hash}
                </div>
              </div>
              <a
                href={`${config.apiUrl}/api/v1/evidence/${bundle.content_hash}/blobs/${artifact.name}`}
                style={styles.downloadButton}
                target="_blank"
                rel="noopener noreferrer"
              >
                Download
              </a>
            </div>
          ))
        )}
      </div>

      {/* Raw Manifest Card */}
      <div style={styles.card}>
        <h2 style={styles.cardTitle}>
          <span>Raw Manifest</span>
          <button
            onClick={() => setShowRawManifest(!showRawManifest)}
            style={{
              ...styles.downloadButton,
              backgroundColor: showRawManifest ? '#666' : '#0066cc',
            }}
          >
            {showRawManifest ? 'Hide' : 'Show'}
          </button>
        </h2>
        {showRawManifest && (
          <pre style={styles.jsonViewer}>
            {JSON.stringify(manifest, null, 2)}
          </pre>
        )}
      </div>
    </div>
  );
}

export default EvidenceDetail;
